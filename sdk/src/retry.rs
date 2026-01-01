//! Retry logic with exponential backoff.

use crate::error::AllscreenshotsError;
use rand::Rng;
use std::time::Duration;

/// Configuration for retry behavior.
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retry attempts
    pub max_retries: u32,
    /// Initial delay between retries
    pub initial_delay: Duration,
    /// Maximum delay between retries
    pub max_delay: Duration,
    /// Multiplier for exponential backoff
    pub multiplier: f64,
    /// Jitter factor (0.0 to 1.0)
    pub jitter: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay: Duration::from_millis(500),
            max_delay: Duration::from_secs(30),
            multiplier: 2.0,
            jitter: 0.1,
        }
    }
}

impl RetryConfig {
    /// Calculate the delay for a given attempt number.
    pub fn delay_for_attempt(&self, attempt: u32) -> Duration {
        if attempt == 0 {
            return Duration::ZERO;
        }

        let base_delay = self.initial_delay.as_secs_f64() * self.multiplier.powi(attempt as i32 - 1);
        let max_delay = self.max_delay.as_secs_f64();
        let capped_delay = base_delay.min(max_delay);

        // Add jitter
        let jitter_range = capped_delay * self.jitter;
        let jitter = rand::thread_rng().gen_range(-jitter_range..jitter_range);
        let final_delay = (capped_delay + jitter).max(0.0);

        Duration::from_secs_f64(final_delay)
    }
}

/// Execute an async operation with retry logic.
pub async fn with_retry<F, Fut, T>(
    config: &RetryConfig,
    mut operation: F,
) -> Result<T, AllscreenshotsError>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, AllscreenshotsError>>,
{
    let mut last_error = None;

    for attempt in 0..=config.max_retries {
        if attempt > 0 {
            let delay = config.delay_for_attempt(attempt);
            tokio::time::sleep(delay).await;
        }

        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                if e.is_retryable() && attempt < config.max_retries {
                    last_error = Some(e);
                    continue;
                }
                return Err(e);
            }
        }
    }

    Err(AllscreenshotsError::RetriesExhausted(
        last_error
            .map(|e| e.to_string())
            .unwrap_or_else(|| "Unknown error".to_string()),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = RetryConfig::default();
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.initial_delay, Duration::from_millis(500));
        assert_eq!(config.max_delay, Duration::from_secs(30));
    }

    #[test]
    fn test_delay_calculation() {
        let config = RetryConfig {
            max_retries: 3,
            initial_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(60),
            multiplier: 2.0,
            jitter: 0.0,
        };

        assert_eq!(config.delay_for_attempt(0), Duration::ZERO);
        assert_eq!(config.delay_for_attempt(1), Duration::from_secs(1));
        assert_eq!(config.delay_for_attempt(2), Duration::from_secs(2));
        assert_eq!(config.delay_for_attempt(3), Duration::from_secs(4));
    }

    #[test]
    fn test_delay_capped_at_max() {
        let config = RetryConfig {
            max_retries: 10,
            initial_delay: Duration::from_secs(10),
            max_delay: Duration::from_secs(30),
            multiplier: 2.0,
            jitter: 0.0,
        };

        let delay = config.delay_for_attempt(5);
        assert!(delay <= Duration::from_secs(30));
    }
}
