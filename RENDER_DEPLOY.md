# Render Deployment Guide

## 🚀 Deploy Edilen Servisler

### 1. Rust Backend (Ana Uygulama)
URL: https://melanify.onrender.com

### 2. Python AI Music Service (Ayrı Deploy Gerekli)

---

## 📋 Python AI Servisini Deploy Etme

### Web Dashboard İle (Kolay):

1. **Render Dashboard'a Git**: https://dashboard.render.com
2. **"New +" → "Web Service"** seç
3. GitHub reposunu bağla: `TAnomaly/Melanify`
4. Ayarları yap:
   ```
   Name: melanify-ai-service
   Root Directory: ai_music_service
   Environment: Docker
   Dockerfile Path: ./Dockerfile
   Instance Type: Free
   ```
5. **Environment Variables** ekle:
   ```
   PORT=5000
   CORS_ORIGINS=https://melanify.onrender.com,https://relaxed-mooncake-8e5630.netlify.app
   ```
6. **Create Web Service** tıkla

### CLI İle (Hızlı):

```bash
# Render CLI kurulu değilse önce kur
npm install -g render-cli
# veya
brew install render

# Login
render login

# Python servisini deploy et
cd /home/tugmirk/Desktop/Melanify
render deploy --service-type web \
  --name melanify-ai-service \
  --root-dir ai_music_service \
  --env docker \
  --dockerfile-path ./Dockerfile \
  --instance-type free \
  --env-var PORT=5000 \
  --env-var CORS_ORIGINS=https://melanify.onrender.com,https://relaxed-mooncake-8e5630.netlify.app
```

---

## 🔧 Environment Variables

### Rust Backend (melanify.onrender.com):
```
HOST=0.0.0.0
PORT=8081
FRONTEND_URL=https://relaxed-mooncake-8e5630.netlify.app
SPOTIFY_CLIENT_ID=ae95afc24c12492a952e3d586ab8dcca
SPOTIFY_CLIENT_SECRET=0c4fc4b5032b4b4fac846d69073d3d54
SPOTIFY_REDIRECT_URI=https://melanify.onrender.com/callback
GEMINI_API_KEY=AIzaSyDMWZXSVKPIvqRuDwktWYQZ5OqFCZh6J-8
RUST_LOG=info
```

### Python AI Service:
```
PORT=5000
CORS_ORIGINS=https://melanify.onrender.com,https://relaxed-mooncake-8e5630.netlify.app
PYTHONUNBUFFERED=1
```

### Netlify Frontend:
```
REACT_APP_API_URL=https://melanify.onrender.com
```

---

## ⚠️ Önemli Notlar

1. **Free Tier Soğuk Başlatma**: İlk istekte 50 saniye sürebilir
2. **Python Servisi Memory**: AI modelleri 512MB+ RAM gerektirir
3. **Rust Build Süresi**: İlk deploy 10-15 dakika sürebilir
4. **CORS**: Frontend URL'i tüm servislerde tanımlı olmalı

---

## 🔍 Hata Kontrolü

### Rust Backend Logları:
```bash
render logs --service melanify
```

### Python Service Logları:
```bash
render logs --service melanify-ai-service
```

### Health Check:
- Rust: https://melanify.onrender.com/health (veya /)
- Python: https://melanify-ai-service.onrender.com/health
