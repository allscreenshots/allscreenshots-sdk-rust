//! Bulk screenshot request and response models.

use serde::{Deserialize, Serialize};
use super::common::*;

/// Request for bulk screenshots.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkRequest {
    /// List of URLs to capture (max 100)
    pub urls: Vec<BulkUrlRequest>,
    /// Default options for all URLs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaults: Option<BulkDefaults>,
    /// Webhook URL for notifications
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
    /// Secret for webhook signature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_secret: Option<String>,
}

impl BulkRequest {
    /// Create a new bulk request with the given URLs.
    pub fn new(urls: Vec<BulkUrlRequest>) -> Self {
        Self {
            urls,
            defaults: None,
            webhook_url: None,
            webhook_secret: None,
        }
    }

    /// Set default options for all URLs.
    pub fn with_defaults(mut self, defaults: BulkDefaults) -> Self {
        self.defaults = Some(defaults);
        self
    }

    /// Set the webhook URL.
    pub fn with_webhook(mut self, url: String, secret: Option<String>) -> Self {
        self.webhook_url = Some(url);
        self.webhook_secret = secret;
        self
    }
}

/// URL configuration for bulk requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkUrlRequest {
    /// Target URL
    pub url: String,
    /// URL-specific options (override defaults)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<BulkUrlOptions>,
}

impl BulkUrlRequest {
    /// Create a simple URL request.
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            options: None,
        }
    }

    /// Create a URL request with options.
    pub fn with_options(url: impl Into<String>, options: BulkUrlOptions) -> Self {
        Self {
            url: url.into(),
            options: Some(options),
        }
    }
}

/// URL-specific options for bulk requests.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BulkUrlOptions {
    /// Viewport configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewport: Option<ViewportConfig>,
    /// Device preset name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    /// Output image format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ImageFormat>,
    /// Capture full page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_page: Option<bool>,
    /// Image quality (1-100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<i32>,
    /// Delay before capture in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<i32>,
    /// CSS selector to wait for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_for: Option<String>,
    /// Page load condition to wait for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_until: Option<WaitUntil>,
    /// Timeout in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    /// Enable dark mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark_mode: Option<bool>,
    /// Custom CSS to inject
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_css: Option<String>,
    /// Block ads
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_ads: Option<bool>,
    /// Block cookie banners
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_cookie_banners: Option<bool>,
    /// Blocking level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_level: Option<BlockLevel>,
}

/// Default options for bulk requests.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BulkDefaults {
    /// Viewport configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewport: Option<ViewportConfig>,
    /// Device preset name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    /// Output image format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ImageFormat>,
    /// Capture full page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_page: Option<bool>,
    /// Image quality (1-100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<i32>,
    /// Delay before capture in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<i32>,
    /// CSS selector to wait for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_for: Option<String>,
    /// Page load condition to wait for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_until: Option<WaitUntil>,
    /// Timeout in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    /// Enable dark mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark_mode: Option<bool>,
    /// Custom CSS to inject
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_css: Option<String>,
    /// Block ads
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_ads: Option<bool>,
    /// Block cookie banners
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_cookie_banners: Option<bool>,
    /// Blocking level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_level: Option<BlockLevel>,
}

/// Response for bulk job creation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkResponse {
    /// Bulk job ID
    pub id: String,
    /// Current status
    pub status: String,
    /// Total number of jobs
    pub total_jobs: i32,
    /// Number of completed jobs
    pub completed_jobs: i32,
    /// Number of failed jobs
    pub failed_jobs: i32,
    /// Progress percentage (0-100)
    pub progress: i32,
    /// Individual job information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<BulkJobInfo>>,
    /// Creation timestamp
    pub created_at: Option<String>,
    /// Completion timestamp
    pub completed_at: Option<String>,
}

/// Basic job information for bulk requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkJobInfo {
    /// Job ID
    pub id: String,
    /// Target URL
    pub url: String,
    /// Job status
    pub status: String,
}

/// Summary of a bulk job.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkJobSummary {
    /// Bulk job ID
    pub id: String,
    /// Current status
    pub status: String,
    /// Total number of jobs
    pub total_jobs: i32,
    /// Number of completed jobs
    pub completed_jobs: i32,
    /// Number of failed jobs
    pub failed_jobs: i32,
    /// Progress percentage (0-100)
    pub progress: i32,
    /// Creation timestamp
    pub created_at: Option<String>,
    /// Completion timestamp
    pub completed_at: Option<String>,
}

/// Detailed status response for a bulk job.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkStatusResponse {
    /// Bulk job ID
    pub id: String,
    /// Current status
    pub status: String,
    /// Total number of jobs
    pub total_jobs: i32,
    /// Number of completed jobs
    pub completed_jobs: i32,
    /// Number of failed jobs
    pub failed_jobs: i32,
    /// Progress percentage (0-100)
    pub progress: i32,
    /// Detailed job information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<BulkJobDetailInfo>>,
    /// Creation timestamp
    pub created_at: Option<String>,
    /// Completion timestamp
    pub completed_at: Option<String>,
}

/// Detailed job information for bulk status.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkJobDetailInfo {
    /// Job ID
    pub id: String,
    /// Target URL
    pub url: String,
    /// Job status
    pub status: String,
    /// URL to download the result
    pub result_url: Option<String>,
    /// Storage URL
    pub storage_url: Option<String>,
    /// Output format
    pub format: Option<String>,
    /// Image width
    pub width: Option<i32>,
    /// Image height
    pub height: Option<i32>,
    /// File size in bytes
    pub file_size: Option<i64>,
    /// Render time in milliseconds
    pub render_time_ms: Option<i64>,
    /// Error code if failed
    pub error_code: Option<String>,
    /// Error message if failed
    pub error_message: Option<String>,
    /// Creation timestamp
    pub created_at: Option<String>,
    /// Completion timestamp
    pub completed_at: Option<String>,
}
