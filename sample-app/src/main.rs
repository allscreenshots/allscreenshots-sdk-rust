//! Allscreenshots Demo - A web application demonstrating the Rust SDK.

use allscreenshots_sdk::{AllscreenshotsClient, ScreenshotRequest, AllscreenshotsError};
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use base64::{engine::general_purpose::STANDARD, Engine};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Application state shared across handlers.
struct AppState {
    client: AllscreenshotsClient,
}

/// Screenshot request from the frontend.
#[derive(Debug, Deserialize)]
struct ScreenshotFormRequest {
    url: String,
    device: String,
    full_page: bool,
}

/// Screenshot response to the frontend.
#[derive(Debug, Serialize)]
struct ScreenshotResponse {
    success: bool,
    image: Option<String>,
    error: Option<String>,
}

/// Application error type.
struct AppError(AllscreenshotsError);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let error_message = self.0.to_string();
        let response = ScreenshotResponse {
            success: false,
            image: None,
            error: Some(error_message),
        };
        (StatusCode::OK, Json(response)).into_response()
    }
}

impl From<AllscreenshotsError> for AppError {
    fn from(err: AllscreenshotsError) -> Self {
        AppError(err)
    }
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "allscreenshots_demo=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Create SDK client
    let client = match AllscreenshotsClient::from_env() {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Failed to create Allscreenshots client: {}", e);
            eprintln!("Make sure ALLSCREENSHOTS_API_KEY environment variable is set.");
            std::process::exit(1);
        }
    };

    let state = Arc::new(AppState { client });

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build router
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/api/screenshot", post(screenshot_handler))
        .layer(cors)
        .with_state(state);

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Server running at http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

/// Serve the index HTML page.
async fn index_handler() -> Html<&'static str> {
    Html(INDEX_HTML)
}

/// Handle screenshot requests.
async fn screenshot_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ScreenshotFormRequest>,
) -> Result<Json<ScreenshotResponse>, AppError> {
    tracing::info!(
        "Taking screenshot: url={}, device={}, full_page={}",
        payload.url,
        payload.device,
        payload.full_page
    );

    // Build the screenshot request
    let request = ScreenshotRequest::builder()
        .url(&payload.url)
        .device(&payload.device)
        .full_page(payload.full_page)
        .build()?;

    // Take the screenshot
    let image_bytes = state.client.screenshot(&request).await?;

    // Encode as base64
    let base64_image = STANDARD.encode(&image_bytes);
    let data_url = format!("data:image/png;base64,{}", base64_image);

    tracing::info!("Screenshot captured successfully ({} bytes)", image_bytes.len());

    Ok(Json(ScreenshotResponse {
        success: true,
        image: Some(data_url),
        error: None,
    }))
}

const INDEX_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Allscreenshots Demo</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif;
            background: #f5f5f5;
            color: #333;
            min-height: 100vh;
        }
        .container {
            max-width: 900px;
            margin: 0 auto;
            padding: 20px;
        }
        header {
            background: #1a1a1a;
            color: white;
            padding: 20px;
            margin-bottom: 30px;
        }
        header h1 {
            font-size: 20px;
            font-weight: 600;
        }
        .form-section {
            background: white;
            border-radius: 8px;
            padding: 24px;
            margin-bottom: 24px;
            box-shadow: 0 1px 3px rgba(0,0,0,0.1);
        }
        .form-row {
            display: flex;
            gap: 16px;
            margin-bottom: 16px;
            align-items: flex-end;
        }
        .form-group {
            flex: 1;
        }
        .form-group.url-input {
            flex: 2;
        }
        .form-group label {
            display: block;
            font-size: 14px;
            font-weight: 500;
            margin-bottom: 6px;
            color: #555;
        }
        .form-group input[type="text"],
        .form-group select {
            width: 100%;
            padding: 10px 12px;
            border: 1px solid #ddd;
            border-radius: 6px;
            font-size: 14px;
            transition: border-color 0.2s;
        }
        .form-group input[type="text"]:focus,
        .form-group select:focus {
            outline: none;
            border-color: #1a1a1a;
        }
        .checkbox-group {
            display: flex;
            align-items: center;
            gap: 8px;
            padding: 10px 0;
        }
        .checkbox-group input[type="checkbox"] {
            width: 18px;
            height: 18px;
            cursor: pointer;
        }
        .checkbox-group label {
            font-size: 14px;
            cursor: pointer;
            margin-bottom: 0;
        }
        button {
            background: #1a1a1a;
            color: white;
            border: none;
            padding: 12px 24px;
            border-radius: 6px;
            font-size: 14px;
            font-weight: 500;
            cursor: pointer;
            transition: background 0.2s;
        }
        button:hover {
            background: #333;
        }
        button:disabled {
            background: #999;
            cursor: not-allowed;
        }
        .result-section {
            background: white;
            border-radius: 8px;
            overflow: hidden;
            box-shadow: 0 1px 3px rgba(0,0,0,0.1);
        }
        .result-header {
            padding: 16px 24px;
            border-bottom: 1px solid #eee;
            font-weight: 500;
        }
        .result-content {
            padding: 24px;
            min-height: 300px;
            display: flex;
            align-items: center;
            justify-content: center;
            background: #fafafa;
        }
        .result-content.empty {
            color: #999;
            font-size: 14px;
        }
        .result-content img {
            max-width: 100%;
            height: auto;
            border-radius: 4px;
            box-shadow: 0 2px 8px rgba(0,0,0,0.15);
        }
        .loading {
            display: flex;
            flex-direction: column;
            align-items: center;
            gap: 16px;
            color: #666;
        }
        .spinner {
            width: 40px;
            height: 40px;
            border: 3px solid #eee;
            border-top-color: #1a1a1a;
            border-radius: 50%;
            animation: spin 1s linear infinite;
        }
        @keyframes spin {
            to { transform: rotate(360deg); }
        }
        .error {
            background: #fee2e2;
            color: #991b1b;
            padding: 16px;
            border-radius: 6px;
            font-size: 14px;
            width: 100%;
        }
    </style>
</head>
<body>
    <header>
        <div class="container">
            <h1>Allscreenshots Demo</h1>
        </div>
    </header>
    <div class="container">
        <div class="form-section">
            <div class="form-row">
                <div class="form-group url-input">
                    <label for="url">URL</label>
                    <input type="text" id="url" value="https://github.com" placeholder="https://example.com">
                </div>
                <button id="capture-btn" onclick="takeScreenshot()">Take Screenshot</button>
            </div>
            <div class="form-row">
                <div class="form-group">
                    <label for="device">Device</label>
                    <select id="device">
                        <option value="Desktop HD">Desktop HD</option>
                        <option value="Desktop">Desktop</option>
                        <option value="Laptop">Laptop</option>
                        <option value="iPhone 14">iPhone 14</option>
                        <option value="iPhone 14 Pro Max">iPhone 14 Pro Max</option>
                        <option value="iPad">iPad</option>
                        <option value="iPad Pro">iPad Pro</option>
                    </select>
                </div>
                <div class="form-group">
                    <div class="checkbox-group">
                        <input type="checkbox" id="fullPage">
                        <label for="fullPage">Full page</label>
                    </div>
                </div>
            </div>
        </div>
        <div class="result-section">
            <div class="result-header">Result</div>
            <div class="result-content empty" id="result">
                Enter a URL and click "Take Screenshot" to capture
            </div>
        </div>
    </div>
    <script>
        async function takeScreenshot() {
            const url = document.getElementById('url').value;
            const device = document.getElementById('device').value;
            const fullPage = document.getElementById('fullPage').checked;
            const resultDiv = document.getElementById('result');
            const captureBtn = document.getElementById('capture-btn');

            if (!url) {
                resultDiv.innerHTML = '<div class="error">Please enter a URL</div>';
                resultDiv.className = 'result-content';
                return;
            }

            // Show loading state
            captureBtn.disabled = true;
            captureBtn.textContent = 'Capturing...';
            resultDiv.className = 'result-content';
            resultDiv.innerHTML = `
                <div class="loading">
                    <div class="spinner"></div>
                    <span>Capturing screenshot...</span>
                </div>
            `;

            try {
                const response = await fetch('/api/screenshot', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify({
                        url: url,
                        device: device,
                        full_page: fullPage
                    })
                });

                const data = await response.json();

                if (data.success && data.image) {
                    resultDiv.innerHTML = `<img src="${data.image}" alt="Screenshot">`;
                } else {
                    resultDiv.innerHTML = `<div class="error">${data.error || 'Failed to capture screenshot'}</div>`;
                }
            } catch (error) {
                resultDiv.innerHTML = `<div class="error">Error: ${error.message}</div>`;
            } finally {
                captureBtn.disabled = false;
                captureBtn.textContent = 'Take Screenshot';
            }
        }

        // Allow Enter key to trigger screenshot
        document.getElementById('url').addEventListener('keypress', function(e) {
            if (e.key === 'Enter') {
                takeScreenshot();
            }
        });
    </script>
</body>
</html>
"#;
