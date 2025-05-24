use crate::models::playlist::{GeminiPromptResponse, GeminiTrack};
use reqwest::Client;
use serde_json::json;
use std::error::Error;

#[derive(Debug)]
pub struct GeminiService {
    api_key: String,
    #[doc(hidden)]
    client: Client,
}

impl Clone for GeminiService {
    fn clone(&self) -> Self {
        Self {
            api_key: self.api_key.clone(),
            client: Client::new(),
        }
    }
}

impl GeminiService {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
        }
    }

    pub async fn generate_playlist(
        &self,
        prompt: &str,
    ) -> Result<GeminiPromptResponse, Box<dyn Error>> {
        println!(
            "Starting playlist generation with API key length: {}",
            self.api_key.len()
        );
        println!("Generating playlist with prompt: {}", prompt);

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}",
            self.api_key
        );
        println!("Using Gemini API URL: {}", url);

        // Format the request prompt to ask for specific song suggestions
        let instruction = format!(
            "Based on this prompt: '{}', create a cohesive music playlist.
            
            Think deeply about what kind of music would fit this theme or mood. Then suggest 5-10 specific songs (with correct artist names) that would make a great playlist.
            
            Your response should be in JSON format with the following structure:
            {{
                \"tracks\": [
                    {{ \"title\": \"Song Title 1\", \"artist\": \"Artist Name 1\" }},
                    {{ \"title\": \"Song Title 2\", \"artist\": \"Artist Name 2\" }},
                    ...
                ],
                \"playlist_name\": \"Suggested Playlist Name\",
                \"playlist_description\": \"A description explaining the playlist concept and how these songs fit together\"
            }}
            
            Be thoughtful in your song selections, ensuring they're real songs by real artists that can be found on music streaming platforms.",
            prompt
        );

        println!(
            "Making request to Gemini API with instruction: {}",
            instruction
        );
        let request_body = json!({
            "contents": [{
                "parts": [{
                    "text": instruction
                }]
            }],
            "generationConfig": {
                "responseMimeType": "application/json",
                "responseSchema": {
                    "type": "OBJECT",
                    "properties": {
                        "tracks": {
                            "type": "ARRAY",
                            "items": {
                                "type": "OBJECT",
                                "properties": {
                                    "title": {"type": "STRING"},
                                    "artist": {"type": "STRING"}
                                },
                                "required": ["title", "artist"]
                            }
                        },
                        "playlist_name": {
                            "type": "STRING"
                        },
                        "playlist_description": {
                            "type": "STRING"
                        }
                    },
                    "required": ["tracks", "playlist_name", "playlist_description"]
                }
            }
        });
        println!(
            "Request body: {}",
            serde_json::to_string_pretty(&request_body).unwrap()
        );

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let status = response.status();
        println!("Gemini API response status: {}", status);

        if !status.is_success() {
            let error_text = response.text().await?;
            eprintln!("Gemini API error response: {}", error_text);
            return Err(format!("Gemini API error ({}): {}", status, error_text).into());
        }

        let response_text = response.text().await?;
        println!("Raw Gemini Response: {}", response_text);

        let response_json: serde_json::Value =
            serde_json::from_str(&response_text).map_err(|e| {
                eprintln!("Failed to parse response as JSON: {}", e);
                format!("Failed to parse Gemini response as JSON: {}", e)
            })?;

        // Extract the text output from the response
        if let Some(candidates) = response_json.get("candidates").and_then(|c| c.as_array()) {
            if let Some(candidate) = candidates.first() {
                if let Some(content) = candidate.get("content") {
                    if let Some(parts) = content.get("parts").and_then(|p| p.as_array()) {
                        if let Some(part) = parts.first() {
                            if let Some(text) = part.get("text").and_then(|t| t.as_str()) {
                                println!("Generated text from Gemini: {}", text);

                                // Parse the JSON response
                                match serde_json::from_str::<GeminiPromptResponse>(text) {
                                    Ok(result) => {
                                        if result.tracks.is_empty() {
                                            eprintln!("Generated playlist has no tracks");
                                            return Err("Generated playlist has no tracks".into());
                                        }
                                        println!(
                                            "Successfully parsed playlist with {} tracks",
                                            result.tracks.len()
                                        );
                                        return Ok(result);
                                    }
                                    Err(e) => {
                                        eprintln!("Error parsing Gemini response as JSON: {}", e);
                                        eprintln!("Raw response: {}", text);
                                        return Err("Failed to parse AI response. Please try a different prompt.".into());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Err("Invalid response structure from AI".into())
    }
}
