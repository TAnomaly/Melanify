# Melanify - Dual Mode Setup Guide

## Overview

Melanify now supports **two modes** of playlist creation:

1. **Spotify Mode**: Creates playlists with real songs from Spotify (using Gemini AI)
2. **AI Music Mode**: Generates completely original AI music using Facebook's MusicGen

## Quick Start Guide

### Prerequisites

- **Rust** (latest stable version)
- **Node.js** (v16+) and npm
- **Python** (3.8+) for AI music generation
- **Spotify Account** (for Spotify mode)

---

## Setup Instructions

### Step 1: Install Rust Backend Dependencies

```bash
cd /home/tugmirk/Desktop/Melanify
cargo build
```

### Step 2: Install Frontend Dependencies

```bash
cd src  # Navigate to frontend source
npm install
# or if you have a separate frontend directory
cd frontend && npm install
```

### Step 3: Set Up AI Music Service (Python)

```bash
cd ai_music_service

# Create virtual environment
python3 -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate

# Install dependencies
pip install -r requirements.txt
```

**Note**: First run will download the MusicGen model (~300MB). This may take a few minutes.

---

## Running the Application

### Terminal 1: Start the AI Music Service (for AI Music mode)

```bash
cd ai_music_service
source venv/bin/activate
python app.py
```

You should see:
```
Starting AI Music Service...
Loading model: facebook/musicgen-small
Model initialized successfully!
Starting Flask server on http://localhost:5000
```

**Test the service:**
```bash
curl http://localhost:5000/health
```

### Terminal 2: Start the Rust Backend

```bash
cd /home/tugmirk/Desktop/Melanify
cargo run
```

You should see:
```
Server starting at http://127.0.0.1:8081
```

### Terminal 3: Start the Frontend (if separate)

```bash
cd frontend  # or wherever your React app is
npm start
```

The app should open at `http://localhost:3000`

---

## Using the Application

### Mode 1: Spotify Playlists (Real Songs)

1. Open the app at `http://localhost:3000`
2. Select **"Spotify Şarkıları"** mode (left button)
3. Enter a prompt like:
   - "90'ların en iyi rock şarkıları"
   - "Sakin akustik müzikler"
   - "Workout için enerjik şarkılar"
4. Click **"Çalma Listesi Oluştur"**
5. Review the generated playlist
6. Click **"Spotify'a Aktar"** to save to your Spotify account
7. Authorize with Spotify when prompted

### Mode 2: AI Generated Music (Original Songs)

1. Open the app at `http://localhost:3000`
2. Select **"AI Müzik Üret"** mode (right button)
3. Enter a prompt like:
   - "Upbeat electronic dance music"
   - "Sad piano melody"
   - "Epic orchestral soundtrack"
4. Click **"AI Müzik Üret"**
5. Wait for generation (takes 30-60 seconds per song)
6. Play the generated music directly in the browser
7. Download individual tracks as WAV files

**Note**: AI Music generation requires the Python service to be running!

---

## API Endpoints

### Rust Backend (Port 8081)

#### Spotify Mode:
- `POST /process-prompt` - Generate playlist with Gemini AI
- `POST /create-spotify-playlist` - Create playlist in Spotify
- `GET /callback` - Spotify OAuth callback

#### AI Music Mode:
- `GET /ai-music-health` - Check if Python service is running
- `POST /generate-ai-music` - Generate single AI song
- `POST /generate-ai-music-batch` - Generate multiple AI songs

### Python Service (Port 5000)

- `GET /health` - Health check
- `POST /generate` - Generate single song
- `POST /batch-generate` - Generate multiple songs
- `GET /download/{file_id}` - Download generated music

---

## Testing

### Test Spotify Mode

```bash
# Test Gemini prompt processing
curl -X POST http://127.0.0.1:8081/process-prompt \
  -H "Content-Type: application/json" \
  -d '{"prompt": "happy pop songs"}'
```

### Test AI Music Mode

```bash
# Check Python service
curl http://localhost:5000/health

# Generate AI music
curl -X POST http://localhost:5000/generate \
  -H "Content-Type: application/json" \
  -d '{"prompt": "upbeat electronic music", "duration": 5}'
```

---

## Troubleshooting

### Python Service Won't Start

**Error**: `Module not found` or import errors

**Solution**:
```bash
cd ai_music_service
source venv/bin/activate
pip install --upgrade pip
pip install -r requirements.txt
```

### "AI Music service is not available"

**Problem**: The Python service isn't running

**Solution**:
1. Make sure Python service is running on port 5000
2. Check with: `curl http://localhost:5000/health`
3. Look at Python terminal for errors

### Out of Memory (AI Music Generation)

**Problem**: System crashes during music generation

**Solution**:
- Edit `ai_music_service/app.py`
- Change model from `musicgen-small` to `musicgen-tiny` (lighter)
- Or reduce duration in requests

### Model Download Fails

**Problem**: Cannot download MusicGen model

**Solution**:
```bash
# Manually download model
python -c "from transformers import MusicgenForConditionalGeneration; MusicgenForConditionalGeneration.from_pretrained('facebook/musicgen-small')"
```

### CORS Errors in Frontend

**Solution**: Make sure you're accessing the app from `http://localhost:3000` or `http://127.0.0.1:8081`

---

## Project Structure

```
Melanify/
├── src/                          # Rust backend source
│   ├── main.rs                   # Server entry point
│   ├── lib.rs                    # App configuration
│   ├── handlers/                 # HTTP request handlers
│   │   └── mod.rs                # All handlers including AI music
│   ├── services/                 # Business logic
│   │   ├── gemini_service.rs     # Gemini AI integration
│   │   └── musicgen_service.rs   # MusicGen integration (NEW)
│   └── models/                   # Data structures
│       └── playlist.rs           # Models including AI music (UPDATED)
│
├── ai_music_service/             # Python AI service (NEW)
│   ├── app.py                    # Flask server
│   ├── requirements.txt          # Python dependencies
│   ├── README.md                 # Service documentation
│   └── generated_music/          # Output directory
│
├── frontend/src/                 # React frontend
│   ├── components/
│   │   └── Home/
│   │       ├── HomePage.tsx      # Main page with mode selector (UPDATED)
│   │       └── HomePage.css      # Styling (UPDATED)
│   └── services/
│       └── api.ts                # API client
│
├── Cargo.toml                    # Rust dependencies
└── SETUP.md                      # This file
```

---

## Performance Tips

### AI Music Generation
- **Small model** (~300MB): Fast, decent quality
- **Medium model** (~1.5GB): Balanced, better quality
- **Large model** (~3.3GB): Slow, best quality

To change model, edit `ai_music_service/app.py`:
```python
model_name = "facebook/musicgen-medium"  # or musicgen-large
```

### GPU Acceleration
If you have NVIDIA GPU with CUDA:
```bash
pip install torch torchvision torchaudio --index-url https://download.pytorch.org/whl/cu118
```

This speeds up music generation significantly!

---

## Additional Features to Implement

- [ ] Save AI-generated songs to user's library
- [ ] Combine AI music with Spotify playlists
- [ ] Custom duration per song
- [ ] Different music styles/genres
- [ ] Export playlists as ZIP
- [ ] Share generated music with friends

---

## License & Credits

- **MusicGen**: Meta/Facebook (License: CC-BY-NC 4.0)
- **Gemini AI**: Google
- **Spotify API**: Spotify AB

## Support

If you encounter issues:
1. Check all services are running (Python, Rust, Frontend)
2. Look at terminal logs for error messages
3. Test each service independently with curl
4. Make sure ports 5000, 8081, and 3000 are free

Happy music creating! 🎵🤖
