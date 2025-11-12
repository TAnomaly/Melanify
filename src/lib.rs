pub mod handlers;
pub mod models;
pub mod services;

use actix_web::web;
use services::gemini_service::GeminiService;
use services::musicgen_service::MusicGenService;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub pending_tracks: Arc<Mutex<HashMap<String, models::playlist::CreatePlaylistRequest>>>,
    pub gemini_service: GeminiService,
    pub musicgen_service: MusicGenService,
    pub auth_states: Arc<Mutex<HashMap<String, String>>>,
}

pub fn configure_app(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .route(
                "/process-prompt",
                web::post().to(handlers::process_gemini_prompt),
            )
            .route("/", web::get().to(handlers::index))
            .route("/callback", web::get().to(handlers::spotify_callback))
            .route(
                "/history-auth",
                web::get().to(handlers::get_history_auth_url),
            )
            .route(
                "/create-spotify-playlist",
                web::post().to(handlers::create_spotify_playlist_handler),
            )
            // AI Music Generation routes
            .route(
                "/generate-ai-music",
                web::post().to(handlers::generate_ai_music),
            )
            .route(
                "/generate-ai-music-batch",
                web::post().to(handlers::generate_ai_music_batch),
            )
            .route(
                "/ai-music-health",
                web::get().to(handlers::ai_music_health_check),
            ),
    );
}
