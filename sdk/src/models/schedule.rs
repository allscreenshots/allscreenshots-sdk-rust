//! Schedule-related request and response models.

use serde::{Deserialize, Serialize};
use super::common::*;

/// Request to create a scheduled screenshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateScheduleRequest {
    /// Schedule name (required, max 255 chars)
    pub name: String,
    /// Target URL (required)
    pub url: String,
    /// Cron expression (required)
    pub schedule: String,
    /// Timezone (e.g., "America/New_York")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// Screenshot options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<ScheduleScreenshotOptions>,
    /// Webhook URL for notifications
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
    /// Secret for webhook signature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_secret: Option<String>,
    /// Retention period in days (1-365)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i32>,
    /// Start date for the schedule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starts_at: Option<String>,
    /// End date for the schedule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ends_at: Option<String>,
}

impl CreateScheduleRequest {
    /// Create a new schedule request.
    pub fn new(name: impl Into<String>, url: impl Into<String>, schedule: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            url: url.into(),
            schedule: schedule.into(),
            timezone: None,
            options: None,
            webhook_url: None,
            webhook_secret: None,
            retention_days: None,
            starts_at: None,
            ends_at: None,
        }
    }

    /// Set the timezone.
    pub fn with_timezone(mut self, timezone: impl Into<String>) -> Self {
        self.timezone = Some(timezone.into());
        self
    }

    /// Set screenshot options.
    pub fn with_options(mut self, options: ScheduleScreenshotOptions) -> Self {
        self.options = Some(options);
        self
    }

    /// Set the retention period.
    pub fn with_retention_days(mut self, days: i32) -> Self {
        self.retention_days = Some(days);
        self
    }
}

/// Request to update a schedule.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateScheduleRequest {
    /// Schedule name (max 255 chars)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Target URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Cron expression
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    /// Timezone
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// Screenshot options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<ScheduleScreenshotOptions>,
    /// Webhook URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
    /// Webhook secret
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_secret: Option<String>,
    /// Retention period in days (1-365)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i32>,
    /// Start date for the schedule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starts_at: Option<String>,
    /// End date for the schedule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ends_at: Option<String>,
}

/// Screenshot options for scheduled captures.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleScreenshotOptions {
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
    pub block_level: Option<BlockLevel>,
}

/// Response for a schedule.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleResponse {
    /// Schedule ID
    pub id: String,
    /// Schedule name
    pub name: String,
    /// Target URL
    pub url: String,
    /// Cron expression
    pub schedule: String,
    /// Human-readable schedule description
    pub schedule_description: Option<String>,
    /// Timezone
    pub timezone: Option<String>,
    /// Current status
    pub status: String,
    /// Screenshot options
    pub options: Option<serde_json::Value>,
    /// Webhook URL
    pub webhook_url: Option<String>,
    /// Retention period in days
    pub retention_days: Option<i32>,
    /// Start date
    pub starts_at: Option<String>,
    /// End date
    pub ends_at: Option<String>,
    /// Last execution timestamp
    pub last_executed_at: Option<String>,
    /// Next execution timestamp
    pub next_execution_at: Option<String>,
    /// Total execution count
    pub execution_count: Option<i32>,
    /// Successful execution count
    pub success_count: Option<i32>,
    /// Failed execution count
    pub failure_count: Option<i32>,
    /// Creation timestamp
    pub created_at: Option<String>,
    /// Last update timestamp
    pub updated_at: Option<String>,
}

/// Response for listing schedules.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleListResponse {
    /// List of schedules
    pub schedules: Vec<ScheduleResponse>,
    /// Total count
    pub total: i32,
}

/// Response for schedule execution history.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleHistoryResponse {
    /// Schedule ID
    pub schedule_id: String,
    /// Total number of executions
    pub total_executions: i64,
    /// List of executions
    pub executions: Vec<ScheduleExecutionResponse>,
}

/// Response for a single schedule execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleExecutionResponse {
    /// Execution ID
    pub id: String,
    /// Execution timestamp
    pub executed_at: String,
    /// Execution status
    pub status: String,
    /// Result URL
    pub result_url: Option<String>,
    /// Storage URL
    pub storage_url: Option<String>,
    /// File size in bytes
    pub file_size: Option<i64>,
    /// Render time in milliseconds
    pub render_time_ms: Option<i64>,
    /// Error code if failed
    pub error_code: Option<String>,
    /// Error message if failed
    pub error_message: Option<String>,
    /// Expiration timestamp
    pub expires_at: Option<String>,
}
