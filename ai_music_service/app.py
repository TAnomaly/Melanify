"""
AI Music Generation Service using MusicGen
Flask API that generates music from text prompts
"""

from flask import Flask, request, jsonify, send_file
from flask_cors import CORS
import torch
from transformers import AutoProcessor, MusicgenForConditionalGeneration, BlipProcessor, BlipForConditionalGeneration
from PIL import Image
import scipy.io.wavfile as wavfile
import numpy as np
import os
import uuid
from datetime import datetime
import logging
import io
import base64

app = Flask(__name__)

# Get CORS origins from environment or use defaults
cors_origins = os.getenv('CORS_ORIGINS', 'http://localhost:3000,http://127.0.0.1:3000').split(',')
CORS(app, resources={r"/*": {"origins": cors_origins}},
     supports_credentials=True,
     allow_headers=["Content-Type", "Authorization"],
     methods=["GET", "POST", "PUT", "DELETE", "OPTIONS"])

# Setup logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

# Configuration
OUTPUT_DIR = "generated_music"
os.makedirs(OUTPUT_DIR, exist_ok=True)

# Global model variables (loaded once at startup)
processor = None
model = None
device = None

# BLIP Image Captioning model
blip_processor = None
blip_model = None

def enhance_music_prompt(user_prompt, with_vocals=False):
    """
    Enhance user prompt with more detailed music generation instructions
    """
    # Music quality descriptors
    quality_terms = "high quality, clear sound, professional production"

    # Add genre-specific enhancements
    enhanced = user_prompt.strip()

    # Add instrumentation details if not specified
    if "instrument" not in enhanced.lower():
        enhanced += ", rich instrumentation"

    # Add tempo/energy if not specified
    if not any(word in enhanced.lower() for word in ["fast", "slow", "upbeat", "calm", "energetic", "relaxed"]):
        enhanced += ", dynamic tempo"

    # Add production quality
    enhanced = f"{enhanced}, {quality_terms}"

    # Add vocal specification
    if with_vocals:
        enhanced += ", with expressive vocals and clear melody"
    else:
        enhanced += ", instrumental"

    # Add musicality
    enhanced += ", harmonic progression, well-structured composition"

    logger.info(f"Enhanced prompt: {enhanced}")
    return enhanced

def initialize_model():
    """Initialize MusicGen model - called on startup"""
    global processor, model, device

    try:
        logger.info("Initializing MusicGen model...")

        # Check if CUDA is available
        device = "cuda" if torch.cuda.is_available() else "cpu"
        logger.info(f"Using device: {device}")

        # Load model and processor - using medium for better quality
        model_name = "facebook/musicgen-medium"  # Medium model for better quality
        logger.info(f"Loading model: {model_name}")

        processor = AutoProcessor.from_pretrained(model_name)
        model = MusicgenForConditionalGeneration.from_pretrained(model_name)
        model = model.to(device)

        logger.info("Model initialized successfully!")
        return True
    except Exception as e:
        logger.error(f"Failed to initialize model: {str(e)}")
        return False

def initialize_blip_model():
    """Initialize BLIP image captioning model"""
    global blip_processor, blip_model, device

    try:
        logger.info("Initializing BLIP image captioning model...")

        # Use the same device as MusicGen
        if device is None:
            device = "cuda" if torch.cuda.is_available() else "cpu"

        model_name = "Salesforce/blip-image-captioning-base"
        logger.info(f"Loading BLIP model: {model_name}")

        blip_processor = BlipProcessor.from_pretrained(model_name)
        blip_model = BlipForConditionalGeneration.from_pretrained(model_name)
        blip_model = blip_model.to(device)

        logger.info("BLIP model initialized successfully!")
        return True
    except Exception as e:
        logger.error(f"Failed to initialize BLIP model: {str(e)}")
        return False

@app.route('/health', methods=['GET'])
def health_check():
    """Health check endpoint"""
    return jsonify({
        "status": "ok",
        "model_loaded": model is not None,
        "device": str(device) if device else "unknown"
    })

@app.route('/generate', methods=['POST'])
def generate_music():
    """
    Generate music from text prompt

    Request JSON:
    {
        "prompt": "upbeat electronic dance music",
        "duration": 10,  # seconds (optional, default: 10, max: 30)
        "with_vocals": false,  # include vocals (optional, default: false)
        "return_url": true  # return file path instead of audio data
    }

    Response JSON:
    {
        "success": true,
        "file_id": "uuid",
        "file_path": "/download/uuid",
        "duration": 10
    }
    """
    try:
        if model is None:
            return jsonify({
                "success": False,
                "error": "Model not initialized"
            }), 500

        # Get request data
        data = request.get_json()
        user_prompt = data.get('prompt', '')
        duration = min(data.get('duration', 10), 30)  # Default 10 seconds, max 30
        with_vocals = data.get('with_vocals', False)
        return_url = data.get('return_url', True)

        if not user_prompt:
            return jsonify({
                "success": False,
                "error": "Prompt is required"
            }), 400

        # Enhance the prompt for better quality
        prompt = enhance_music_prompt(user_prompt, with_vocals)

        logger.info(f"Generating music for prompt: '{prompt}' (duration: {duration}s)")

        # Prepare inputs
        inputs = processor(
            text=[prompt],
            padding=True,
            return_tensors="pt",
        )
        inputs = inputs.to(device)

        # Calculate max_new_tokens based on duration
        # MusicGen generates at 50Hz, so 50 tokens per second
        max_new_tokens = int(duration * 50)

        # Generate audio with quality-optimized settings
        logger.info("Generating audio...")
        with torch.no_grad():
            audio_values = model.generate(
                **inputs,
                max_new_tokens=max_new_tokens,
                do_sample=True,
                guidance_scale=3.5,  # Higher guidance for better quality
                temperature=0.9,  # Slightly lower for more coherent output
                top_k=250,  # Add top-k sampling
                top_p=0.95  # Add nucleus sampling
            )

        # Convert to numpy array
        audio_array = audio_values[0].cpu().numpy()

        logger.info(f"Audio array shape: {audio_array.shape}, dtype: {audio_array.dtype}")
        logger.info(f"Audio range: min={audio_array.min()}, max={audio_array.max()}")

        # Save to file
        file_id = str(uuid.uuid4())
        file_path = os.path.join(OUTPUT_DIR, f"{file_id}.wav")

        # MusicGen outputs at 32kHz sample rate
        sample_rate = model.config.audio_encoder.sampling_rate

        # If audio is 2D (channels, samples), transpose to (samples, channels)
        if len(audio_array.shape) == 2:
            audio_array = audio_array.T
            logger.info(f"Transposed audio shape: {audio_array.shape}")

        # Normalize audio safely
        # Ensure the array is not all zeros
        max_val = np.abs(audio_array).max()
        logger.info(f"Max absolute value: {max_val}")
        if max_val > 1e-8:  # Avoid division by very small numbers
            audio_array = audio_array / max_val
        else:
            logger.warning("Audio is nearly silent, using default normalization")
            audio_array = audio_array * 0.1  # Small default amplitude

        # Clamp values between -1 and 1
        audio_array = np.clip(audio_array, -1.0, 1.0)

        # Convert to int16 with proper rounding
        audio_array = np.round(audio_array * 32767.0).astype(np.int16)

        logger.info(f"Final audio shape: {audio_array.shape}, dtype: {audio_array.dtype}")

        wavfile.write(file_path, sample_rate, audio_array)

        logger.info(f"Music generated successfully: {file_path}")

        return jsonify({
            "success": True,
            "file_id": file_id,
            "file_path": f"/download/{file_id}",
            "duration": duration,
            "sample_rate": sample_rate,
            "prompt": prompt,
            "timestamp": datetime.now().isoformat()
        })

    except Exception as e:
        logger.error(f"Error generating music: {str(e)}")
        return jsonify({
            "success": False,
            "error": str(e)
        }), 500

@app.route('/download/<file_id>', methods=['GET'])
def download_music(file_id):
    """Download generated music file"""
    try:
        file_path = os.path.join(OUTPUT_DIR, f"{file_id}.wav")

        if not os.path.exists(file_path):
            return jsonify({
                "success": False,
                "error": "File not found"
            }), 404

        return send_file(
            file_path,
            mimetype='audio/wav',
            as_attachment=True,
            download_name=f"generated_{file_id}.wav"
        )
    except Exception as e:
        logger.error(f"Error downloading file: {str(e)}")
        return jsonify({
            "success": False,
            "error": str(e)
        }), 500

@app.route('/batch-generate', methods=['POST'])
def batch_generate():
    """
    Generate multiple songs from a list of prompts

    Request JSON:
    {
        "prompts": [
            {"title": "Song 1", "prompt": "happy pop music"},
            {"title": "Song 2", "prompt": "sad piano melody"}
        ],
        "duration": 10,
        "with_vocals": false
    }

    Response JSON:
    {
        "success": true,
        "songs": [
            {"title": "Song 1", "file_id": "uuid1", "file_path": "/download/uuid1"},
            {"title": "Song 2", "file_id": "uuid2", "file_path": "/download/uuid2"}
        ]
    }
    """
    try:
        if model is None:
            return jsonify({
                "success": False,
                "error": "Model not initialized"
            }), 500

        data = request.get_json()
        prompts = data.get('prompts', [])
        duration = min(data.get('duration', 10), 30)  # Max 30 seconds
        with_vocals = data.get('with_vocals', False)

        if not prompts:
            return jsonify({
                "success": False,
                "error": "Prompts list is required"
            }), 400

        results = []

        for item in prompts:
            title = item.get('title', 'Untitled')
            user_prompt = item.get('prompt', '')

            if not user_prompt:
                continue

            # Enhance the prompt for better quality
            prompt = enhance_music_prompt(user_prompt, with_vocals)

            logger.info(f"Generating '{title}': {prompt}")

            # Generate music using the same logic as single generation
            inputs = processor(
                text=[prompt],
                padding=True,
                return_tensors="pt",
            )
            inputs = inputs.to(device)

            max_new_tokens = int(duration * 50)

            with torch.no_grad():
                audio_values = model.generate(
                    **inputs,
                    max_new_tokens=max_new_tokens,
                    do_sample=True,
                    guidance_scale=3.5,  # Higher guidance for better quality
                    temperature=0.9,  # Slightly lower for more coherent output
                    top_k=250,  # Add top-k sampling
                    top_p=0.95  # Add nucleus sampling
                )

            audio_array = audio_values[0].cpu().numpy()

            file_id = str(uuid.uuid4())
            file_path = os.path.join(OUTPUT_DIR, f"{file_id}.wav")

            sample_rate = model.config.audio_encoder.sampling_rate

            # If audio is 2D (channels, samples), transpose to (samples, channels)
            if len(audio_array.shape) == 2:
                audio_array = audio_array.T

            # Normalize audio safely
            max_val = np.abs(audio_array).max()
            if max_val > 1e-8:  # Avoid division by very small numbers
                audio_array = audio_array / max_val
            else:
                audio_array = audio_array * 0.1  # Small default amplitude

            # Clamp values between -1 and 1
            audio_array = np.clip(audio_array, -1.0, 1.0)

            # Convert to int16 with proper rounding
            audio_array = np.round(audio_array * 32767.0).astype(np.int16)

            wavfile.write(file_path, sample_rate, audio_array)

            results.append({
                "title": title,
                "file_id": file_id,
                "file_path": f"/download/{file_id}",
                "prompt": prompt
            })

        return jsonify({
            "success": True,
            "songs": results,
            "count": len(results)
        })

    except Exception as e:
        logger.error(f"Error in batch generation: {str(e)}")
        return jsonify({
            "success": False,
            "error": str(e)
        }), 500

@app.route('/analyze-image', methods=['POST'])
def analyze_image():
    """
    Analyze an image and generate a caption describing the mood/scene

    Request JSON:
    {
        "image": "base64_encoded_image_data"
    }

    Response JSON:
    {
        "success": true,
        "caption": "a photo of a sunset over the ocean",
        "suggested_prompt": "Relaxing beach vibes music for sunset moments"
    }
    """
    try:
        if blip_model is None:
            return jsonify({
                "success": False,
                "error": "BLIP model not initialized"
            }), 500

        data = request.get_json()
        image_data = data.get('image', '')

        if not image_data:
            return jsonify({
                "success": False,
                "error": "Image data is required"
            }), 400

        logger.info("Analyzing image...")

        # Decode base64 image
        image_bytes = base64.b64decode(image_data.split(',')[1] if ',' in image_data else image_data)
        image = Image.open(io.BytesIO(image_bytes)).convert('RGB')

        # Generate caption
        inputs = blip_processor(image, return_tensors="pt").to(device)

        with torch.no_grad():
            out = blip_model.generate(**inputs, max_length=50)

        caption = blip_processor.decode(out[0], skip_special_tokens=True)

        logger.info(f"Generated caption: {caption}")

        # Create a music prompt based on the caption
        suggested_prompt = f"Music that captures the mood of: {caption}"

        return jsonify({
            "success": True,
            "caption": caption,
            "suggested_prompt": suggested_prompt
        })

    except Exception as e:
        logger.error(f"Error analyzing image: {str(e)}")
        return jsonify({
            "success": False,
            "error": str(e)
        }), 500

if __name__ == '__main__':
    logger.info("Starting AI Music Service...")

    # Initialize models on startup
    musicgen_ok = initialize_model()
    blip_ok = initialize_blip_model()

    # Get port from environment variable
    port = int(os.getenv('PORT', 5000))

    if musicgen_ok and blip_ok:
        logger.info("All models initialized successfully!")
        logger.info(f"Starting Flask server on http://0.0.0.0:{port}")
        app.run(host='0.0.0.0', port=port, debug=False)
    elif musicgen_ok:
        logger.warning("BLIP model failed to initialize, but MusicGen is ready")
        logger.info(f"Starting Flask server on http://0.0.0.0:{port}")
        app.run(host='0.0.0.0', port=port, debug=False)
    else:
        logger.error("Failed to start service - MusicGen model initialization failed")
