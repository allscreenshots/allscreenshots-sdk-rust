//! Common types used across multiple API endpoints.

use serde::{Deserialize, Serialize};

/// Viewport configuration for screenshots.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ViewportConfig {
    /// Width in pixels (100-4096)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    /// Height in pixels (100-4096)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    /// Device scale factor (1-3)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_scale_factor: Option<i32>,
}

impl ViewportConfig {
    /// Create a new viewport configuration.
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width: Some(width),
            height: Some(height),
            device_scale_factor: None,
        }
    }

    /// Set the device scale factor.
    pub fn with_scale_factor(mut self, factor: i32) -> Self {
        self.device_scale_factor = Some(factor);
        self
    }
}

/// Image format for screenshots.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ImageFormat {
    /// PNG format (lossless)
    #[default]
    Png,
    /// JPEG format
    Jpeg,
    /// JPEG format (alias)
    Jpg,
    /// WebP format
    Webp,
    /// PDF format
    Pdf,
}

/// Wait condition for page loading.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum WaitUntil {
    /// Wait for the load event
    #[default]
    Load,
    /// Wait for DOMContentLoaded
    #[serde(rename = "domcontentloaded")]
    DomContentLoaded,
    /// Wait for network to be idle
    #[serde(rename = "networkidle")]
    NetworkIdle,
    /// Wait for first commit
    Commit,
}

/// Block level for ads and trackers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum BlockLevel {
    /// No blocking
    #[default]
    None,
    /// Light blocking
    Light,
    /// Normal blocking
    Normal,
    /// Pro blocking
    Pro,
    /// Pro plus blocking
    ProPlus,
    /// Ultimate blocking
    Ultimate,
}

/// Job status for async operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum JobStatus {
    /// Job is queued
    Queued,
    /// Job is processing
    Processing,
    /// Job completed successfully
    Completed,
    /// Job failed
    Failed,
    /// Job was cancelled
    Cancelled,
}

impl JobStatus {
    /// Returns `true` if the job is in a terminal state.
    pub fn is_terminal(&self) -> bool {
        matches!(self, JobStatus::Completed | JobStatus::Failed | JobStatus::Cancelled)
    }

    /// Returns `true` if the job completed successfully.
    pub fn is_success(&self) -> bool {
        matches!(self, JobStatus::Completed)
    }
}

/// Response type for screenshot requests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ResponseType {
    /// Return binary image data
    #[default]
    Binary,
    /// Return JSON with image URL
    Json,
}
