//! HTTP client for the Allscreenshots API.

use crate::error::{AllscreenshotsError, ApiErrorResponse};
use crate::models::*;
use crate::retry::{with_retry, RetryConfig};
use reqwest::{Client, Response, StatusCode};
use std::env;
use std::time::Duration;

const DEFAULT_BASE_URL: &str = "https://api.allscreenshots.com";
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(60);
const API_KEY_ENV_VAR: &str = "ALLSCREENSHOTS_API_KEY";
const API_KEY_HEADER: &str = "X-API-Key";

/// Client for interacting with the Allscreenshots API.
///
/// # Example
///
/// ```rust,no_run
/// use allscreenshots_sdk::{AllscreenshotsClient, ScreenshotRequest};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = AllscreenshotsClient::from_env()?;
///
///     let request = ScreenshotRequest::builder()
///         .url("https://github.com")
///         .device("Desktop HD")
///         .build()?;
///
///     let image_bytes = client.screenshot(&request).await?;
///     std::fs::write("screenshot.png", &image_bytes)?;
///
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct AllscreenshotsClient {
    http_client: Client,
    base_url: String,
    api_key: String,
    retry_config: RetryConfig,
}

impl AllscreenshotsClient {
    /// Create a new client builder.
    pub fn builder() -> AllscreenshotsClientBuilder {
        AllscreenshotsClientBuilder::default()
    }

    /// Create a client using the API key from the environment variable.
    ///
    /// Reads the API key from `ALLSCREENSHOTS_API_KEY`.
    pub fn from_env() -> Result<Self, AllscreenshotsError> {
        Self::builder().build()
    }

    /// Create a client with the given API key.
    pub fn new(api_key: impl Into<String>) -> Result<Self, AllscreenshotsError> {
        Self::builder().api_key(api_key).build()
    }

    // =========================================================================
    // Screenshot endpoints
    // =========================================================================

    /// Take a screenshot synchronously.
    ///
    /// Returns the raw image bytes.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use allscreenshots_sdk::{AllscreenshotsClient, ScreenshotRequest};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AllscreenshotsClient::from_env()?;
    ///
    /// let request = ScreenshotRequest::builder()
    ///     .url("https://github.com")
    ///     .device("Desktop HD")
    ///     .full_page(true)
    ///     .build()?;
    ///
    /// let image_bytes = client.screenshot(&request).await?;
    /// std::fs::write("screenshot.png", &image_bytes)?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn screenshot(&self, request: &ScreenshotRequest) -> Result<Vec<u8>, AllscreenshotsError> {
        let url = format!("{}/v1/screenshots", self.base_url);

        let retry_config = self.retry_config.clone();
        with_retry(&retry_config, || async {
            let response = self
                .http_client
                .post(&url)
                .header(API_KEY_HEADER, &self.api_key)
                .json(request)
                .send()
                .await?;

            self.handle_binary_response(response).await
        })
        .await
    }

    /// Take a screenshot asynchronously.
    ///
    /// Returns job information that can be used to poll for results.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use allscreenshots_sdk::{AllscreenshotsClient, ScreenshotRequest};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AllscreenshotsClient::from_env()?;
    ///
    /// let request = ScreenshotRequest::builder()
    ///     .url("https://github.com")
    ///     .build()?;
    ///
    /// let job = client.screenshot_async(&request).await?;
    /// println!("Job created: {}", job.id);
    ///
    /// // Poll for results
    /// loop {
    ///     let status = client.get_job(&job.id).await?;
    ///     if status.status.is_terminal() {
    ///         if status.status.is_success() {
    ///             let image = client.get_job_result(&job.id).await?;
    ///             std::fs::write("screenshot.png", &image)?;
    ///         }
    ///         break;
    ///     }
    ///     tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn screenshot_async(
        &self,
        request: &ScreenshotRequest,
    ) -> Result<AsyncJobCreatedResponse, AllscreenshotsError> {
        let url = format!("{}/v1/screenshots/async", self.base_url);
        self.post_json(&url, request).await
    }

    /// List all screenshot jobs.
    pub async fn list_jobs(&self) -> Result<Vec<JobResponse>, AllscreenshotsError> {
        let url = format!("{}/v1/screenshots/jobs", self.base_url);
        self.get_json(&url).await
    }

    /// Get the status of a screenshot job.
    pub async fn get_job(&self, job_id: &str) -> Result<JobResponse, AllscreenshotsError> {
        let url = format!("{}/v1/screenshots/jobs/{}", self.base_url, job_id);
        self.get_json(&url).await
    }

    /// Get the result image of a completed job.
    pub async fn get_job_result(&self, job_id: &str) -> Result<Vec<u8>, AllscreenshotsError> {
        let url = format!("{}/v1/screenshots/jobs/{}/result", self.base_url, job_id);

        let retry_config = self.retry_config.clone();
        with_retry(&retry_config, || async {
            let response = self
                .http_client
                .get(&url)
                .header(API_KEY_HEADER, &self.api_key)
                .send()
                .await?;

            self.handle_binary_response(response).await
        })
        .await
    }

    /// Cancel a screenshot job.
    pub async fn cancel_job(&self, job_id: &str) -> Result<JobResponse, AllscreenshotsError> {
        let url = format!("{}/v1/screenshots/jobs/{}/cancel", self.base_url, job_id);
        self.post_empty(&url).await
    }

    // =========================================================================
    // Bulk screenshot endpoints
    // =========================================================================

    /// Create a bulk screenshot job.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use allscreenshots_sdk::{AllscreenshotsClient, BulkRequest, BulkUrlRequest};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AllscreenshotsClient::from_env()?;
    ///
    /// let request = BulkRequest::new(vec![
    ///     BulkUrlRequest::new("https://github.com"),
    ///     BulkUrlRequest::new("https://google.com"),
    ///     BulkUrlRequest::new("https://rust-lang.org"),
    /// ]);
    ///
    /// let bulk_job = client.create_bulk_job(&request).await?;
    /// println!("Bulk job created: {}", bulk_job.id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_bulk_job(&self, request: &BulkRequest) -> Result<BulkResponse, AllscreenshotsError> {
        let url = format!("{}/v1/screenshots/bulk", self.base_url);
        self.post_json(&url, request).await
    }

    /// List all bulk screenshot jobs.
    pub async fn list_bulk_jobs(&self) -> Result<Vec<BulkJobSummary>, AllscreenshotsError> {
        let url = format!("{}/v1/screenshots/bulk", self.base_url);
        self.get_json(&url).await
    }

    /// Get the status of a bulk screenshot job.
    pub async fn get_bulk_job(&self, job_id: &str) -> Result<BulkStatusResponse, AllscreenshotsError> {
        let url = format!("{}/v1/screenshots/bulk/{}", self.base_url, job_id);
        self.get_json(&url).await
    }

    /// Cancel a bulk screenshot job.
    pub async fn cancel_bulk_job(&self, job_id: &str) -> Result<BulkJobSummary, AllscreenshotsError> {
        let url = format!("{}/v1/screenshots/bulk/{}/cancel", self.base_url, job_id);
        self.post_empty(&url).await
    }

    // =========================================================================
    // Compose endpoints
    // =========================================================================

    /// Compose multiple screenshots into one image.
    ///
    /// Returns either a `ComposeResponse` (sync) or `ComposeJobStatusResponse` (async).
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use allscreenshots_sdk::{AllscreenshotsClient, ComposeRequest, CaptureItem, ComposeOutputConfig, LayoutType};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AllscreenshotsClient::from_env()?;
    ///
    /// let request = ComposeRequest::with_captures(vec![
    ///     CaptureItem::new("https://github.com").with_device("Desktop HD"),
    ///     CaptureItem::new("https://github.com").with_device("iPhone 14"),
    /// ]).with_output(ComposeOutputConfig {
    ///     layout: Some(LayoutType::Horizontal),
    ///     ..Default::default()
    /// });
    ///
    /// let result = client.compose(&request).await?;
    /// println!("Composed image: {:?}", result.url);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn compose(&self, request: &ComposeRequest) -> Result<ComposeResponse, AllscreenshotsError> {
        let url = format!("{}/v1/screenshots/compose", self.base_url);
        self.post_json(&url, request).await
    }

    /// Compose multiple screenshots asynchronously.
    pub async fn compose_async(
        &self,
        request: &ComposeRequest,
    ) -> Result<ComposeJobStatusResponse, AllscreenshotsError> {
        let mut req = request.clone();
        req.is_async = Some(true);
        let url = format!("{}/v1/screenshots/compose", self.base_url);
        self.post_json(&url, &req).await
    }

    /// Preview layout placement.
    pub async fn preview_layout(
        &self,
        layout: &str,
        image_count: i32,
        canvas_width: Option<i32>,
        canvas_height: Option<i32>,
        aspect_ratios: Option<&str>,
    ) -> Result<LayoutPreviewResponse, AllscreenshotsError> {
        let mut url = format!(
            "{}/v1/screenshots/compose/preview?layout={}&image_count={}",
            self.base_url, layout, image_count
        );
        if let Some(w) = canvas_width {
            url.push_str(&format!("&canvas_width={}", w));
        }
        if let Some(h) = canvas_height {
            url.push_str(&format!("&canvas_height={}", h));
        }
        if let Some(ar) = aspect_ratios {
            url.push_str(&format!("&aspect_ratios={}", ar));
        }
        self.get_json(&url).await
    }

    /// List all compose jobs.
    pub async fn list_compose_jobs(&self) -> Result<Vec<ComposeJobSummaryResponse>, AllscreenshotsError> {
        let url = format!("{}/v1/screenshots/compose/jobs", self.base_url);
        self.get_json(&url).await
    }

    /// Get the status of a compose job.
    pub async fn get_compose_job(&self, job_id: &str) -> Result<ComposeJobStatusResponse, AllscreenshotsError> {
        let url = format!("{}/v1/screenshots/compose/jobs/{}", self.base_url, job_id);
        self.get_json(&url).await
    }

    // =========================================================================
    // Schedule endpoints
    // =========================================================================

    /// Create a scheduled screenshot.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use allscreenshots_sdk::{AllscreenshotsClient, CreateScheduleRequest};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AllscreenshotsClient::from_env()?;
    ///
    /// let request = CreateScheduleRequest::new(
    ///     "Daily GitHub capture",
    ///     "https://github.com",
    ///     "0 9 * * *", // Every day at 9 AM
    /// ).with_timezone("America/New_York");
    ///
    /// let schedule = client.create_schedule(&request).await?;
    /// println!("Schedule created: {}", schedule.id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_schedule(
        &self,
        request: &CreateScheduleRequest,
    ) -> Result<ScheduleResponse, AllscreenshotsError> {
        let url = format!("{}/v1/schedules", self.base_url);
        self.post_json(&url, request).await
    }

    /// List all schedules.
    pub async fn list_schedules(&self) -> Result<ScheduleListResponse, AllscreenshotsError> {
        let url = format!("{}/v1/schedules", self.base_url);
        self.get_json(&url).await
    }

    /// Get a schedule by ID.
    pub async fn get_schedule(&self, schedule_id: &str) -> Result<ScheduleResponse, AllscreenshotsError> {
        let url = format!("{}/v1/schedules/{}", self.base_url, schedule_id);
        self.get_json(&url).await
    }

    /// Update a schedule.
    pub async fn update_schedule(
        &self,
        schedule_id: &str,
        request: &UpdateScheduleRequest,
    ) -> Result<ScheduleResponse, AllscreenshotsError> {
        let url = format!("{}/v1/schedules/{}", self.base_url, schedule_id);
        self.put_json(&url, request).await
    }

    /// Delete a schedule.
    pub async fn delete_schedule(&self, schedule_id: &str) -> Result<(), AllscreenshotsError> {
        let url = format!("{}/v1/schedules/{}", self.base_url, schedule_id);
        self.delete(&url).await
    }

    /// Pause a schedule.
    pub async fn pause_schedule(&self, schedule_id: &str) -> Result<ScheduleResponse, AllscreenshotsError> {
        let url = format!("{}/v1/schedules/{}/pause", self.base_url, schedule_id);
        self.post_empty(&url).await
    }

    /// Resume a schedule.
    pub async fn resume_schedule(&self, schedule_id: &str) -> Result<ScheduleResponse, AllscreenshotsError> {
        let url = format!("{}/v1/schedules/{}/resume", self.base_url, schedule_id);
        self.post_empty(&url).await
    }

    /// Manually trigger a schedule.
    pub async fn trigger_schedule(&self, schedule_id: &str) -> Result<ScheduleResponse, AllscreenshotsError> {
        let url = format!("{}/v1/schedules/{}/trigger", self.base_url, schedule_id);
        self.post_empty(&url).await
    }

    /// Get the execution history of a schedule.
    pub async fn get_schedule_history(
        &self,
        schedule_id: &str,
        limit: Option<i32>,
    ) -> Result<ScheduleHistoryResponse, AllscreenshotsError> {
        let mut url = format!("{}/v1/schedules/{}/history", self.base_url, schedule_id);
        if let Some(l) = limit {
            url.push_str(&format!("?limit={}", l));
        }
        self.get_json(&url).await
    }

    // =========================================================================
    // Usage endpoints
    // =========================================================================

    /// Get usage statistics.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use allscreenshots_sdk::AllscreenshotsClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AllscreenshotsClient::from_env()?;
    ///
    /// let usage = client.get_usage().await?;
    /// println!("Tier: {}", usage.tier);
    /// println!("Screenshots this period: {}", usage.current_period.screenshots_count);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_usage(&self) -> Result<UsageResponse, AllscreenshotsError> {
        let url = format!("{}/v1/usage", self.base_url);
        self.get_json(&url).await
    }

    /// Get quota status.
    pub async fn get_quota(&self) -> Result<QuotaStatusResponse, AllscreenshotsError> {
        let url = format!("{}/v1/usage/quota", self.base_url);
        self.get_json(&url).await
    }

    // =========================================================================
    // Helper methods
    // =========================================================================

    async fn get_json<T: serde::de::DeserializeOwned>(&self, url: &str) -> Result<T, AllscreenshotsError> {
        let retry_config = self.retry_config.clone();
        let url = url.to_string();

        with_retry(&retry_config, || {
            let url = url.clone();
            async move {
                let response = self
                    .http_client
                    .get(&url)
                    .header(API_KEY_HEADER, &self.api_key)
                    .send()
                    .await?;

                self.handle_json_response(response).await
            }
        })
        .await
    }

    async fn post_json<T: serde::de::DeserializeOwned, B: serde::Serialize>(
        &self,
        url: &str,
        body: &B,
    ) -> Result<T, AllscreenshotsError> {
        let retry_config = self.retry_config.clone();
        let url = url.to_string();
        let body_json = serde_json::to_string(body)?;

        with_retry(&retry_config, || {
            let url = url.clone();
            let body_json = body_json.clone();
            async move {
                let response = self
                    .http_client
                    .post(&url)
                    .header(API_KEY_HEADER, &self.api_key)
                    .header("Content-Type", "application/json")
                    .body(body_json)
                    .send()
                    .await?;

                self.handle_json_response(response).await
            }
        })
        .await
    }

    async fn post_empty<T: serde::de::DeserializeOwned>(&self, url: &str) -> Result<T, AllscreenshotsError> {
        let retry_config = self.retry_config.clone();
        let url = url.to_string();

        with_retry(&retry_config, || {
            let url = url.clone();
            async move {
                let response = self
                    .http_client
                    .post(&url)
                    .header(API_KEY_HEADER, &self.api_key)
                    .send()
                    .await?;

                self.handle_json_response(response).await
            }
        })
        .await
    }

    async fn put_json<T: serde::de::DeserializeOwned, B: serde::Serialize>(
        &self,
        url: &str,
        body: &B,
    ) -> Result<T, AllscreenshotsError> {
        let retry_config = self.retry_config.clone();
        let url = url.to_string();
        let body_json = serde_json::to_string(body)?;

        with_retry(&retry_config, || {
            let url = url.clone();
            let body_json = body_json.clone();
            async move {
                let response = self
                    .http_client
                    .put(&url)
                    .header(API_KEY_HEADER, &self.api_key)
                    .header("Content-Type", "application/json")
                    .body(body_json)
                    .send()
                    .await?;

                self.handle_json_response(response).await
            }
        })
        .await
    }

    async fn delete(&self, url: &str) -> Result<(), AllscreenshotsError> {
        let retry_config = self.retry_config.clone();
        let url = url.to_string();

        with_retry(&retry_config, || {
            let url = url.clone();
            async move {
                let response = self
                    .http_client
                    .delete(&url)
                    .header(API_KEY_HEADER, &self.api_key)
                    .send()
                    .await?;

                let status = response.status();
                if status.is_success() {
                    Ok(())
                } else {
                    Err(self.parse_error_response(response).await)
                }
            }
        })
        .await
    }

    async fn handle_json_response<T: serde::de::DeserializeOwned>(
        &self,
        response: Response,
    ) -> Result<T, AllscreenshotsError> {
        let status = response.status();

        if status.is_success() {
            let body = response.text().await?;
            serde_json::from_str(&body).map_err(AllscreenshotsError::from)
        } else {
            Err(self.parse_error_response(response).await)
        }
    }

    async fn handle_binary_response(&self, response: Response) -> Result<Vec<u8>, AllscreenshotsError> {
        let status = response.status();

        if status.is_success() {
            response.bytes().await.map(|b| b.to_vec()).map_err(AllscreenshotsError::from)
        } else {
            Err(self.parse_error_response(response).await)
        }
    }

    async fn parse_error_response(&self, response: Response) -> AllscreenshotsError {
        let status = response.status().as_u16();

        match response.text().await {
            Ok(body) => {
                if let Ok(error_response) = serde_json::from_str::<ApiErrorResponse>(&body) {
                    AllscreenshotsError::from_api_response(
                        status,
                        error_response.error_code.as_deref(),
                        &error_response.get_message(),
                    )
                } else {
                    AllscreenshotsError::from_api_response(
                        status,
                        None,
                        &format!("HTTP {} error", status),
                    )
                }
            }
            Err(_) => AllscreenshotsError::from_api_response(
                status,
                None,
                &format!("HTTP {} error", status),
            ),
        }
    }
}

/// Builder for creating an AllscreenshotsClient.
#[derive(Debug, Default)]
pub struct AllscreenshotsClientBuilder {
    api_key: Option<String>,
    base_url: Option<String>,
    timeout: Option<Duration>,
    max_retries: Option<u32>,
}

impl AllscreenshotsClientBuilder {
    /// Set the API key.
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Set the base URL for the API.
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// Set the request timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Set the maximum number of retries.
    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = Some(max_retries);
        self
    }

    /// Build the client.
    pub fn build(self) -> Result<AllscreenshotsClient, AllscreenshotsError> {
        let api_key = match self.api_key {
            Some(key) => key,
            None => env::var(API_KEY_ENV_VAR)
                .map_err(|_| AllscreenshotsError::EnvVarNotSet(API_KEY_ENV_VAR.to_string()))?,
        };

        if api_key.is_empty() {
            return Err(AllscreenshotsError::ConfigError(
                "API key cannot be empty".to_string(),
            ));
        }

        let base_url = self.base_url.unwrap_or_else(|| DEFAULT_BASE_URL.to_string());
        let timeout = self.timeout.unwrap_or(DEFAULT_TIMEOUT);

        let http_client = Client::builder()
            .timeout(timeout)
            .build()
            .map_err(|e| AllscreenshotsError::ConfigError(format!("Failed to create HTTP client: {}", e)))?;

        let mut retry_config = RetryConfig::default();
        if let Some(max_retries) = self.max_retries {
            retry_config.max_retries = max_retries;
        }

        Ok(AllscreenshotsClient {
            http_client,
            base_url,
            api_key,
            retry_config,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_missing_api_key() {
        // Ensure env var is not set for this test
        env::remove_var(API_KEY_ENV_VAR);

        let result = AllscreenshotsClient::builder().build();
        assert!(result.is_err());
    }

    #[test]
    fn test_builder_empty_api_key() {
        let result = AllscreenshotsClient::builder()
            .api_key("")
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_builder_with_api_key() {
        let result = AllscreenshotsClient::builder()
            .api_key("test-api-key")
            .build();
        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_custom_base_url() {
        let client = AllscreenshotsClient::builder()
            .api_key("test-api-key")
            .base_url("https://custom.api.com")
            .build()
            .unwrap();

        assert_eq!(client.base_url, "https://custom.api.com");
    }

    #[test]
    fn test_builder_custom_timeout() {
        let result = AllscreenshotsClient::builder()
            .api_key("test-api-key")
            .timeout(Duration::from_secs(120))
            .build();
        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_custom_max_retries() {
        let client = AllscreenshotsClient::builder()
            .api_key("test-api-key")
            .max_retries(5)
            .build()
            .unwrap();

        assert_eq!(client.retry_config.max_retries, 5);
    }
}
