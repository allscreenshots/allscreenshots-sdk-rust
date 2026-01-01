//! Compose (multi-screenshot layout) request and response models.

use serde::{Deserialize, Serialize};
use super::common::*;

/// Layout type for composed images.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LayoutType {
    /// Grid layout
    #[default]
    Grid,
    /// Horizontal layout
    Horizontal,
    /// Vertical layout
    Vertical,
    /// Masonry layout
    Masonry,
    /// Mondrian layout
    Mondrian,
    /// Partitioning layout
    Partitioning,
    /// Auto-select best layout
    Auto,
}

/// Alignment for composed images.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Alignment {
    /// Align to top
    Top,
    /// Align to center
    #[default]
    Center,
    /// Align to bottom
    Bottom,
}

/// Request to compose multiple screenshots.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComposeRequest {
    /// List of URLs/captures to include (max 20)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captures: Option<Vec<CaptureItem>>,
    /// Single URL (use with variants)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Different configurations for the same URL (max 20)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variants: Option<Vec<VariantConfig>>,
    /// Default options for all captures
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaults: Option<CaptureDefaults>,
    /// Output configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<ComposeOutputConfig>,
    /// Run asynchronously
    #[serde(rename = "async", skip_serializing_if = "Option::is_none")]
    pub is_async: Option<bool>,
    /// Webhook URL for notifications
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
    /// Secret for webhook signature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_secret: Option<String>,
    /// Use captures mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captures_mode: Option<bool>,
    /// Use variants mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variants_mode: Option<bool>,
}

impl ComposeRequest {
    /// Create a compose request with multiple captures.
    pub fn with_captures(captures: Vec<CaptureItem>) -> Self {
        Self {
            captures: Some(captures),
            url: None,
            variants: None,
            defaults: None,
            output: None,
            is_async: None,
            webhook_url: None,
            webhook_secret: None,
            captures_mode: None,
            variants_mode: None,
        }
    }

    /// Create a compose request with variants of a single URL.
    pub fn with_variants(url: String, variants: Vec<VariantConfig>) -> Self {
        Self {
            captures: None,
            url: Some(url),
            variants: Some(variants),
            defaults: None,
            output: None,
            is_async: None,
            webhook_url: None,
            webhook_secret: None,
            captures_mode: None,
            variants_mode: None,
        }
    }

    /// Set the output configuration.
    pub fn with_output(mut self, output: ComposeOutputConfig) -> Self {
        self.output = Some(output);
        self
    }

    /// Set default options.
    pub fn with_defaults(mut self, defaults: CaptureDefaults) -> Self {
        self.defaults = Some(defaults);
        self
    }

    /// Run the request asynchronously.
    pub fn async_mode(mut self, is_async: bool) -> Self {
        self.is_async = Some(is_async);
        self
    }
}

/// Individual capture configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CaptureItem {
    /// Target URL
    pub url: String,
    /// Custom ID for this capture
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Label for this capture
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Viewport configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewport: Option<ViewportConfig>,
    /// Device preset name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    /// Capture full page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_page: Option<bool>,
    /// Enable dark mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark_mode: Option<bool>,
    /// Delay before capture in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<i32>,
}

impl CaptureItem {
    /// Create a simple capture for a URL.
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            id: None,
            label: None,
            viewport: None,
            device: None,
            full_page: None,
            dark_mode: None,
            delay: None,
        }
    }

    /// Set the device preset.
    pub fn with_device(mut self, device: impl Into<String>) -> Self {
        self.device = Some(device.into());
        self
    }

    /// Set a label for this capture.
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}

/// Variant configuration for the same URL.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VariantConfig {
    /// Custom ID for this variant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Label for this variant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Viewport configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewport: Option<ViewportConfig>,
    /// Device preset name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    /// Capture full page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_page: Option<bool>,
    /// Enable dark mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark_mode: Option<bool>,
    /// Delay before capture in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<i32>,
    /// Custom CSS to inject
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_css: Option<String>,
}

/// Default options for captures.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CaptureDefaults {
    /// Viewport configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewport: Option<ViewportConfig>,
    /// Device preset name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    /// Output image format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
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
    pub wait_until: Option<String>,
    /// Timeout in milliseconds
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
    /// Block ads
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_ads: Option<bool>,
    /// Block cookie banners
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_cookie_banners: Option<bool>,
    /// Blocking level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_level: Option<String>,
}

/// Output configuration for composed images.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComposeOutputConfig {
    /// Layout type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<LayoutType>,
    /// Output format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ImageFormat>,
    /// Image quality (1-100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<i32>,
    /// Number of columns (1-10)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<i32>,
    /// Spacing between images (0-100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spacing: Option<i32>,
    /// Padding around the canvas (0-100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<i32>,
    /// Background color (#RRGGBB, #RRGGBBAA, or "transparent")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
    /// Vertical alignment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<Alignment>,
    /// Maximum width (100-10000)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_width: Option<i32>,
    /// Maximum height (100-10000)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_height: Option<i32>,
    /// Thumbnail width (50-2000)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_width: Option<i32>,
    /// Label configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<LabelConfig>,
    /// Border configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border: Option<BorderConfig>,
    /// Shadow configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow: Option<ShadowConfig>,
}

/// Label configuration for composed images.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LabelConfig {
    /// Enable labels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Font size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<i32>,
    /// Font color
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// Label position
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// Border configuration for composed images.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BorderConfig {
    /// Enable border
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Border width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    /// Border color
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// Border radius
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius: Option<i32>,
}

/// Shadow configuration for composed images.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShadowConfig {
    /// Enable shadow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Shadow blur
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blur: Option<i32>,
    /// Shadow color
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// Shadow offset X
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_x: Option<i32>,
    /// Shadow offset Y
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_y: Option<i32>,
}

/// Response for synchronous compose request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComposeResponse {
    /// URL to the composed image
    pub url: Option<String>,
    /// Storage URL
    pub storage_url: Option<String>,
    /// Expiration timestamp
    pub expires_at: Option<String>,
    /// Image width
    pub width: Option<i32>,
    /// Image height
    pub height: Option<i32>,
    /// Output format
    pub format: Option<String>,
    /// File size in bytes
    pub file_size: Option<i64>,
    /// Render time in milliseconds
    pub render_time_ms: Option<i64>,
    /// Layout used
    pub layout: Option<String>,
    /// Additional metadata
    pub metadata: Option<ComposeMetadata>,
}

/// Metadata for compose response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComposeMetadata {
    /// Number of captures
    pub capture_count: Option<i32>,
    /// Layout type used
    pub layout_type: Option<String>,
}

/// Status response for async compose job.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComposeJobStatusResponse {
    /// Job ID
    pub job_id: String,
    /// Current status
    pub status: String,
    /// Progress percentage (0-100)
    pub progress: Option<i32>,
    /// Total number of captures
    pub total_captures: Option<i32>,
    /// Number of completed captures
    pub completed_captures: Option<i32>,
    /// Result when completed
    pub result: Option<ComposeResponse>,
    /// Error code if failed
    pub error_code: Option<String>,
    /// Error message if failed
    pub error_message: Option<String>,
    /// Creation timestamp
    pub created_at: Option<String>,
    /// Completion timestamp
    pub completed_at: Option<String>,
}

/// Summary of a compose job.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComposeJobSummaryResponse {
    /// Job ID
    pub job_id: String,
    /// Current status
    pub status: String,
    /// Total number of captures
    pub total_captures: Option<i32>,
    /// Number of completed captures
    pub completed_captures: Option<i32>,
    /// Number of failed captures
    pub failed_captures: Option<i32>,
    /// Progress percentage (0-100)
    pub progress: Option<i32>,
    /// Layout type
    pub layout_type: Option<String>,
    /// Creation timestamp
    pub created_at: Option<String>,
    /// Completion timestamp
    pub completed_at: Option<String>,
}

/// Layout preview response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutPreviewResponse {
    /// Layout type
    pub layout: String,
    /// Resolved layout type
    pub resolved_layout: Option<String>,
    /// Canvas width
    pub canvas_width: i32,
    /// Canvas height
    pub canvas_height: i32,
    /// Placement positions
    pub placements: Vec<PlacementPreview>,
    /// Additional metadata
    pub metadata: Option<serde_json::Value>,
}

/// Preview of image placement.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlacementPreview {
    /// Index of the image
    pub index: i32,
    /// X position
    pub x: i32,
    /// Y position
    pub y: i32,
    /// Width
    pub width: i32,
    /// Height
    pub height: i32,
    /// Label
    pub label: Option<String>,
}
