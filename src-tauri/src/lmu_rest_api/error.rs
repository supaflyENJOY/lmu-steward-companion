//! Error types for LMU REST API operations

use thiserror::Error;

/// Custom error type for LMU API operations
#[derive(Error, Debug)]
pub enum LmuApiError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("JSON parsing failed: {0}")]
    Json(#[from] serde_json::Error),
    #[error("API error: {status} - {message}")]
    Api { status: u16, message: String },
}

/// Result type for LMU API operations
pub type LmuApiResult<T> = Result<T, LmuApiError>;
