//! # Allscreenshots SDK
//!
//! Official Rust SDK for the Allscreenshots API - capture website screenshots programmatically.
//!
//! ## Quick start
//!
//! ```rust,no_run
//! use allscreenshots_sdk::{AllscreenshotsClient, ScreenshotRequest};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create client (reads API key from ALLSCREENSHOTS_API_KEY env var)
//!     let client = AllscreenshotsClient::from_env()?;
//!
//!     // Take a screenshot
//!     let request = ScreenshotRequest::builder()
//!         .url("https://github.com")
//!         .device("Desktop HD")
//!         .build()?;
//!
//!     let image_bytes = client.screenshot(&request).await?;
//!     std::fs::write("screenshot.png", &image_bytes)?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Configuration
//!
//! ```rust,no_run
//! use allscreenshots_sdk::AllscreenshotsClient;
//! use std::time::Duration;
//!
//! let client = AllscreenshotsClient::builder()
//!     .api_key("your-api-key")
//!     .base_url("https://api.allscreenshots.com")
//!     .timeout(Duration::from_secs(60))
//!     .max_retries(3)
//!     .build()?;
//! # Ok::<(), allscreenshots_sdk::AllscreenshotsError>(())
//! ```

pub mod client;
pub mod error;
pub mod models;
mod retry;

pub use client::{AllscreenshotsClient, AllscreenshotsClientBuilder};
pub use error::{AllscreenshotsError, ErrorCode};
pub use models::*;
