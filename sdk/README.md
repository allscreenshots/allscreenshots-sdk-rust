# Allscreenshots SDK for Rust

Official Rust SDK for the [Allscreenshots](https://allscreenshots.com) API - capture website screenshots programmatically.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
allscreenshots-sdk = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick start

```rust
use allscreenshots_sdk::{AllscreenshotsClient, ScreenshotRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client (reads API key from ALLSCREENSHOTS_API_KEY env var)
    let client = AllscreenshotsClient::from_env()?;

    // Take a screenshot
    let request = ScreenshotRequest::builder()
        .url("https://github.com")
        .device("Desktop HD")
        .build()?;

    let image_bytes = client.screenshot(&request).await?;
    std::fs::write("screenshot.png", &image_bytes)?;

    println!("Screenshot saved to screenshot.png");
    Ok(())
}
```

## Configuration

### Environment variable

The SDK automatically reads the API key from the `ALLSCREENSHOTS_API_KEY` environment variable:

```bash
export ALLSCREENSHOTS_API_KEY=your-api-key
```

### Builder pattern

Configure the client with custom options:

```rust
use allscreenshots_sdk::AllscreenshotsClient;
use std::time::Duration;

let client = AllscreenshotsClient::builder()
    .api_key("your-api-key")
    .base_url("https://api.allscreenshots.com")
    .timeout(Duration::from_secs(120))
    .max_retries(5)
    .build()?;
```

## API reference

### Screenshot capture

#### Synchronous screenshot

```rust
use allscreenshots_sdk::{AllscreenshotsClient, ScreenshotRequest, ImageFormat};

let client = AllscreenshotsClient::from_env()?;

let request = ScreenshotRequest::builder()
    .url("https://example.com")
    .device("Desktop HD")
    .full_page(true)
    .format(ImageFormat::Png)
    .quality(90)
    .dark_mode(true)
    .build()?;

let image_bytes = client.screenshot(&request).await?;
std::fs::write("screenshot.png", &image_bytes)?;
```

#### Asynchronous screenshot

For long-running screenshots, use the async API:

```rust
use allscreenshots_sdk::{AllscreenshotsClient, ScreenshotRequest};
use std::time::Duration;

let client = AllscreenshotsClient::from_env()?;

let request = ScreenshotRequest::builder()
    .url("https://example.com")
    .full_page(true)
    .build()?;

// Start the job
let job = client.screenshot_async(&request).await?;
println!("Job created: {}", job.id);

// Poll for completion
loop {
    let status = client.get_job(&job.id).await?;

    if status.status.is_terminal() {
        if status.status.is_success() {
            let image = client.get_job_result(&job.id).await?;
            std::fs::write("screenshot.png", &image)?;
            println!("Screenshot saved!");
        } else {
            println!("Job failed: {:?}", status.error_message);
        }
        break;
    }

    tokio::time::sleep(Duration::from_secs(1)).await;
}
```

### Bulk screenshots

Capture multiple URLs in a single request:

```rust
use allscreenshots_sdk::{AllscreenshotsClient, BulkRequest, BulkUrlRequest, BulkDefaults};

let client = AllscreenshotsClient::from_env()?;

let request = BulkRequest::new(vec![
    BulkUrlRequest::new("https://github.com"),
    BulkUrlRequest::new("https://google.com"),
    BulkUrlRequest::new("https://rust-lang.org"),
]).with_defaults(BulkDefaults {
    device: Some("Desktop HD".to_string()),
    ..Default::default()
});

let bulk_job = client.create_bulk_job(&request).await?;
println!("Bulk job created: {}", bulk_job.id);

// Check status
let status = client.get_bulk_job(&bulk_job.id).await?;
println!("Progress: {}%", status.progress);
```

### Compose multiple screenshots

Combine multiple screenshots into a single image:

```rust
use allscreenshots_sdk::{
    AllscreenshotsClient, ComposeRequest, CaptureItem,
    ComposeOutputConfig, LayoutType
};

let client = AllscreenshotsClient::from_env()?;

let request = ComposeRequest::with_captures(vec![
    CaptureItem::new("https://github.com").with_device("Desktop HD"),
    CaptureItem::new("https://github.com").with_device("iPhone 14"),
    CaptureItem::new("https://github.com").with_device("iPad"),
]).with_output(ComposeOutputConfig {
    layout: Some(LayoutType::Horizontal),
    spacing: Some(20),
    background: Some("#ffffff".to_string()),
    ..Default::default()
});

let result = client.compose(&request).await?;
println!("Composed image URL: {:?}", result.url);
```

### Scheduled screenshots

Set up recurring screenshot captures:

```rust
use allscreenshots_sdk::{AllscreenshotsClient, CreateScheduleRequest, ScheduleScreenshotOptions};

let client = AllscreenshotsClient::from_env()?;

let request = CreateScheduleRequest::new(
    "Daily GitHub capture",
    "https://github.com",
    "0 9 * * *", // Every day at 9 AM
)
.with_timezone("America/New_York")
.with_retention_days(30)
.with_options(ScheduleScreenshotOptions {
    device: Some("Desktop HD".to_string()),
    full_page: Some(true),
    ..Default::default()
});

let schedule = client.create_schedule(&request).await?;
println!("Schedule created: {}", schedule.id);

// List all schedules
let schedules = client.list_schedules().await?;
for s in schedules.schedules {
    println!("  {} - {}", s.name, s.status);
}

// Pause a schedule
client.pause_schedule(&schedule.id).await?;

// Resume a schedule
client.resume_schedule(&schedule.id).await?;

// Get execution history
let history = client.get_schedule_history(&schedule.id, Some(10)).await?;
for execution in history.executions {
    println!("  {} - {}", execution.executed_at, execution.status);
}
```

### Usage and quota

Check your API usage:

```rust
use allscreenshots_sdk::AllscreenshotsClient;

let client = AllscreenshotsClient::from_env()?;

// Get usage statistics
let usage = client.get_usage().await?;
println!("Tier: {}", usage.tier);
println!("Screenshots this period: {}", usage.current_period.screenshots_count);
println!("Bandwidth used: {}", usage.current_period.bandwidth_formatted);

// Get quota status
let quota = client.get_quota().await?;
println!("Screenshots remaining: {}", quota.screenshots.remaining);
println!("Quota used: {}%", quota.screenshots.percent_used);
```

## Screenshot options

| Option | Type | Description |
|--------|------|-------------|
| `url` | `String` | Target URL to capture (required) |
| `device` | `String` | Device preset (e.g., "Desktop HD", "iPhone 14", "iPad") |
| `viewport` | `ViewportConfig` | Custom viewport dimensions |
| `format` | `ImageFormat` | Output format: Png, Jpeg, Webp, Pdf |
| `full_page` | `bool` | Capture the full scrollable page |
| `quality` | `i32` | Image quality (1-100, for JPEG/WebP) |
| `delay` | `i32` | Delay before capture in milliseconds (0-30000) |
| `wait_for` | `String` | CSS selector to wait for |
| `wait_until` | `WaitUntil` | Page load condition (Load, DomContentLoaded, NetworkIdle) |
| `timeout` | `i32` | Request timeout in milliseconds (1000-60000) |
| `dark_mode` | `bool` | Enable dark mode |
| `custom_css` | `String` | Custom CSS to inject |
| `hide_selectors` | `Vec<String>` | CSS selectors to hide |
| `selector` | `String` | Capture specific element |
| `block_ads` | `bool` | Block advertisements |
| `block_cookie_banners` | `bool` | Block cookie consent banners |
| `block_level` | `BlockLevel` | Ad/tracker blocking level |

## Device presets

Common device presets:

| Device | Viewport |
|--------|----------|
| Desktop HD | 1920x1080 |
| Desktop | 1440x900 |
| Laptop | 1366x768 |
| iPhone 14 | 390x844 |
| iPhone 14 Pro Max | 430x932 |
| iPad | 820x1180 |
| iPad Pro | 1024x1366 |

## Error handling

The SDK provides typed errors for robust error handling:

```rust
use allscreenshots_sdk::{AllscreenshotsClient, ScreenshotRequest, AllscreenshotsError, ErrorCode};

let client = AllscreenshotsClient::from_env()?;
let request = ScreenshotRequest::builder()
    .url("https://example.com")
    .build()?;

match client.screenshot(&request).await {
    Ok(image) => {
        std::fs::write("screenshot.png", &image)?;
    }
    Err(AllscreenshotsError::ApiError { code, message, status }) => {
        match code {
            ErrorCode::RateLimitExceeded => {
                println!("Rate limit exceeded, please wait and retry");
            }
            ErrorCode::Unauthorized => {
                println!("Invalid API key");
            }
            ErrorCode::ValidationError => {
                println!("Invalid request: {}", message);
            }
            _ => {
                println!("API error ({}): {}", status, message);
            }
        }
    }
    Err(AllscreenshotsError::ValidationError(msg)) => {
        println!("Request validation failed: {}", msg);
    }
    Err(AllscreenshotsError::Timeout) => {
        println!("Request timed out");
    }
    Err(e) => {
        println!("Error: {}", e);
    }
}
```

### Retryable errors

The SDK automatically retries transient errors with exponential backoff:

- Rate limit exceeded (429)
- Internal server errors (5xx)
- Network timeouts
- Connection errors

Configure retry behavior:

```rust
let client = AllscreenshotsClient::builder()
    .api_key("your-api-key")
    .max_retries(5)
    .build()?;
```

## License

Apache License 2.0

## Links

- [Allscreenshots website](https://allscreenshots.com)
- [API documentation](https://allscreenshots.com/docs)
- [GitHub repository](https://github.com/allscreenshots/allscreenshots-sdk-rust)
