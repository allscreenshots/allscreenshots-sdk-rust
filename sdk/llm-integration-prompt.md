# Allscreenshots Rust SDK - LLM integration prompt

Use this prompt to help LLMs understand and use the Allscreenshots Rust SDK.

---

## SDK overview

The `allscreenshots-sdk` crate provides a Rust client for the Allscreenshots API, enabling programmatic website screenshot capture.

## Installation

```toml
[dependencies]
allscreenshots-sdk = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Authentication

Set the `ALLSCREENSHOTS_API_KEY` environment variable, or pass it directly:

```rust
// From environment
let client = AllscreenshotsClient::from_env()?;

// Direct API key
let client = AllscreenshotsClient::new("your-api-key")?;

// Builder pattern
let client = AllscreenshotsClient::builder()
    .api_key("your-api-key")
    .timeout(Duration::from_secs(120))
    .max_retries(5)
    .build()?;
```

## Common operations

### Take a screenshot

```rust
use allscreenshots_sdk::{AllscreenshotsClient, ScreenshotRequest, ImageFormat};

let client = AllscreenshotsClient::from_env()?;

let request = ScreenshotRequest::builder()
    .url("https://example.com")
    .device("Desktop HD")        // Optional: "iPhone 14", "iPad", etc.
    .full_page(true)             // Optional: capture full scrollable page
    .format(ImageFormat::Png)    // Optional: Png, Jpeg, Webp, Pdf
    .quality(90)                 // Optional: 1-100 for JPEG/WebP
    .dark_mode(true)             // Optional: enable dark mode
    .delay(1000)                 // Optional: wait 1s before capture
    .build()?;

let image_bytes = client.screenshot(&request).await?;
std::fs::write("screenshot.png", &image_bytes)?;
```

### Async screenshot with polling

```rust
let job = client.screenshot_async(&request).await?;

loop {
    let status = client.get_job(&job.id).await?;
    if status.status.is_terminal() {
        if status.status.is_success() {
            let image = client.get_job_result(&job.id).await?;
            std::fs::write("screenshot.png", &image)?;
        }
        break;
    }
    tokio::time::sleep(Duration::from_secs(1)).await;
}
```

### Bulk screenshots

```rust
use allscreenshots_sdk::{BulkRequest, BulkUrlRequest};

let request = BulkRequest::new(vec![
    BulkUrlRequest::new("https://github.com"),
    BulkUrlRequest::new("https://google.com"),
]);

let bulk_job = client.create_bulk_job(&request).await?;
let status = client.get_bulk_job(&bulk_job.id).await?;
```

### Compose multiple screenshots

```rust
use allscreenshots_sdk::{ComposeRequest, CaptureItem, ComposeOutputConfig, LayoutType};

let request = ComposeRequest::with_captures(vec![
    CaptureItem::new("https://example.com").with_device("Desktop HD"),
    CaptureItem::new("https://example.com").with_device("iPhone 14"),
]).with_output(ComposeOutputConfig {
    layout: Some(LayoutType::Horizontal),
    spacing: Some(20),
    ..Default::default()
});

let result = client.compose(&request).await?;
```

### Scheduled screenshots

```rust
use allscreenshots_sdk::CreateScheduleRequest;

let request = CreateScheduleRequest::new(
    "Daily capture",
    "https://example.com",
    "0 9 * * *",  // Cron: every day at 9 AM
).with_timezone("America/New_York");

let schedule = client.create_schedule(&request).await?;
```

### Check usage/quota

```rust
let usage = client.get_usage().await?;
let quota = client.get_quota().await?;
println!("Screenshots remaining: {}", quota.screenshots.remaining);
```

## Key types

- `AllscreenshotsClient` - Main API client
- `ScreenshotRequest` - Screenshot request configuration (use builder pattern)
- `JobResponse` - Async job status
- `JobStatus` - Enum: Queued, Processing, Completed, Failed, Cancelled
- `BulkRequest` / `BulkUrlRequest` - Bulk screenshot requests
- `ComposeRequest` / `CaptureItem` - Multi-screenshot composition
- `CreateScheduleRequest` - Scheduled screenshot configuration
- `ImageFormat` - Enum: Png, Jpeg, Webp, Pdf
- `AllscreenshotsError` - Error type with variants for API errors, validation, etc.

## Error handling

```rust
use allscreenshots_sdk::{AllscreenshotsError, ErrorCode};

match client.screenshot(&request).await {
    Ok(image) => { /* success */ }
    Err(AllscreenshotsError::ApiError { code, message, status }) => {
        match code {
            ErrorCode::RateLimitExceeded => { /* retry later */ }
            ErrorCode::Unauthorized => { /* invalid API key */ }
            ErrorCode::ValidationError => { /* invalid request */ }
            _ => { /* other error */ }
        }
    }
    Err(AllscreenshotsError::ValidationError(msg)) => { /* invalid input */ }
    Err(AllscreenshotsError::Timeout) => { /* request timed out */ }
    Err(e) => { /* other error */ }
}
```

## Device presets

Common presets: `"Desktop HD"` (1920x1080), `"Desktop"` (1440x900), `"Laptop"` (1366x768), `"iPhone 14"` (390x844), `"iPhone 14 Pro Max"` (430x932), `"iPad"` (820x1180), `"iPad Pro"` (1024x1366)
