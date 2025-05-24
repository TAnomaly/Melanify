use actix_cors::Cors;
use actix_session::config::PersistentSession;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::{time::Duration, Key};
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use spotify_ai_playlist::services::gemini_service::GeminiService;
use spotify_ai_playlist::{configure_app, AppState, GEMINI_API_KEY};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Update Spotify credentials
const SPOTIFY_CLIENT_ID: &str = "ae95afc24c12492a952e3d586ab8dcca";
const SPOTIFY_CLIENT_SECRET: &str = "0c4fc4b5032b4b4fac846d69073d3d54";
const SPOTIFY_REDIRECT_URI: &str = "http://127.0.0.1:8081/callback";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    println!("Server starting at http://127.0.0.1:8081");

    // Generate a random secret key for session encryption
    let secret_key = Key::generate();

    let app_state = AppState {
        pending_tracks: Arc::new(Mutex::new(HashMap::new())),
        gemini_service: GeminiService::new(GEMINI_API_KEY.to_string()),
        auth_states: Arc::new(Mutex::new(HashMap::new())),
    };

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:8081")
            .allowed_origin("http://localhost:3000")
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
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
