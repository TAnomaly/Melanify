use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsService {
    pub total_listening_time: i32,
    pub favorite_genres: Vec<GenreStats>,
    pub favorite_artists: Vec<ArtistStats>,
    pub listening_history: Vec<ListeningRecord>,
    pub daily_stats: Vec<DailyStats>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenreStats {
    pub genre: String,
    pub percentage: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArtistStats {
    pub artist_name: String,
    pub listen_count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListeningRecord {
    pub track_name: String,
    pub artist_name: String,
    pub listened_at: String,
    pub duration: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyStats {
    pub date: String,
    pub total_minutes: i32,
}

impl StatisticsService {
    pub async fn get_user_statistics(_user_id: &str) -> Result<Self, Box<dyn Error>> {
        // Mock data for now
        Ok(Self {
            total_listening_time: 1234,
            favorite_genres: vec![
                GenreStats {
                    genre: "Rock".to_string(),
                    percentage: 35.5,
                },
                GenreStats {
                    genre: "Pop".to_string(),
                    percentage: 25.0,
                },
            ],
            favorite_artists: vec![
                ArtistStats {
                    artist_name: "The Beatles".to_string(),
                    listen_count: 50,
                },
                ArtistStats {
                    artist_name: "Queen".to_string(),
                    listen_count: 45,
                },
            ],
            listening_history: vec![ListeningRecord {
                track_name: "Yesterday".to_string(),
                artist_name: "The Beatles".to_string(),
                listened_at: "2024-03-20T10:30:00Z".to_string(),
                duration: 180,
            }],
            daily_stats: vec![
                DailyStats {
                    date: "2024-03-20".to_string(),
                    total_minutes: 120,
                },
                DailyStats {
                    date: "2024-03-19".to_string(),
                    total_minutes: 90,
                },
            ],
        })
    }

    pub async fn get_listening_history() -> Result<Vec<ListeningRecord>, Box<dyn Error>> {
        let stats = Self::get_user_statistics("dummy").await?;
        Ok(stats.listening_history)
    }

    pub async fn get_daily_stats() -> Result<Vec<DailyStats>, Box<dyn Error>> {
        let stats = Self::get_user_statistics("dummy").await?;
        Ok(stats.daily_stats)
    }
}
