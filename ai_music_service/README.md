# AI Music Generation Service

Flask API service for generating AI music using Facebook's MusicGen model.

## Setup

1. **Create virtual environment:**
```bash
python3 -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
```

2. **Install dependencies:**
```bash
pip install -r requirements.txt
```

3. **Run the service:**
```bash
python app.py
```

The service will start on `http://localhost:5000`

## API Endpoints

### Health Check
```bash
GET /health
```

### Generate Single Song
```bash
POST /generate
Content-Type: application/json

{
  "prompt": "upbeat electronic dance music",
  "duration": 10
}
```

### Generate Multiple Songs (Batch)
```bash
POST /batch-generate
Content-Type: application/json

{
  "prompts": [
    {"title": "Song 1", "prompt": "happy pop music"},
    {"title": "Song 2", "prompt": "sad piano melody"}
  ],
  "duration": 10
}
```

### Download Generated Music
```bash
GET /download/{file_id}
```

## Notes

- First run will download the MusicGen model (~1.5GB for small version)
- GPU is recommended but not required
- Generated files are saved in `generated_music/` directory
- Default model: `facebook/musicgen-small` (faster, lower quality)
- For better quality: Change to `facebook/musicgen-medium` or `facebook/musicgen-large`

## Model Options

- `facebook/musicgen-small` - ~300MB, fast
- `facebook/musicgen-medium` - ~1.5GB, balanced
- `facebook/musicgen-large` - ~3.3GB, best quality
- `facebook/musicgen-stereo-*` - Stereo versions
