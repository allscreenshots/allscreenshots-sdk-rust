//! Error types for the Allscreenshots SDK.

use thiserror::Error;

/// Error codes returned by the Allscreenshots API.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorCode {
    /// Invalid request parameters
    ValidationError,
    /// Authentication failed
    Unauthorized,
    /// Resource not found
    NotFound,
    /// Rate limit exceeded
    RateLimitExceeded,
    /// Internal server error
    InternalError,
    /// Job was cancelled
    Cancelled,
    /// Request timeout
    Timeout,
    /// Network error
    NetworkError,
    /// Unknown error code
    Unknown(String),
}

impl From<&str> for ErrorCode {
    fn from(s: &str) -> Self {
        match s {
            "VALIDATION_ERROR" => ErrorCode::ValidationError,
            "UNAUTHORIZED" => ErrorCode::Unauthorized,
            "NOT_FOUND" => ErrorCode::NotFound,
            "RATE_LIMIT_EXCEEDED" => ErrorCode::RateLimitExceeded,
            "INTERNAL_ERROR" => ErrorCode::InternalError,
            "CANCELLED" => ErrorCode::Cancelled,
            "TIMEOUT" => ErrorCode::Timeout,
            "NETWORK_ERROR" => ErrorCode::NetworkError,
            _ => ErrorCode::Unknown(s.to_string()),
        }
    }
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorCode::ValidationError => write!(f, "VALIDATION_ERROR"),
            ErrorCode::Unauthorized => write!(f, "UNAUTHORIZED"),
            ErrorCode::NotFound => write!(f, "NOT_FOUND"),
            ErrorCode::RateLimitExceeded => write!(f, "RATE_LIMIT_EXCEEDED"),
            ErrorCode::InternalError => write!(f, "INTERNAL_ERROR"),
            ErrorCode::Cancelled => write!(f, "CANCELLED"),
            ErrorCode::Timeout => write!(f, "TIMEOUT"),
            ErrorCode::NetworkError => write!(f, "NETWORK_ERROR"),
            ErrorCode::Unknown(s) => write!(f, "{}", s),
        }
    }
}

/// The main error type for the Allscreenshots SDK.
#[derive(Error, Debug)]
pub enum AllscreenshotsError {
    /// API returned an error response
    #[error("API error ({code}): {message}")]
    ApiError {
        /// The error code from the API
        code: ErrorCode,
        /// The error message from the API
        message: String,
        /// HTTP status code
        status: u16,
    },

    /// Validation error for request parameters
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// HTTP request failed
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    /// Failed to parse URL
    #[error("Invalid URL: {0}")]
    UrlError(#[from] url::ParseError),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// Environment variable not set
    #[error("Environment variable '{0}' not set")]
    EnvVarNotSet(String),

    /// All retries exhausted
    #[error("All retries exhausted: {0}")]
    RetriesExhausted(String),

    /// Request timeout
    #[error("Request timeout")]
    Timeout,
}

impl AllscreenshotsError {
    /// Returns `true` if the error is retryable.
    pub fn is_retryable(&self) -> bool {
        match self {
            AllscreenshotsError::ApiError { code, status, .. } => {
                matches!(code, ErrorCode::RateLimitExceeded | ErrorCode::InternalError)
                    || *status >= 500
            }
            AllscreenshotsError::HttpError(e) => e.is_timeout() || e.is_connect(),
            AllscreenshotsError::Timeout => true,
            _ => false,
        }
    }

    /// Creates an API error from response data.
    pub fn from_api_response(status: u16, code: Option<&str>, message: &str) -> Self {
        AllscreenshotsError::ApiError {
            code: code.map(ErrorCode::from).unwrap_or(ErrorCode::Unknown("UNKNOWN".to_string())),
            message: message.to_string(),
            status,
        }
    }
}

/// API error response structure for deserialization.
#[derive(Debug, serde::Deserialize)]
pub(crate) struct ApiErrorResponse {
    #[serde(rename = "errorCode")]
    pub error_code: Option<String>,
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
    pub message: Option<String>,
    pub error: Option<String>,
}

impl ApiErrorResponse {
    /// Get the error message from any available field.
    pub fn get_message(&self) -> String {
        self.error_message
            .as_ref()
            .or(self.message.as_ref())
            .or(self.error.as_ref())
            .cloned()
            .unwrap_or_else(|| "Unknown error".to_string())
    }
}
