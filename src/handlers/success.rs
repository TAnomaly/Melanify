use crate::services::qr_service::QrService;
use actix_web::{Error, HttpResponse};

pub async fn render_success_page(playlist_url: &str) -> Result<HttpResponse, Error> {
    let qr_code = QrService::generate_playlist_qr(playlist_url)
        .unwrap_or_else(|_| String::from("<!-- QR code generation failed -->"));

    Ok(HttpResponse::Ok().content_type("text/html").body(format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Success!</title>
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
        h1 {{
            font-size: 2.5em;
            margin-bottom: 20px;
            color: #1DB954;
        }}
        p {{
            font-size: 1.2em;
            margin-bottom: 30px;
            line-height: 1.6;
        }}
        .button {{
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
        }}
        .button:hover {{
            transform: translateY(-2px);
            box-shadow: 0 5px 15px rgba(29, 185, 84, 0.3);
            background: #1ed760;
        }}
        .qr-container {{
            margin: 30px auto;
            padding: 20px;
            background: white;
            border-radius: 10px;
            width: fit-content;
        }}
        .qr-code {{
            width: 200px;
            height: 200px;
        }}
        .qr-text {{
            color: #191414;
            margin-top: 15px;
            font-size: 0.9em;
        }}
    </style>
</head>
<body>
    <div class="container">
        <h1>✨ Success!</h1>
        <p>Your new playlist has been created in your Spotify account.</p>
        <div class="qr-container">
            {qr_code}
            <p class="qr-text">Scan to open playlist on your phone</p>
        </div>
        <a href="{playlist_url}" class="button" target="_blank">Open in Spotify</a>
        <a href="/" class="button">Create Another Playlist</a>
    </div>
</body>
</html>"#
    )))
}
