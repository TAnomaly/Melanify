use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Track {
    pub name: String,
    pub artist: String,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spotify_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreatePlaylistRequest {
    pub tracks: Vec<Track>,
    pub playlist_name: String,
    pub playlist_description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecentTrack {
    pub name: String,
    pub artist: String,
    pub album_image: Option<String>,
    pub played_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiTrack {
    pub title: String,
    pub artist: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiPromptResponse {
    pub tracks: Vec<GeminiTrack>,
    pub playlist_name: String,
    pub playlist_description: String,
}

#[derive(Debug, Deserialize)]
pub struct CallbackQuery {
    pub code: String,
    pub state: Option<String>,
    pub for_history: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct GeminiPromptRequest {
    pub prompt: String,
}

#[derive(Debug, Deserialize)]
pub struct LastFmResponse {
    #[serde(default)]
    pub tracks: LastFmTracks,
}

#[derive(Debug, Deserialize, Default)]
pub struct LastFmTracks {
    #[serde(default)]
    pub track: Vec<LastFmTrack>,
}

#[derive(Debug, Deserialize)]
pub struct LastFmTrack {
    pub name: String,
    pub artist: LastFmArtist,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct LastFmArtist {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecommendationRequest {
    pub seed_tracks: Vec<String>,
    pub limit: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiRequest {
    pub contents: Vec<GeminiContent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiContent {
    pub parts: Vec<GeminiPart>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiPart {
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct GeminiResponse {
    pub candidates: Vec<GeminiCandidate>,
}

#[derive(Debug, Deserialize)]
pub struct GeminiCandidate {
    pub content: GeminiContent,
}

#[derive(Debug, Serialize)]
pub struct RecentlyPlayedResponse {
    pub tracks: Vec<RecentTrack>,
}

#[derive(Debug, Deserialize)]
pub struct CallbackForm {
    pub tracks: String,
}

// AI Music Generation Models
#[derive(Debug, Serialize, Deserialize)]
pub struct AiMusicRequest {
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AiMusicBatchRequest {
    pub prompts: Vec<AiMusicPrompt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AiMusicPrompt {
    pub title: String,
    pub prompt: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AiMusicResponse {
    pub success: bool,
    pub file_id: String,
    pub file_path: String,
    pub duration: u32,
    pub sample_rate: u32,
    pub prompt: String,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AiMusicBatchResponse {
    pub success: bool,
    pub songs: Vec<AiSong>,
    pub count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AiSong {
    pub title: String,
    pub file_id: String,
    pub file_path: String,
    pub prompt: String,
}
