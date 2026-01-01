//! Usage and quota-related response models.

use serde::{Deserialize, Serialize};

/// Response for usage statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageResponse {
    /// Account tier
    pub tier: String,
    /// Current period usage
    pub current_period: PeriodUsageResponse,
    /// Quota information
    pub quota: Option<QuotaResponse>,
    /// Historical usage
    pub history: Option<Vec<PeriodUsageResponse>>,
    /// Total usage across all time
    pub totals: Option<TotalsResponse>,
}

/// Response for quota status.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuotaStatusResponse {
    /// Account tier
    pub tier: String,
    /// Screenshot quota details
    pub screenshots: QuotaDetailResponse,
    /// Bandwidth quota details
    pub bandwidth: BandwidthQuotaResponse,
    /// Period end date
    pub period_ends: Option<String>,
}

/// Detailed quota information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuotaDetailResponse {
    /// Quota limit
    pub limit: i32,
    /// Used amount
    pub used: i32,
    /// Remaining amount
    pub remaining: i32,
    /// Percentage used
    pub percent_used: i32,
}

/// Bandwidth quota information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BandwidthQuotaResponse {
    /// Limit in bytes
    pub limit_bytes: i64,
    /// Limit formatted as human-readable string
    pub limit_formatted: String,
    /// Used in bytes
    pub used_bytes: i64,
    /// Used formatted as human-readable string
    pub used_formatted: String,
    /// Remaining in bytes
    pub remaining_bytes: i64,
    /// Remaining formatted as human-readable string
    pub remaining_formatted: String,
    /// Percentage used
    pub percent_used: i32,
}

/// Usage for a single period.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PeriodUsageResponse {
    /// Period start date
    pub period_start: String,
    /// Period end date
    pub period_end: String,
    /// Number of screenshots taken
    pub screenshots_count: i32,
    /// Bandwidth used in bytes
    pub bandwidth_bytes: i64,
    /// Bandwidth formatted as human-readable string
    pub bandwidth_formatted: String,
}

/// Total usage information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TotalsResponse {
    /// Total screenshots taken
    pub screenshots_count: i64,
    /// Total bandwidth used in bytes
    pub bandwidth_bytes: i64,
    /// Total bandwidth formatted as human-readable string
    pub bandwidth_formatted: String,
}

/// Quota information for usage response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuotaResponse {
    /// Monthly screenshot limit
    pub monthly_limit: i32,
    /// Monthly bandwidth limit in bytes
    pub monthly_bandwidth_bytes: Option<i64>,
    /// Monthly bandwidth limit formatted
    pub monthly_bandwidth_formatted: Option<String>,
}
