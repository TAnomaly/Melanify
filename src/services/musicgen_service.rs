use crate::models::playlist::{
    AiMusicBatchRequest, AiMusicBatchResponse, AiMusicRequest, AiMusicResponse,
};
use reqwest::Client;
use serde_json::json;
use std::error::Error;

#[derive(Debug)]
pub struct MusicGenService {
    api_url: String,
    client: Client,
}

impl Clone for MusicGenService {
    fn clone(&self) -> Self {
        Self {
            api_url: self.api_url.clone(),
            client: Client::new(),
        }
    }
}

impl MusicGenService {
    /// Create a new MusicGenService instance
    /// Default API URL is http://localhost:5000
    pub fn new() -> Self {
        Self {
            api_url: "http://localhost:5000".to_string(),
            client: Client::new(),
        }
    }

    /// Create a new MusicGenService with custom API URL
    pub fn with_url(api_url: String) -> Self {
        Self {
            api_url,
            client: Client::new(),
        }
    }

    /// Check if the AI music service is healthy and running
    pub async fn health_check(&self) -> Result<bool, Box<dyn Error>> {
        let url = format!("{}/health", self.api_url);

        match self.client.get(&url).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(e) => {
                eprintln!("Health check failed: {}", e);
                Err(format!(
                    "AI Music service is not running. Please start the Python service at {}",
                    self.api_url
                )
                .into())
            }
        }
    }

    /// Generate a single AI song from a text prompt
    pub async fn generate_song(
        &self,
        prompt: &str,
        duration: Option<u32>,
    ) -> Result<AiMusicResponse, Box<dyn Error>> {
        println!("Generating AI music with prompt: {}", prompt);

        // First check if service is running
        if !self.health_check().await? {
            return Err("AI Music service is not available".into());
        }

        let url = format!("{}/generate", self.api_url);

        let request_body = json!({
            "prompt": prompt,
            "duration": duration.unwrap_or(10),
            "return_url": true
        });

        println!("Making request to MusicGen API: {}", url);

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let status = response.status();
        println!("MusicGen API response status: {}", status);

        if !status.is_success() {
            let error_text = response.text().await?;
            eprintln!("MusicGen API error response: {}", error_text);
            return Err(format!("MusicGen API error ({}): {}", status, error_text).into());
        }

        let response_json: AiMusicResponse = response.json().await?;

        if !response_json.success {
            return Err("Failed to generate AI music".into());
        }

        println!(
            "Successfully generated AI music: {}",
            response_json.file_id
        );

        Ok(response_json)
    }

    /// Generate multiple AI songs from a list of prompts
    pub async fn generate_batch(
        &self,
        request: AiMusicBatchRequest,
    ) -> Result<AiMusicBatchResponse, Box<dyn Error>> {
        println!("Generating batch of {} AI songs", request.prompts.len());

        // First check if service is running
        if !self.health_check().await? {
            return Err("AI Music service is not available".into());
        }

        let url = format!("{}/batch-generate", self.api_url);

        let request_body = json!({
            "prompts": request.prompts,
            "duration": request.duration.unwrap_or(10)
        });

        println!("Making batch request to MusicGen API: {}", url);

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let status = response.status();
        println!("MusicGen API batch response status: {}", status);

        if !status.is_success() {
            let error_text = response.text().await?;
            eprintln!("MusicGen API batch error response: {}", error_text);
            return Err(format!("MusicGen API error ({}): {}", status, error_text).into());
        }

        let response_json: AiMusicBatchResponse = response.json().await?;

        if !response_json.success {
            return Err("Failed to generate AI music batch".into());
        }

        println!(
            "Successfully generated {} AI songs",
            response_json.songs.len()
        );

        Ok(response_json)
    }

    /// Get the download URL for a generated song
    pub fn get_download_url(&self, file_id: &str) -> String {
        format!("{}/download/{}", self.api_url, file_id)
    }
}
