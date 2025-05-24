use qrcode::render::svg;
use qrcode::QrCode;
use std::error::Error;

pub struct QrService;

impl QrService {
    pub fn generate_playlist_qr(playlist_url: &str) -> Result<String, Box<dyn Error>> {
        // Create QR code
        let code = QrCode::new(playlist_url)?;

        // Render as SVG with some styling
        let svg_xml = code
            .render::<svg::Color>()
            .min_dimensions(200, 200)
            .dark_color(svg::Color("#1DB954")) // Spotify green
            .light_color(svg::Color("#FFFFFF")) // White background
            .build();

        Ok(svg_xml)
    }
}
