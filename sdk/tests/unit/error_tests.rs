//! Unit tests for error handling.

use allscreenshots_sdk::{AllscreenshotsError, ErrorCode};

#[test]
fn test_error_code_from_str() {
    assert_eq!(ErrorCode::from("VALIDATION_ERROR"), ErrorCode::ValidationError);
    assert_eq!(ErrorCode::from("UNAUTHORIZED"), ErrorCode::Unauthorized);
    assert_eq!(ErrorCode::from("NOT_FOUND"), ErrorCode::NotFound);
    assert_eq!(ErrorCode::from("RATE_LIMIT_EXCEEDED"), ErrorCode::RateLimitExceeded);
    assert_eq!(ErrorCode::from("INTERNAL_ERROR"), ErrorCode::InternalError);
    assert_eq!(ErrorCode::from("CANCELLED"), ErrorCode::Cancelled);
    assert_eq!(ErrorCode::from("TIMEOUT"), ErrorCode::Timeout);
    assert_eq!(ErrorCode::from("NETWORK_ERROR"), ErrorCode::NetworkError);
    assert_eq!(ErrorCode::from("UNKNOWN_CODE"), ErrorCode::Unknown("UNKNOWN_CODE".to_string()));
}

#[test]
fn test_error_code_display() {
    assert_eq!(format!("{}", ErrorCode::ValidationError), "VALIDATION_ERROR");
    assert_eq!(format!("{}", ErrorCode::Unauthorized), "UNAUTHORIZED");
    assert_eq!(format!("{}", ErrorCode::Unknown("CUSTOM".to_string())), "CUSTOM");
}

#[test]
fn test_api_error_retryable() {
    let error = AllscreenshotsError::ApiError {
        code: ErrorCode::RateLimitExceeded,
        message: "Rate limit exceeded".to_string(),
        status: 429,
    };
    assert!(error.is_retryable());

    let error = AllscreenshotsError::ApiError {
        code: ErrorCode::InternalError,
        message: "Internal error".to_string(),
        status: 500,
    };
    assert!(error.is_retryable());

    let error = AllscreenshotsError::ApiError {
        code: ErrorCode::ValidationError,
        message: "Invalid input".to_string(),
        status: 400,
    };
    assert!(!error.is_retryable());
}

#[test]
fn test_validation_error_not_retryable() {
    let error = AllscreenshotsError::ValidationError("Invalid URL".to_string());
    assert!(!error.is_retryable());
}

#[test]
fn test_timeout_error_retryable() {
    let error = AllscreenshotsError::Timeout;
    assert!(error.is_retryable());
}

#[test]
fn test_config_error_not_retryable() {
    let error = AllscreenshotsError::ConfigError("Invalid config".to_string());
    assert!(!error.is_retryable());
}

#[test]
fn test_env_var_not_set_error() {
    let error = AllscreenshotsError::EnvVarNotSet("ALLSCREENSHOTS_API_KEY".to_string());
    assert!(!error.is_retryable());
    assert!(format!("{}", error).contains("ALLSCREENSHOTS_API_KEY"));
}

#[test]
fn test_api_error_from_response() {
    let error = AllscreenshotsError::from_api_response(
        400,
        Some("VALIDATION_ERROR"),
        "URL is required",
    );

    match error {
        AllscreenshotsError::ApiError { code, message, status } => {
            assert_eq!(code, ErrorCode::ValidationError);
            assert_eq!(message, "URL is required");
            assert_eq!(status, 400);
        }
        _ => panic!("Expected ApiError"),
    }
}

#[test]
fn test_api_error_without_code() {
    let error = AllscreenshotsError::from_api_response(
        500,
        None,
        "Something went wrong",
    );

    match error {
        AllscreenshotsError::ApiError { code, message, status } => {
            assert!(matches!(code, ErrorCode::Unknown(_)));
            assert_eq!(message, "Something went wrong");
            assert_eq!(status, 500);
        }
        _ => panic!("Expected ApiError"),
    }
}

#[test]
fn test_error_display() {
    let error = AllscreenshotsError::ApiError {
        code: ErrorCode::ValidationError,
        message: "URL is required".to_string(),
        status: 400,
    };
    let display = format!("{}", error);
    assert!(display.contains("VALIDATION_ERROR"));
    assert!(display.contains("URL is required"));

    let error = AllscreenshotsError::ValidationError("Invalid input".to_string());
    let display = format!("{}", error);
    assert!(display.contains("Invalid input"));
}
