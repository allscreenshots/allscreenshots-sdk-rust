//! Unit tests for request/response models.

use allscreenshots_sdk::*;

#[test]
fn test_screenshot_request_builder() {
    let request = ScreenshotRequest::builder()
        .url("https://example.com")
        .device("Desktop HD")
        .full_page(true)
        .format(ImageFormat::Png)
        .quality(90)
        .build()
        .unwrap();

    assert_eq!(request.url, "https://example.com");
    assert_eq!(request.device, Some("Desktop HD".to_string()));
    assert_eq!(request.full_page, Some(true));
    assert_eq!(request.format, Some(ImageFormat::Png));
    assert_eq!(request.quality, Some(90));
}

#[test]
fn test_screenshot_request_missing_url() {
    let result = ScreenshotRequest::builder().build();
    assert!(result.is_err());
}

#[test]
fn test_screenshot_request_invalid_url() {
    let result = ScreenshotRequest::builder()
        .url("not-a-valid-url")
        .build();
    assert!(result.is_err());
}

#[test]
fn test_screenshot_request_invalid_quality() {
    let result = ScreenshotRequest::builder()
        .url("https://example.com")
        .quality(150)
        .build();
    assert!(result.is_err());
}

#[test]
fn test_screenshot_request_invalid_delay() {
    let result = ScreenshotRequest::builder()
        .url("https://example.com")
        .delay(50000)
        .build();
    assert!(result.is_err());
}

#[test]
fn test_screenshot_request_invalid_timeout() {
    let result = ScreenshotRequest::builder()
        .url("https://example.com")
        .timeout(500)
        .build();
    assert!(result.is_err());
}

#[test]
fn test_screenshot_request_simple() {
    let request = ScreenshotRequest::simple("https://example.com");
    assert_eq!(request.url, "https://example.com");
    assert!(request.device.is_none());
    assert!(request.full_page.is_none());
}

#[test]
fn test_viewport_config() {
    let viewport = ViewportConfig::new(1920, 1080).with_scale_factor(2);
    assert_eq!(viewport.width, Some(1920));
    assert_eq!(viewport.height, Some(1080));
    assert_eq!(viewport.device_scale_factor, Some(2));
}

#[test]
fn test_bulk_url_request() {
    let url_request = BulkUrlRequest::new("https://example.com");
    assert_eq!(url_request.url, "https://example.com");
    assert!(url_request.options.is_none());
}

#[test]
fn test_bulk_request() {
    let bulk_request = BulkRequest::new(vec![
        BulkUrlRequest::new("https://example1.com"),
        BulkUrlRequest::new("https://example2.com"),
    ]);
    assert_eq!(bulk_request.urls.len(), 2);
}

#[test]
fn test_capture_item() {
    let capture = CaptureItem::new("https://example.com")
        .with_device("iPhone 14")
        .with_label("Mobile view");
    assert_eq!(capture.url, "https://example.com");
    assert_eq!(capture.device, Some("iPhone 14".to_string()));
    assert_eq!(capture.label, Some("Mobile view".to_string()));
}

#[test]
fn test_create_schedule_request() {
    let schedule = CreateScheduleRequest::new(
        "Daily capture",
        "https://example.com",
        "0 9 * * *",
    )
    .with_timezone("America/New_York")
    .with_retention_days(30);

    assert_eq!(schedule.name, "Daily capture");
    assert_eq!(schedule.url, "https://example.com");
    assert_eq!(schedule.schedule, "0 9 * * *");
    assert_eq!(schedule.timezone, Some("America/New_York".to_string()));
    assert_eq!(schedule.retention_days, Some(30));
}

#[test]
fn test_job_status_terminal() {
    assert!(JobStatus::Completed.is_terminal());
    assert!(JobStatus::Failed.is_terminal());
    assert!(JobStatus::Cancelled.is_terminal());
    assert!(!JobStatus::Queued.is_terminal());
    assert!(!JobStatus::Processing.is_terminal());
}

#[test]
fn test_job_status_success() {
    assert!(JobStatus::Completed.is_success());
    assert!(!JobStatus::Failed.is_success());
    assert!(!JobStatus::Cancelled.is_success());
    assert!(!JobStatus::Queued.is_success());
    assert!(!JobStatus::Processing.is_success());
}

#[test]
fn test_screenshot_request_serialization() {
    let request = ScreenshotRequest::builder()
        .url("https://example.com")
        .device("Desktop HD")
        .full_page(true)
        .build()
        .unwrap();

    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("\"url\":\"https://example.com\""));
    assert!(json.contains("\"device\":\"Desktop HD\""));
    assert!(json.contains("\"fullPage\":true"));
}

#[test]
fn test_job_response_deserialization() {
    let json = r#"{
        "id": "job-123",
        "status": "COMPLETED",
        "url": "https://example.com",
        "resultUrl": "https://results.allscreenshots.com/job-123.png"
    }"#;

    let response: JobResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.id, "job-123");
    assert_eq!(response.status, JobStatus::Completed);
    assert_eq!(response.url, Some("https://example.com".to_string()));
}

#[test]
fn test_image_format_serialization() {
    let format = ImageFormat::Png;
    let json = serde_json::to_string(&format).unwrap();
    assert_eq!(json, "\"png\"");

    let format = ImageFormat::Jpeg;
    let json = serde_json::to_string(&format).unwrap();
    assert_eq!(json, "\"jpeg\"");
}

#[test]
fn test_wait_until_serialization() {
    let wait = WaitUntil::DomContentLoaded;
    let json = serde_json::to_string(&wait).unwrap();
    assert_eq!(json, "\"domcontentloaded\"");

    let wait = WaitUntil::NetworkIdle;
    let json = serde_json::to_string(&wait).unwrap();
    assert_eq!(json, "\"networkidle\"");
}

#[test]
fn test_layout_type_serialization() {
    let layout = LayoutType::Grid;
    let json = serde_json::to_string(&layout).unwrap();
    assert_eq!(json, "\"GRID\"");

    let layout = LayoutType::Horizontal;
    let json = serde_json::to_string(&layout).unwrap();
    assert_eq!(json, "\"HORIZONTAL\"");
}
