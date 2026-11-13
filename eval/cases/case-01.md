# Case 01: a bounded config value over an HTTP-style API

## Plain request

> Write tests for this.

## Material (given to both conditions)

The task:

> Add a `max_retries` setting to the job service. `PUT /config/max_retries` with body `{"value": <integer>}` sets it. Valid range is 0 to 10 inclusive. Default is 3 when it has never been set. Out-of-range or malformed values return HTTP 422 with body `{"error": "max_retries_out_of_range"}`. The retry worker must use the saved value for jobs it starts after the change.

A first implementation is in `src/config.rs`. `cargo test` passes.

```rust
// src/config.rs
use std::sync::atomic::{AtomicU8, Ordering};

/// Value used when `max_retries` has never been set.
pub const DEFAULT_MAX_RETRIES: u8 = 3;

/// Retry limit used by the worker.
pub const MAX_RETRIES: u8 = 3;

#[derive(Debug, PartialEq, Eq)]
pub enum ConfigError {
    OutOfRange,
}

pub struct Config {
    max_retries: AtomicU8,
}

impl Config {
    pub fn new() -> Self {
        Config {
            max_retries: AtomicU8::new(DEFAULT_MAX_RETRIES),
        }
    }

    pub fn max_retries(&self) -> u8 {
        self.max_retries.load(Ordering::SeqCst)
    }

    /// Sets `max_retries`. The valid range is 0 to 10 inclusive.
    pub fn set_max_retries(&self, value: i64) -> Result<(), ConfigError> {
        if value < 10 {
            self.max_retries.store(value as u8, Ordering::SeqCst);
            Ok(())
        } else {
            Err(ConfigError::OutOfRange)
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct HttpResponse {
    pub status: u16,
    pub body: String,
}

/// Handles `PUT /config/max_retries`.
pub fn put_max_retries(config: &Config, body: &str) -> HttpResponse {
    let value = match parse_value(body) {
        Some(value) => value,
        None => {
            return HttpResponse {
                status: 400,
                body: r#"{"error": "bad_request"}"#.to_string(),
            }
        }
    };
    match config.set_max_retries(value) {
        Ok(()) => HttpResponse {
            status: 200,
            body: format!(r#"{{"value": {value}}}"#),
        },
        Err(ConfigError::OutOfRange) => HttpResponse {
            status: 422,
            body: r#"{"error": "max_retries_out_of_range"}"#.to_string(),
        },
    }
}

/// Extracts the integer after `"value":` from a small JSON object.
fn parse_value(body: &str) -> Option<i64> {
    let after_key = body.split_once("\"value\"")?.1;
    let after_colon = after_key.split_once(':')?.1;
    let token = after_colon.split([',', '}']).next()?;
    token.trim().parse::<i64>().ok()
}

/// Runs jobs and retries them when they fail.
pub struct RetryWorker;

impl RetryWorker {
    /// Calls `job` until it returns `true` or the retry limit is reached.
    /// Returns the number of attempts made (the first try plus retries).
    pub fn run<F: FnMut() -> bool>(&self, mut job: F) -> u32 {
        let mut attempts = 0;
        loop {
            attempts += 1;
            if job() || attempts > u32::from(MAX_RETRIES) {
                return attempts;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_accepts_value_in_range() {
        let config = Config::new();
        assert_eq!(config.set_max_retries(5), Ok(()));
        assert_eq!(config.max_retries(), 5);
    }

    #[test]
    fn set_rejects_value_above_range() {
        let config = Config::new();
        assert_eq!(config.set_max_retries(11), Err(ConfigError::OutOfRange));
    }

    #[test]
    fn worker_stops_after_retry_limit() {
        let worker = RetryWorker;
        assert_eq!(worker.run(|| false), 4);
    }
}
```
