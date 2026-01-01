//! Screenshot-related request and response models.

use crate::error::AllscreenshotsError;
use serde::{Deserialize, Serialize};
use super::common::*;

/// Request to take a screenshot.
///
/// # Example
///
/// ```rust
/// use allscreenshots_sdk::{ScreenshotRequest, ImageFormat};
///
/// let request = ScreenshotRequest::builder()
///     .url("https://github.com")
///     .device("Desktop HD")
///     .full_page(true)
///     .format(ImageFormat::Png)
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScreenshotRequest {
    /// Target URL to capture (required)
    pub url: String,
    /// Viewport configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewport: Option<ViewportConfig>,
    /// Device preset name (e.g., "Desktop HD", "iPhone 14", "iPad")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    /// Output image format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ImageFormat>,
    /// Capture full page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_page: Option<bool>,
    /// Image quality (1-100, for JPEG/WebP)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<i32>,
    /// Delay before capture in milliseconds (0-30000)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<i32>,
    /// CSS selector to wait for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_for: Option<String>,
    /// Page load condition to wait for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_until: Option<WaitUntil>,
    /// Timeout in milliseconds (1000-60000)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    /// Enable dark mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark_mode: Option<bool>,
    /// Custom CSS to inject
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_css: Option<String>,
    /// CSS selectors to hide
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_selectors: Option<Vec<String>>,
    /// CSS selector of element to capture
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    /// Block ads
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_ads: Option<bool>,
    /// Block cookie banners
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_cookie_banners: Option<bool>,
    /// Blocking level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_level: Option<BlockLevel>,
    /// Webhook URL for notifications
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
    /// Secret for webhook signature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_secret: Option<String>,
    /// Response type (BINARY or JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_type: Option<ResponseType>,
}

impl ScreenshotRequest {
    /// Create a new builder for ScreenshotRequest.
    pub fn builder() -> ScreenshotRequestBuilder {
        ScreenshotRequestBuilder::default()
    }

    /// Create a simple screenshot request for a URL.
    pub fn simple(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            ..Default::default()
        }
    }
}

/// Builder for ScreenshotRequest.
#[derive(Debug, Default)]
pub struct ScreenshotRequestBuilder {
    request: ScreenshotRequest,
}

impl ScreenshotRequestBuilder {
    /// Set the target URL (required).
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.request.url = url.into();
        self
    }

    /// Set the viewport configuration.
    pub fn viewport(mut self, viewport: ViewportConfig) -> Self {
        self.request.viewport = Some(viewport);
        self
    }

    /// Set the device preset.
    pub fn device(mut self, device: impl Into<String>) -> Self {
        self.request.device = Some(device.into());
        self
    }

    /// Set the output format.
    pub fn format(mut self, format: ImageFormat) -> Self {
        self.request.format = Some(format);
        self
    }

    /// Enable or disable full page capture.
    pub fn full_page(mut self, full_page: bool) -> Self {
        self.request.full_page = Some(full_page);
        self
    }

    /// Set the image quality (1-100).
    pub fn quality(mut self, quality: i32) -> Self {
        self.request.quality = Some(quality);
        self
    }

    /// Set the delay before capture in milliseconds.
    pub fn delay(mut self, delay: i32) -> Self {
        self.request.delay = Some(delay);
        self
    }

    /// Set a CSS selector to wait for.
    pub fn wait_for(mut self, selector: impl Into<String>) -> Self {
        self.request.wait_for = Some(selector.into());
        self
    }

    /// Set the page load condition to wait for.
    pub fn wait_until(mut self, condition: WaitUntil) -> Self {
        self.request.wait_until = Some(condition);
        self
    }

    /// Set the timeout in milliseconds.
    pub fn timeout(mut self, timeout: i32) -> Self {
        self.request.timeout = Some(timeout);
        self
    }

    /// Enable or disable dark mode.
    pub fn dark_mode(mut self, dark_mode: bool) -> Self {
        self.request.dark_mode = Some(dark_mode);
        self
    }

    /// Set custom CSS to inject.
    pub fn custom_css(mut self, css: impl Into<String>) -> Self {
        self.request.custom_css = Some(css.into());
        self
    }

    /// Set CSS selectors to hide.
    pub fn hide_selectors(mut self, selectors: Vec<String>) -> Self {
        self.request.hide_selectors = Some(selectors);
        self
    }

    /// Set a CSS selector for the element to capture.
    pub fn selector(mut self, selector: impl Into<String>) -> Self {
        self.request.selector = Some(selector.into());
        self
    }

    /// Enable or disable ad blocking.
    pub fn block_ads(mut self, block: bool) -> Self {
        self.request.block_ads = Some(block);
        self
    }

    /// Enable or disable cookie banner blocking.
    pub fn block_cookie_banners(mut self, block: bool) -> Self {
        self.request.block_cookie_banners = Some(block);
        self
    }

    /// Set the blocking level.
    pub fn block_level(mut self, level: BlockLevel) -> Self {
        self.request.block_level = Some(level);
        self
    }

    /// Set the webhook URL.
    pub fn webhook_url(mut self, url: impl Into<String>) -> Self {
        self.request.webhook_url = Some(url.into());
        self
    }

    /// Set the webhook secret.
    pub fn webhook_secret(mut self, secret: impl Into<String>) -> Self {
        self.request.webhook_secret = Some(secret.into());
        self
    }

    /// Set the response type.
    pub fn response_type(mut self, response_type: ResponseType) -> Self {
        self.request.response_type = Some(response_type);
        self
    }

    /// Build the request, validating required fields.
    pub fn build(self) -> Result<ScreenshotRequest, AllscreenshotsError> {
        if self.request.url.is_empty() {
            return Err(AllscreenshotsError::ValidationError(
                "URL is required".to_string(),
            ));
        }

        // Validate URL format
        if !self.request.url.starts_with("http://") && !self.request.url.starts_with("https://") {
            return Err(AllscreenshotsError::ValidationError(
                "URL must start with http:// or https://".to_string(),
            ));
        }

        // Validate quality if set
        if let Some(quality) = self.request.quality {
            if !(1..=100).contains(&quality) {
                return Err(AllscreenshotsError::ValidationError(
                    "Quality must be between 1 and 100".to_string(),
                ));
            }
        }

        // Validate delay if set
        if let Some(delay) = self.request.delay {
            if !(0..=30000).contains(&delay) {
                return Err(AllscreenshotsError::ValidationError(
                    "Delay must be between 0 and 30000 milliseconds".to_string(),
                ));
            }
        }

        // Validate timeout if set
        if let Some(timeout) = self.request.timeout {
            if !(1000..=60000).contains(&timeout) {
                return Err(AllscreenshotsError::ValidationError(
                    "Timeout must be between 1000 and 60000 milliseconds".to_string(),
                ));
            }
        }

        Ok(self.request)
    }
}

/// Response for an async screenshot job creation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AsyncJobCreatedResponse {
    /// Job ID
    pub id: String,
    /// Current job status
    pub status: JobStatus,
    /// URL to check job status
    pub status_url: Option<String>,
    /// Creation timestamp
    pub created_at: Option<String>,
}

/// Response for a screenshot job status.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobResponse {
    /// Job ID
    pub id: String,
    /// Current job status
    pub status: JobStatus,
    /// Original URL that was captured
    pub url: Option<String>,
    /// URL to download the result
    pub result_url: Option<String>,
    /// Error code if failed
    pub error_code: Option<String>,
    /// Error message if failed
    pub error_message: Option<String>,
    /// Creation timestamp
    pub created_at: Option<String>,
    /// Start timestamp
    pub started_at: Option<String>,
    /// Completion timestamp
    pub completed_at: Option<String>,
    /// Expiration timestamp
    pub expires_at: Option<String>,
    /// Additional metadata
    pub metadata: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_valid() {
        let request = ScreenshotRequest::builder()
            .url("https://example.com")
            .device("Desktop HD")
            .full_page(true)
            .build();

        assert!(request.is_ok());
        let request = request.unwrap();
        assert_eq!(request.url, "https://example.com");
        assert_eq!(request.device, Some("Desktop HD".to_string()));
        assert_eq!(request.full_page, Some(true));
    }

    #[test]
    fn test_builder_missing_url() {
        let result = ScreenshotRequest::builder().build();
        assert!(result.is_err());
    }

    #[test]
    fn test_builder_invalid_url() {
        let result = ScreenshotRequest::builder()
            .url("not-a-valid-url")
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_builder_invalid_quality() {
        let result = ScreenshotRequest::builder()
            .url("https://example.com")
            .quality(150)
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_simple_request() {
        let request = ScreenshotRequest::simple("https://example.com");
        assert_eq!(request.url, "https://example.com");
    }
}
