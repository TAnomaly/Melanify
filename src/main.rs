use actix_cors::Cors;
use actix_session::config::PersistentSession;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::{time::Duration, Key};
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use spotify_ai_playlist::services::gemini_service::GeminiService;
use spotify_ai_playlist::services::musicgen_service::MusicGenService;
use spotify_ai_playlist::{configure_app, AppState};
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    // Get environment variables
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8081".to_string());
    let gemini_api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set");
    let frontend_url = env::var("FRONTEND_URL").unwrap_or_else(|_| format!("http://{}:{}", host, port));

    println!("Server starting at http://{}:{}", host, port);

    // Generate a random secret key for session encryption
    let secret_key = Key::generate();

    let app_state = AppState {
        pending_tracks: Arc::new(Mutex::new(HashMap::new())),
        gemini_service: GeminiService::new(gemini_api_key.clone()),
        musicgen_service: MusicGenService::new(),
        auth_states: Arc::new(Mutex::new(HashMap::new())),
    };

    let bind_addr = format!("{}:{}", host, port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&frontend_url)
            .allowed_origin("http://localhost:3000")
            .allowed_origin("https://relaxed-mooncake-8e5630.netlify.app")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                "Content-Type",
                "Authorization",
                "Cookie",
                "X-CSRF-Token",
                "Accept",
                "Origin",
            ])
            .expose_headers(vec!["Set-Cookie"])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_secure(false)
                    .cookie_http_only(false)
                    .cookie_name("spotify_ai_session".to_string())
                    .cookie_path("/".to_string())
                    .cookie_domain(None)
                    .cookie_same_site(actix_web::cookie::SameSite::None)
                    .session_lifecycle(
                        PersistentSession::default().session_ttl(Duration::hours(24)),
                    )
                    .build(),
            )
            .wrap(cors)
            .app_data(web::Data::new(app_state.clone()))
            .wrap(Logger::default())
            .configure(configure_app)
    })
    .bind(&bind_addr)?
    .run()
    .await
}
