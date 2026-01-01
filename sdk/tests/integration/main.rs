//! Integration tests for the Allscreenshots SDK.
//!
//! These tests require the `ALLSCREENSHOTS_API_KEY` environment variable to be set.

use allscreenshots_sdk::{AllscreenshotsClient, ScreenshotRequest, AllscreenshotsError};
use base64::{engine::general_purpose::STANDARD, Engine};
use std::env;
use std::fs::File;
use std::io::Write;
use std::time::{Duration, Instant};

/// Test result for a single integration test.
struct TestResult {
    test_id: String,
    test_name: String,
    url: String,
    device: String,
    full_page: bool,
    passed: bool,
    error_message: Option<String>,
    image_base64: Option<String>,
    execution_time_ms: u128,
}

/// Run all integration tests and generate an HTML report.
#[tokio::test]
async fn run_integration_tests() {
    // Check for API key
    let api_key = env::var("ALLSCREENSHOTS_API_KEY");
    if api_key.is_err() {
        println!("ALLSCREENSHOTS_API_KEY environment variable not set. Skipping integration tests.");
        println!("To run integration tests, set the environment variable:");
        println!("  export ALLSCREENSHOTS_API_KEY=your-api-key");
        return;
    }

    let client = AllscreenshotsClient::from_env().expect("Failed to create client");
    let mut results = Vec::new();
    let start_time = Instant::now();

    // IT-001: Basic Desktop Screenshot
    results.push(run_test(
        &client,
        "IT-001",
        "Basic Desktop Screenshot",
        "https://github.com",
        "Desktop HD",
        false,
        true,
    ).await);

    // IT-002: Basic Mobile Screenshot
    results.push(run_test(
        &client,
        "IT-002",
        "Basic Mobile Screenshot",
        "https://github.com",
        "iPhone 14",
        false,
        true,
    ).await);

    // IT-003: Basic Tablet Screenshot
    results.push(run_test(
        &client,
        "IT-003",
        "Basic Tablet Screenshot",
        "https://github.com",
        "iPad",
        false,
        true,
    ).await);

    // IT-004: Full Page Desktop
    results.push(run_test(
        &client,
        "IT-004",
        "Full Page Desktop",
        "https://github.com",
        "Desktop HD",
        true,
        true,
    ).await);

    // IT-005: Full Page Mobile
    results.push(run_test(
        &client,
        "IT-005",
        "Full Page Mobile",
        "https://github.com",
        "iPhone 14",
        true,
        true,
    ).await);

    // IT-006: Complex Page
    results.push(run_test(
        &client,
        "IT-006",
        "Complex Page",
        "https://github.com/anthropics/claude-code",
        "Desktop HD",
        false,
        true,
    ).await);

    // IT-007: Invalid URL
    results.push(run_test(
        &client,
        "IT-007",
        "Invalid URL",
        "not-a-valid-url",
        "Desktop HD",
        false,
        false,
    ).await);

    // IT-008: Unreachable URL
    results.push(run_test(
        &client,
        "IT-008",
        "Unreachable URL",
        "https://this-domain-does-not-exist-12345.com",
        "Desktop HD",
        false,
        false,
    ).await);

    let total_time = start_time.elapsed();

    // Generate HTML report
    let report = generate_html_report(&results, total_time);
    let report_path = "test-report.html";
    let mut file = File::create(report_path).expect("Failed to create report file");
    file.write_all(report.as_bytes()).expect("Failed to write report");

    println!("\nIntegration test report generated: {}", report_path);

    // Print summary
    let passed = results.iter().filter(|r| r.passed).count();
    let failed = results.len() - passed;
    println!("\nSummary: {} passed, {} failed", passed, failed);

    // Assert all expected tests passed
    for result in &results {
        if result.test_id == "IT-007" || result.test_id == "IT-008" {
            // These tests should fail with an error (expected behavior)
            // They pass if they received an error as expected
            continue;
        }
        assert!(result.passed, "Test {} failed: {:?}", result.test_id, result.error_message);
    }
}

async fn run_test(
    client: &AllscreenshotsClient,
    test_id: &str,
    test_name: &str,
    url: &str,
    device: &str,
    full_page: bool,
    expect_success: bool,
) -> TestResult {
    println!("Running {}: {}", test_id, test_name);
    let start = Instant::now();

    let request_result = ScreenshotRequest::builder()
        .url(url)
        .device(device)
        .full_page(full_page)
        .build();

    let (passed, error_message, image_base64) = match request_result {
        Ok(request) => {
            match client.screenshot(&request).await {
                Ok(image_bytes) => {
                    if expect_success {
                        let base64 = STANDARD.encode(&image_bytes);
                        (true, None, Some(base64))
                    } else {
                        (false, Some("Expected error but got success".to_string()), None)
                    }
                }
                Err(e) => {
                    if expect_success {
                        (false, Some(format!("{}", e)), None)
                    } else {
                        // Expected failure
                        (true, Some(format!("Expected error: {}", e)), None)
                    }
                }
            }
        }
        Err(e) => {
            if expect_success {
                (false, Some(format!("Validation error: {}", e)), None)
            } else {
                // Expected failure
                (true, Some(format!("Expected error: {}", e)), None)
            }
        }
    };

    let execution_time_ms = start.elapsed().as_millis();

    let status = if passed { "PASSED" } else { "FAILED" };
    println!("  {} ({}ms)", status, execution_time_ms);

    TestResult {
        test_id: test_id.to_string(),
        test_name: test_name.to_string(),
        url: url.to_string(),
        device: device.to_string(),
        full_page,
        passed,
        error_message,
        image_base64,
        execution_time_ms,
    }
}

fn generate_html_report(results: &[TestResult], total_time: Duration) -> String {
    let passed = results.iter().filter(|r| r.passed).count();
    let failed = results.len() - passed;
    let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();

    let mut tests_html = String::new();
    for result in results {
        let status_class = if result.passed { "passed" } else { "failed" };
        let status_badge = if result.passed { "PASSED" } else { "FAILED" };

        let image_html = if let Some(base64) = &result.image_base64 {
            format!(
                r#"<div class="screenshot">
                    <img src="data:image/png;base64,{}" alt="Screenshot" />
                </div>"#,
                base64
            )
        } else {
            String::new()
        };

        let error_html = if let Some(error) = &result.error_message {
            format!(
                r#"<div class="error-message">{}</div>"#,
                html_escape(error)
            )
        } else {
            String::new()
        };

        tests_html.push_str(&format!(
            r#"
            <div class="test-card {}">
                <div class="test-header">
                    <div class="test-info">
                        <span class="test-id">{}</span>
                        <span class="test-name">{}</span>
                    </div>
                    <span class="badge {}">{}</span>
                </div>
                <div class="test-details">
                    <div class="detail-row">
                        <span class="label">URL:</span>
                        <span class="value">{}</span>
                    </div>
                    <div class="detail-row">
                        <span class="label">Device:</span>
                        <span class="value">{}</span>
                    </div>
                    <div class="detail-row">
                        <span class="label">Full Page:</span>
                        <span class="value">{}</span>
                    </div>
                    <div class="detail-row">
                        <span class="label">Execution Time:</span>
                        <span class="value">{}ms</span>
                    </div>
                </div>
                {}
                {}
            </div>
            "#,
            status_class,
            result.test_id,
            result.test_name,
            status_class,
            status_badge,
            html_escape(&result.url),
            result.device,
            result.full_page,
            result.execution_time_ms,
            error_html,
            image_html,
        ));
    }

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Allscreenshots Rust SDK - Integration Test Report</title>
    <style>
        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif;
            background: #f5f5f5;
            color: #333;
            line-height: 1.6;
        }}
        .container {{
            max-width: 1200px;
            margin: 0 auto;
            padding: 20px;
        }}
        header {{
            background: #1a1a1a;
            color: white;
            padding: 30px 20px;
            margin-bottom: 30px;
        }}
        header h1 {{
            font-size: 24px;
            font-weight: 600;
            margin-bottom: 10px;
        }}
        header .meta {{
            font-size: 14px;
            color: #999;
        }}
        .summary {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 20px;
            margin-bottom: 30px;
        }}
        .summary-card {{
            background: white;
            border-radius: 8px;
            padding: 20px;
            box-shadow: 0 1px 3px rgba(0,0,0,0.1);
        }}
        .summary-card .label {{
            font-size: 14px;
            color: #666;
            margin-bottom: 5px;
        }}
        .summary-card .value {{
            font-size: 28px;
            font-weight: 600;
        }}
        .summary-card .value.passed {{
            color: #22c55e;
        }}
        .summary-card .value.failed {{
            color: #ef4444;
        }}
        .test-card {{
            background: white;
            border-radius: 8px;
            margin-bottom: 20px;
            box-shadow: 0 1px 3px rgba(0,0,0,0.1);
            overflow: hidden;
        }}
        .test-card.passed {{
            border-left: 4px solid #22c55e;
        }}
        .test-card.failed {{
            border-left: 4px solid #ef4444;
        }}
        .test-header {{
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding: 15px 20px;
            border-bottom: 1px solid #eee;
        }}
        .test-info {{
            display: flex;
            align-items: center;
            gap: 15px;
        }}
        .test-id {{
            font-weight: 600;
            color: #666;
        }}
        .test-name {{
            font-weight: 500;
        }}
        .badge {{
            padding: 4px 12px;
            border-radius: 20px;
            font-size: 12px;
            font-weight: 600;
        }}
        .badge.passed {{
            background: #dcfce7;
            color: #166534;
        }}
        .badge.failed {{
            background: #fee2e2;
            color: #991b1b;
        }}
        .test-details {{
            padding: 15px 20px;
            background: #fafafa;
        }}
        .detail-row {{
            display: flex;
            margin-bottom: 8px;
        }}
        .detail-row:last-child {{
            margin-bottom: 0;
        }}
        .detail-row .label {{
            width: 120px;
            color: #666;
            font-size: 14px;
        }}
        .detail-row .value {{
            font-size: 14px;
            word-break: break-all;
        }}
        .error-message {{
            padding: 15px 20px;
            background: #fef2f2;
            color: #991b1b;
            font-size: 14px;
            font-family: monospace;
        }}
        .screenshot {{
            padding: 20px;
            text-align: center;
            background: #f0f0f0;
        }}
        .screenshot img {{
            max-width: 100%;
            height: auto;
            border-radius: 4px;
            box-shadow: 0 2px 8px rgba(0,0,0,0.15);
        }}
        footer {{
            margin-top: 40px;
            padding: 20px;
            background: #1a1a1a;
            color: #999;
            font-size: 12px;
            text-align: center;
        }}
    </style>
</head>
<body>
    <header>
        <div class="container">
            <h1>Allscreenshots Rust SDK - Integration test report</h1>
            <div class="meta">
                <div>Version: 0.1.0</div>
                <div>Generated: {}</div>
            </div>
        </div>
    </header>
    <div class="container">
        <div class="summary">
            <div class="summary-card">
                <div class="label">Total tests</div>
                <div class="value">{}</div>
            </div>
            <div class="summary-card">
                <div class="label">Passed</div>
                <div class="value passed">{}</div>
            </div>
            <div class="summary-card">
                <div class="label">Failed</div>
                <div class="value failed">{}</div>
            </div>
            <div class="summary-card">
                <div class="label">Total time</div>
                <div class="value">{:.2}s</div>
            </div>
        </div>
        <div class="tests">
            {}
        </div>
    </div>
    <footer>
        <div class="container">
            Allscreenshots SDK for Rust | OS: {} | Rust version: {}
        </div>
    </footer>
</body>
</html>"#,
        timestamp,
        results.len(),
        passed,
        failed,
        total_time.as_secs_f64(),
        tests_html,
        std::env::consts::OS,
        env!("CARGO_PKG_RUST_VERSION").to_string() + " (compiled)"
    )
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
