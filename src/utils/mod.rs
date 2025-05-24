use std::error::Error;

pub fn format_error_response(error: Box<dyn Error>) -> String {
    format!("An error occurred: {}", error)
}

pub fn generate_session_id() -> String {
    uuid::Uuid::new_v4().to_string()
}
