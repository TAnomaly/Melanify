pub mod statistics;
pub mod success;

use crate::models::playlist::*;
use crate::AppState;
use actix_session::Session;
use actix_web::{web, Error, HttpResponse};
use rspotify::{
    model::{PlayableId, SearchResult, SearchType},
    prelude::*,
    scopes, AuthCodeSpotify, Credentials, OAuth,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const SPOTIFY_CLIENT_ID: &str = "ae95afc24c12492a952e3d586ab8dcca";
const SPOTIFY_CLIENT_SECRET: &str = "0c4fc4b5032b4b4fac846d69073d3d54";
const SPOTIFY_REDIRECT_URI: &str = "http://127.0.0.1:8081/callback";

const SUCCESS_HTML: &str = r#"<!DOCTYPE html>
<html>
<head>
    <title>Success!</title>
    <meta charset="UTF-8">
    <link href="https://fonts.googleapis.com/css2?family=Roboto:wght@300;400;500;700&display=swap" rel="stylesheet">
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        body {
            font-family: 'Roboto', sans-serif;
            background: linear-gradient(135deg, #1DB954 0%, #191414 100%);
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
            color: white;
        }
        .container {
            background: rgba(255, 255, 255, 0.1);
            backdrop-filter: blur(10px);
            padding: 40px;
            border-radius: 16px;
            text-align: center;
            box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
            max-width: 600px;
            width: 90%;
            animation: fadeIn 0.5s ease-out;
        }
        @keyframes fadeIn {
            from { opacity: 0; transform: translateY(20px); }
            to { opacity: 1; transform: translateY(0); }
        }
        h1 {
            font-size: 2.5em;
            margin-bottom: 20px;
            color: #1DB954;
        }
        p {
            font-size: 1.2em;
            margin-bottom: 30px;
            line-height: 1.6;
        }
        .button {
            display: inline-block;
            padding: 12px 24px;
            background: #1DB954;
            color: white;
            text-decoration: none;
            border-radius: 50px;
            font-weight: 500;
            margin: 10px;
            transition: all 0.3s ease;
            border: none;
            cursor: pointer;
        }
        .button:hover {
            transform: translateY(-2px);
            box-shadow: 0 5px 15px rgba(29, 185, 84, 0.3);
            background: #1ed760;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>✨ Success!</h1>
        <p>Your new playlist has been created in your Spotify account.</p>
        <a href="{}" class="button" target="_blank">Open in Spotify</a>
        <a href="/" class="button">Create Another Playlist</a>
    </div>
</body>
</html>"#;

const ERROR_HTML: &str = r#"<!DOCTYPE html>
<html>
<head>
    <title>Error</title>
    <link href="https://fonts.googleapis.com/css2?family=Roboto:wght@300;400;500;700&display=swap" rel="stylesheet">
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        body {
            font-family: 'Roboto', sans-serif;
            background: linear-gradient(135deg, #ff4b4b 0%, #191414 100%);
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
            color: white;
        }
        .container {
            background: rgba(255, 255, 255, 0.1);
            backdrop-filter: blur(10px);
            padding: 40px;
            border-radius: 16px;
            text-align: center;
            box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
            max-width: 600px;
            width: 90%;
            animation: fadeIn 0.5s ease-out;
        }
        @keyframes fadeIn {
            from { opacity: 0; transform: translateY(20px); }
            to { opacity: 1; transform: translateY(0); }
        }
        h1 {
            font-size: 2.5em;
            margin-bottom: 20px;
            color: #ff4b4b;
        }
        p {
            font-size: 1.2em;
            margin-bottom: 30px;
            line-height: 1.6;
        }
        .button {
            display: inline-block;
            padding: 12px 24px;
            background: #ff4b4b;
            color: white;
            text-decoration: none;
            border-radius: 50px;
            font-weight: 500;
            transition: all 0.3s ease;
        }
        .button:hover {
            transform: translateY(-2px);
            box-shadow: 0 5px 15px rgba(255, 75, 75, 0.3);
            background: #ff6b6b;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>❌ Error</h1>
        <p>{}</p>
        <p>{}</p>
        <a href="/" class="button">Try Again</a>
    </div>
</body>
</html>"#;

pub async fn index() -> impl actix_web::Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../templates/index.html"))
}

pub async fn recommend_songs(_req: web::Json<RecommendationRequest>) -> impl actix_web::Responder {
    // ... existing recommend_songs implementation ...
    HttpResponse::Ok().json(serde_json::json!({"message": "Not implemented"}))
}

pub async fn spotify_callback(
    data: web::Data<AppState>,
    query: web::Query<CallbackQuery>,
    session: Session,
) -> Result<HttpResponse, Error> {
    println!("Received callback query: {:?}", query);

    if query.for_history.unwrap_or(false) {
        return handle_history_callback(&query.code).await;
    }

    // Try to get session ID from state parameter first
    let session_id = if let Some(state) = &query.state {
        println!("Found state parameter: {}", state);
        state.clone()
    } else {
        match session.get::<String>("tracks_session_id")? {
            Some(id) => {
                println!("Found session ID in cookie: {}", id);
                id
            }
            None => {
                eprintln!("No session ID found in cookie or state");
                return Ok(HttpResponse::Ok().content_type("text/html").body(format!(
                    r#"
                    <!DOCTYPE html>
                    <html>
                    <head>
                        <title>Error</title>
                        <script>
                            window.opener.postMessage({{
                                type: "PLAYLIST_ERROR",
                                error: "Session expired. Please try again."
                            }}, "*");
                            window.close();
                        </script>
                    </head>
                    <body>
                        <p>Session expired...</p>
                    </body>
                    </html>
                    "#
                )));
            }
        }
    };

    // Get playlist request from application state using session ID
    let playlist_request = {
        let pending_tracks = data.pending_tracks.lock().unwrap();
        match pending_tracks.get(&session_id) {
            Some(request) => {
                println!(
                    "Found tracks in pending_tracks for session ID: {}",
                    session_id
                );
                Some(request.clone())
            }
            None => {
                eprintln!(
                    "No tracks found in pending_tracks for session ID: {}",
                    session_id
                );
                return Ok(HttpResponse::Ok().content_type("text/html").body(format!(
                    r#"
                    <!DOCTYPE html>
                    <html>
                    <head>
                        <title>Error</title>
                        <script>
                            window.opener.postMessage({{
                                type: "PLAYLIST_ERROR",
                                error: "No playlist data found. Please try again."
                            }}, "*");
                            window.close();
                        </script>
                    </head>
                    <body>
                        <p>No playlist data found...</p>
                    </body>
                    </html>
                    "#
                )));
            }
        }
    };

    match playlist_request {
        Some(request) if !request.tracks.is_empty() => {
            println!(
                "Processing playlist request with {} tracks",
                request.tracks.len()
            );

            let creds = Credentials::new(SPOTIFY_CLIENT_ID, SPOTIFY_CLIENT_SECRET);
            let oauth = OAuth {
                redirect_uri: SPOTIFY_REDIRECT_URI.to_string(),
                scopes: scopes!(
                    "playlist-modify-public",
                    "playlist-modify-private",
                    "user-read-private",
                    "user-read-email"
                ),
                state: session_id.clone(),
                ..Default::default()
            };

            let spotify = AuthCodeSpotify::new(creds, oauth);

            match spotify.request_token(&query.code).await {
                Ok(()) => {
                    // Clean up session data after successful token exchange
                    if let Some(session_id) = session.get::<String>("tracks_session_id")? {
                        let mut pending_tracks = data.pending_tracks.lock().unwrap();
                        pending_tracks.remove(&session_id);
                        println!("Cleaned up session data for ID: {}", session_id);
                    }
                    session.remove("tracks_session_id");

                    // Get user profile and create playlist
                    match spotify.me().await {
                        Ok(user) => {
                            match spotify
                                .user_playlist_create(
                                    user.id,
                                    &request.playlist_name,
                                    Some(false), // not public
                                    Some(false), // not collaborative
                                    request.playlist_description.as_deref(),
                                )
                                .await
                            {
                                Ok(playlist) => {
                                    let mut spotify_track_ids = Vec::new();

                                    // Search for each track
                                    for track in request.tracks {
                                        println!(
                                            "Searching for track: {} by {}",
                                            track.name, track.artist
                                        );
                                        if let Ok(SearchResult::Tracks(page)) = spotify
                                            .search(
                                                &format!(
                                                    "track:{} artist:{}",
                                                    track.name, track.artist
                                                ),
                                                SearchType::Track,
                                                None,
                                                None,
                                                Some(1),
                                                None,
                                            )
                                            .await
                                        {
                                            if let Some(found_track) = page.items.first() {
                                                if let Some(track_id) = &found_track.id {
                                                    spotify_track_ids.push(track_id.clone());
                                                    println!("Found track: {}", found_track.name);
                                                }
                                            }
                                        }
                                    }

                                    if spotify_track_ids.is_empty() {
                                        return Ok(HttpResponse::Ok().content_type("text/html").body(format!(
                                            r#"
                                            <!DOCTYPE html>
                                            <html>
                                            <head>
                                                <title>Error</title>
                                                <script>
                                                    window.opener.postMessage({{
                                                        type: "PLAYLIST_ERROR",
                                                        error: "Could not find any matching tracks on Spotify"
                                                    }}, "*");
                                                    window.close();
                                                </script>
                                            </head>
                                            <body>
                                                <p>Redirecting back...</p>
                                            </body>
                                            </html>
                                            "#
                                        )));
                                    }

                                    // Add tracks in batches
                                    for chunk in spotify_track_ids.chunks(100) {
                                        let track_ids: Vec<PlayableId> = chunk
                                            .iter()
                                            .map(|id| PlayableId::Track(id.clone()))
                                            .collect();

                                        if let Err(e) = spotify
                                            .playlist_add_items(
                                                playlist.id.clone(),
                                                track_ids,
                                                None,
                                            )
                                            .await
                                        {
                                            eprintln!("Error adding tracks to playlist: {}", e);
                                            return Ok(HttpResponse::Ok().content_type("text/html").body(format!(
                                                r#"
                                                <!DOCTYPE html>
                                                <html>
                                                <head>
                                                    <title>Error</title>
                                                    <script>
                                                        window.opener.postMessage({{
                                                            type: "PLAYLIST_ERROR",
                                                            error: "Failed to add tracks to playlist: {}"
                                                        }}, "*");
                                                        window.close();
                                                    </script>
                                                </head>
                                                <body>
                                                    <p>Redirecting back...</p>
                                                </body>
                                                </html>
                                                "#,
                                                e
                                            )));
                                        }
                                    }

                                    let playlist_url = playlist
                                        .external_urls
                                        .get("spotify")
                                        .map(String::as_str)
                                        .unwrap_or("");

                                    println!("Successfully created playlist: {}", playlist_url);
                                    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(format!(
                                        r#"
                                        <!DOCTYPE html>
                                        <html>
                                        <head>
                                            <title>Playlist Oluşturuldu!</title>
                                            <meta charset="UTF-8">
                                            <link href="https://fonts.googleapis.com/css2?family=Roboto:wght@300;400;500;700&display=swap" rel="stylesheet">
                                            <style>
                                                * {{
                                                    margin: 0;
                                                    padding: 0;
                                                    box-sizing: border-box;
                                                }}
                                                body {{
                                                    font-family: 'Roboto', sans-serif;
                                                    background: linear-gradient(135deg, #1DB954 0%, #191414 100%);
                                                    min-height: 100vh;
                                                    display: flex;
                                                    align-items: center;
                                                    justify-content: center;
                                                    color: white;
                                                }}
                                                .container {{
                                                    background: rgba(255, 255, 255, 0.1);
                                                    backdrop-filter: blur(10px);
                                                    padding: 40px;
                                                    border-radius: 16px;
                                                    text-align: center;
                                                    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
                                                    max-width: 600px;
                                                    width: 90%;
                                                    animation: fadeIn 0.5s ease-out;
                                                }}
                                                @keyframes fadeIn {{
                                                    from {{ opacity: 0; transform: translateY(20px); }}
                                                    to {{ opacity: 1; transform: translateY(0); }}
                                                }}
                                                .success-icon {{
                                                    font-size: 64px;
                                                    margin-bottom: 20px;
                                                    animation: bounce 1s ease infinite;
                                                }}
                                                @keyframes bounce {{
                                                    0%, 100% {{ transform: translateY(0); }}
                                                    50% {{ transform: translateY(-10px); }}
                                                }}
                                                h1 {{
                                                    font-size: 2.5em;
                                                    margin-bottom: 20px;
                                                    color: #1DB954;
                                                }}
                                                p {{
                                                    font-size: 1.2em;
                                                    margin-bottom: 30px;
                                                    line-height: 1.6;
                                                    opacity: 0.9;
                                                }}
                                                .buttons {{
                                                    display: flex;
                                                    gap: 15px;
                                                    justify-content: center;
                                                    flex-wrap: wrap;
                                                }}
                                                .button {{
                                                    display: inline-flex;
                                                    align-items: center;
                                                    padding: 12px 24px;
                                                    background: #1DB954;
                                                    color: white;
                                                    text-decoration: none;
                                                    border-radius: 50px;
                                                    font-weight: 500;
                                                    transition: all 0.3s ease;
                                                    border: none;
                                                    cursor: pointer;
                                                    font-size: 1.1em;
                                                }}
                                                .button:hover {{
                                                    transform: translateY(-2px);
                                                    box-shadow: 0 5px 15px rgba(29, 185, 84, 0.3);
                                                    background: #1ed760;
                                                }}
                                                .button.secondary {{
                                                    background: rgba(255, 255, 255, 0.1);
                                                }}
                                                .button.secondary:hover {{
                                                    background: rgba(255, 255, 255, 0.2);
                                                }}
                                                .button img {{
                                                    width: 24px;
                                                    height: 24px;
                                                    margin-right: 8px;
                                                }}
                                            </style>
                                        </head>
                                        <body>
                                            <div class="container">
                                                <div class="success-icon">🎵</div>
                                                <h1>Playlist Oluşturuldu!</h1>
                                                <p>Harika! Yeni playlist'iniz başarıyla Spotify hesabınıza eklendi.</p>
                                                <div class="buttons">
                                                    <a href="{}" class="button" target="_blank">
                                                        <img src="https://storage.googleapis.com/pr-newsroom-wp/1/2018/11/Spotify_Logo_RGB_White.png" alt="Spotify">
                                                        Spotify'da Aç
                                                    </a>
                                                    <a href="/" class="button secondary" onclick="redirectToHome(event)">
                                                        Yeni Playlist Oluştur
                                                    </a>
                                                </div>
                                            </div>
                                            <script>
                                                function redirectToHome(event) {{
                                                    event.preventDefault();
                                                    if (window.opener) {{
                                                        window.opener.location.href = '/';
                                                        window.close();
                                                    }} else {{
                                                        window.location.href = '/';
                                                    }}
                                                }}

                                                window.opener.postMessage({{
                                                    type: "PLAYLIST_CREATED",
                                                    playlistUrl: "{}"
                                                }}, "*");
                                            </script>
                                        </body>
                                        </html>
                                        "#,
                                        playlist_url,
                                        playlist_url
                                    )))
                                }
                                Err(e) => {
                                    eprintln!("Error creating playlist: {}", e);
                                    Ok(HttpResponse::Ok().content_type("text/html").body(format!(
                                        r#"
                                        <!DOCTYPE html>
                                        <html>
                                        <head>
                                            <title>Error</title>
                                            <script>
                                                window.opener.postMessage({{
                                                    type: "PLAYLIST_ERROR",
                                                    error: "Failed to create playlist: {}"
                                                }}, "*");
                                                window.close();
                                            </script>
                                        </head>
                                        <body>
                                            <p>Redirecting back...</p>
                                        </body>
                                        </html>
                                        "#,
                                        e
                                    )))
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Error getting user profile: {}", e);
                            Ok(HttpResponse::Ok().content_type("text/html").body(format!(
                                r#"
                                <!DOCTYPE html>
                                <html>
                                <head>
                                    <title>Error</title>
                                    <script>
                                        window.opener.postMessage({{
                                            type: "PLAYLIST_ERROR",
                                            error: "Failed to get user profile: {}"
                                        }}, "*");
                                        window.close();
                                    </script>
                                </head>
                                <body>
                                    <p>Redirecting back...</p>
                                </body>
                                </html>
                                "#,
                                e
                            )))
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error exchanging code for token: {}", e);
                    Ok(HttpResponse::Ok().content_type("text/html").body(format!(
                        r#"
                        <!DOCTYPE html>
                        <html>
                        <head>
                            <title>Error</title>
                            <script>
                                window.opener.postMessage({{
                                    type: "PLAYLIST_ERROR",
                                    error: "Failed to authenticate with Spotify: {}"
                                }}, "*");
                                window.close();
                            </script>
                        </head>
                        <body>
                            <p>Redirecting back...</p>
                        </body>
                        </html>
                        "#,
                        e
                    )))
                }
            }
        }
        Some(_) => {
            eprintln!("Empty tracks list received");
            Ok(HttpResponse::Ok().content_type("text/html").body(format!(
                r#"
                <!DOCTYPE html>
                <html>
                <head>
                    <title>Error</title>
                    <script>
                        window.opener.postMessage({{
                            type: "PLAYLIST_ERROR",
                            error: "No tracks selected. Please select some songs before creating a playlist."
                        }}, "*");
                        window.close();
                    </script>
                </head>
                <body>
                    <p>Redirecting back...</p>
                </body>
                </html>
                "#
            )))
        }
        None => {
            eprintln!("No tracks found in session");
            Ok(HttpResponse::Ok().content_type("text/html").body(format!(
                r#"
                <!DOCTYPE html>
                <html>
                <head>
                    <title>Error</title>
                    <script>
                        window.opener.postMessage({{
                            type: "PLAYLIST_ERROR",
                            error: "Please go back and select songs before creating a playlist"
                        }}, "*");
                        window.close();
                    </script>
                </head>
                <body>
                    <p>Redirecting back...</p>
                </body>
                </html>
                "#
            )))
        }
    }
}

pub async fn handle_history_callback(code: &str) -> Result<HttpResponse, Error> {
    let creds = Credentials::new(
        "ae95afc24c12492a952e3d586ab8dcca",
        "0c4fc4b5032b4b4fac846d69073d3d54",
    );
    let oauth = OAuth {
        redirect_uri: "http://127.0.0.1:8081/callback".to_string(),
        scopes: scopes!(
            "user-read-private",
            "user-read-email",
            "user-read-recently-played"
        ),
        ..Default::default()
    };

    let mut spotify = AuthCodeSpotify::new(creds, oauth);

    // Exchange the code for an access token
    match spotify.request_token(code).await {
        Ok(()) => match spotify.current_user_recently_played(Some(20), None).await {
            Ok(history) => {
                let tracks: Vec<RecentTrack> = history
                    .items
                    .into_iter()
                    .map(|item| {
                        let artists = item
                            .track
                            .artists
                            .iter()
                            .map(|a| a.name.clone())
                            .collect::<Vec<String>>()
                            .join(", ");

                        let album_image =
                            item.track.album.images.first().map(|img| img.url.clone());

                        RecentTrack {
                            name: item.track.name,
                            artist: artists,
                            album_image,
                            played_at: item.played_at.to_string(),
                        }
                    })
                    .collect();

                let tracks_json =
                    serde_json::to_string(&tracks).unwrap_or_else(|_| "[]".to_string());

                Ok(HttpResponse::Ok().content_type("text/html").body(format!(
                    r#"
                        <!DOCTYPE html>
                        <html>
                        <head>
                            <title>History Loaded</title>
                            <meta charset="UTF-8">
                            <script>
                                window.opener.postMessage({{ 
                                    type: "HISTORY_LOADED", 
                                    tracks: {tracks_json}
                                }}, "*");
                                window.close();
                            </script>
                        </head>
                        <body>
                            <p>Loading your history...</p>
                        </body>
                        </html>
                        "#
                )))
            }
            Err(e) => {
                eprintln!("Error fetching recently played: {}", e);
                Ok(HttpResponse::Ok()
                        .content_type("text/html")
                        .body(format!(
                            r#"
                            <!DOCTYPE html>
                            <html>
                            <head>
                                <title>Error Loading History</title>
                                <meta charset="UTF-8">
                                <script>
                                    window.opener.postMessage({{ 
                                        type: "HISTORY_ERROR", 
                                        error: "Failed to load your recently played tracks. Please try again."
                                    }}, "*");
                                    window.close();
                                </script>
                            </head>
                            <body>
                                <h3>Error Loading History</h3>
                                <p>There was a problem loading your Spotify history.</p>
                                <p>The window will close automatically.</p>
                            </body>
                            </html>
                            "#
                        )))
            }
        },
        Err(e) => {
            eprintln!("Error requesting token: {}", e);
            Ok(HttpResponse::Ok().content_type("text/html").body(format!(
                r#"
                <!DOCTYPE html>
                <html>
                <head>
                    <title>Authentication Error</title>
                    <meta charset="UTF-8">
                    <script>
                        window.opener.postMessage({{ 
                            type: "HISTORY_ERROR", 
                            error: "Failed to authenticate with Spotify. Please try again."
                        }}, "*");
                        window.close();
                    </script>
                </head>
                <body>
                    <h3>Authentication Error</h3>
                    <p>There was a problem logging in to Spotify.</p>
                    <p>The window will close automatically.</p>
                </body>
                </html>
                "#
            )))
        }
    }
}

pub async fn get_history_auth_url() -> Result<HttpResponse, Error> {
    // ... existing get_history_auth_url implementation ...
    Ok(HttpResponse::Ok().json(serde_json::json!({"message": "Not implemented"})))
}

#[derive(Debug, Deserialize)]
pub struct GeminiPromptRequest {
    prompt: String,
}

pub async fn process_gemini_prompt(
    req: web::Json<GeminiPromptRequest>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    println!("Received prompt request: {}", req.prompt);

    if req.prompt.trim().is_empty() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Please provide a prompt for the playlist"
        })));
    }

    match data.gemini_service.generate_playlist(&req.prompt).await {
        Ok(playlist) => {
            if playlist.tracks.is_empty() {
                return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "No tracks were generated. Please try a different prompt."
                })));
            }

            println!(
                "Successfully generated playlist with {} tracks",
                playlist.tracks.len()
            );
            Ok(HttpResponse::Ok().json(playlist))
        }
        Err(e) => {
            eprintln!("Error generating playlist: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to generate playlist. Please try again."
            })))
        }
    }
}

pub async fn create_spotify_playlist_handler(
    data: web::Data<AppState>,
    req: web::Json<CreatePlaylistRequest>,
    session: Session,
) -> Result<HttpResponse, Error> {
    println!("Starting create_spotify_playlist_handler");

    // Generate a unique session ID
    let session_id = uuid::Uuid::new_v4().to_string();
    println!("Generated session ID: {}", session_id);

    // Store playlist request in application state
    {
        let mut pending_tracks = data.pending_tracks.lock().unwrap();
        pending_tracks.insert(session_id.clone(), req.0.clone());
        println!(
            "Stored tracks in pending_tracks with session ID: {}",
            session_id
        );
    }

    // Store session ID in both cookie and state parameter
    session.insert("tracks_session_id", &session_id)?;
    println!("Stored session ID in cookie: {}", session_id);

    let creds = Credentials::new(SPOTIFY_CLIENT_ID, SPOTIFY_CLIENT_SECRET);
    let oauth = OAuth {
        redirect_uri: SPOTIFY_REDIRECT_URI.to_string(),
        scopes: scopes!(
            "playlist-modify-public",
            "playlist-modify-private",
            "user-read-private",
            "user-read-email",
            "user-read-recently-played"
        ),
        state: session_id.clone(),
        ..Default::default()
    };

    let spotify = AuthCodeSpotify::new(creds, oauth);
    match spotify.get_authorize_url(false) {
        Ok(auth_url) => {
            println!("Generated Spotify auth URL successfully");
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "auth_url": auth_url,
                "session_id": session_id
            })))
        }
        Err(e) => {
            eprintln!("Error getting authorization URL: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to get authorization URL: {}", e)
            })))
        }
    }
}

async fn create_spotify_playlist(
    code: &str,
    tracks: Vec<Track>,
    playlist_name: String,
    playlist_description: Option<String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let creds = Credentials::new(SPOTIFY_CLIENT_ID, SPOTIFY_CLIENT_SECRET);
    let oauth = OAuth {
        redirect_uri: "http://127.0.0.1:8081/callback".to_string(),
        scopes: scopes!(
            "playlist-modify-public",
            "playlist-modify-private",
            "user-read-private",
            "user-read-email",
            "user-read-recently-played"
        ),
        ..Default::default()
    };

    let spotify = AuthCodeSpotify::new(creds, oauth);

    // Exchange the code for an access token
    spotify.request_token(code).await?;

    // Get user profile
    let user = spotify.me().await?;

    // Create a new playlist
    let playlist = spotify
        .user_playlist_create(
            user.id,
            &playlist_name,
            Some(false), // not public
            Some(false), // not collaborative
            playlist_description.as_deref(),
        )
        .await?;

    // Search for tracks and add them to the playlist
    let mut spotify_track_ids = Vec::new();

    for track in tracks {
        println!("Processing track: {} by {}", track.name, track.artist);
        let search_query = format!("{} {}", track.name, track.artist);

        if let SearchResult::Tracks(page) = spotify
            .search(&search_query, SearchType::Track, None, None, Some(1), None)
            .await?
        {
            if let Some(found_track) = page.items.first() {
                if let Some(track_id) = &found_track.id {
                    spotify_track_ids.push(track_id.clone());
                    println!("Added track to playlist queue: {}", found_track.name);
                }
            }
        }
    }

    // Add tracks to playlist if we found any
    if spotify_track_ids.is_empty() {
        return Err("No matching tracks found on Spotify".into());
    }

    // Add tracks in batches of 100 (Spotify API limit)
    for chunk in spotify_track_ids.chunks(100) {
        let track_ids: Vec<PlayableId> = chunk
            .iter()
            .map(|id| PlayableId::Track(id.clone()))
            .collect();

        spotify
            .playlist_add_items(playlist.id.clone(), track_ids, None)
            .await?;
    }

    Ok(playlist
        .external_urls
        .get("spotify")
        .map(String::as_str)
        .unwrap_or("")
        .to_string())
}
