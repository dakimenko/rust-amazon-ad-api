use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use tokio::time::sleep;

/// RAII guard that automatically records response when dropped
#[must_use = "RateLimitGuard must be held until the API response is received"]
pub struct RateLimitGuard {
    rate_limiter: Arc<Mutex<HashMap<String, TokenBucketState>>>,
    identifier: String,
    auto_record: bool,
}

impl RateLimitGuard {
    fn new(
        rate_limiter: Arc<Mutex<HashMap<String, TokenBucketState>>>,
        identifier: String,
        auto_record: bool,
    ) -> Self {
        Self {
            rate_limiter,
            identifier,
            auto_record,
        }
    }

    /// Manually mark that the API response was received
    /// This will record the response time and prevent automatic recording on drop
    pub async fn mark_response(mut self) {
        if self.auto_record {
            let mut buckets = self.rate_limiter.lock().await;
            let now = Instant::now();

            if let Some(bucket) = buckets.get_mut(&self.identifier) {
                bucket.last_response_time = Some(now);
                tracing::trace!("Manually recorded response time for {}", self.identifier);
            }
        }

        self.auto_record = false;
    }
}

impl Drop for RateLimitGuard {
    fn drop(&mut self) {
        if !self.auto_record {
            return;
        }

        let rate_limiter = self.rate_limiter.clone();
        let identifier = self.identifier.clone();

        tokio::spawn(async move {
            let mut buckets = rate_limiter.lock().await;
            let now = Instant::now();

            if let Some(bucket) = buckets.get_mut(&identifier) {
                bucket.last_response_time = Some(now);
                tracing::trace!("Auto-recorded response time for {}", identifier);
            } else {
                tracing::warn!(
                    "Attempted to auto-record response for unknown endpoint: {}",
                    identifier
                );
            }
        });
    }
}

/// State of a token bucket for rate limiting
#[derive(Debug, Clone)]
pub struct TokenBucketState {
    pub tokens: f64,
    pub last_refill: Instant,
    pub last_response_time: Option<Instant>,
    pub rate: f64,
    pub burst: u32,
    pub initial_burst_used: bool,
}

/// In-memory rate limiter that manages token buckets for different endpoints.
///
/// Thread-safe but not cross-process. Supports adaptive rate limiting —
/// endpoint rates and burst sizes can be updated dynamically when new
/// `x-ad-api-rate-limit-*` headers arrive.
#[derive(Clone)]
pub struct RateLimiter {
    buckets: Arc<Mutex<HashMap<String, TokenBucketState>>>,
    safety_factor: f64,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            buckets: Arc::new(Mutex::new(HashMap::new())),
            safety_factor: 1.10,
        }
    }

    pub fn new_with_safety_factor(safety_factor: f64) -> Self {
        let factor = if safety_factor < 1.0 {
            tracing::warn!(
                "Safety factor {} is less than 1.0, using 1.0 instead",
                safety_factor
            );
            1.0
        } else {
            safety_factor
        };

        Self {
            buckets: Arc::new(Mutex::new(HashMap::new())),
            safety_factor: factor,
        }
    }

    pub fn set_safety_factor(&mut self, safety_factor: f64) {
        if safety_factor < 1.0 {
            tracing::warn!(
                "Safety factor {} is less than 1.0, using 1.0 instead",
                safety_factor
            );
            self.safety_factor = 1.0;
        } else {
            self.safety_factor = safety_factor;
            tracing::info!("Rate limiter safety factor set to {}", safety_factor);
        }
    }

    pub fn get_safety_factor(&self) -> f64 {
        self.safety_factor
    }

    /// Wait for a token to become available for the given endpoint and return a guard.
    /// When the guard is dropped, the response time is auto-recorded.
    #[must_use = "The returned guard must be held until the API response is received"]
    pub async fn wait(&self, identifier: &str, rate: f64, burst: u32) -> RateLimitGuard {
        self._wait_for_token(identifier, rate, burst).await;

        RateLimitGuard::new(self.buckets.clone(), identifier.to_string(), true)
    }

    /// Dynamically update the rate limit for an endpoint based on response headers.
    pub async fn update_limits(&self, identifier: &str, rate: f64, burst: u32) {
        let mut buckets = self.buckets.lock().await;
        let now = Instant::now();
        let bucket = buckets
            .entry(identifier.to_string())
            .or_insert_with(|| TokenBucketState {
                tokens: burst as f64,
                last_refill: now,
                last_response_time: None,
                rate,
                burst,
                initial_burst_used: false,
            });

        if (bucket.rate - rate).abs() > f64::EPSILON || bucket.burst != burst {
            tracing::info!(
                "Updating rate limit for {}: rate {} -> {}, burst {} -> {}",
                identifier,
                bucket.rate,
                rate,
                bucket.burst,
                burst
            );
            bucket.rate = rate;
            bucket.burst = burst;
        }
    }

    async fn _wait_for_token(&self, identifier: &str, rate: f64, burst: u32) {
        loop {
            let mut buckets = self.buckets.lock().await;
            let now = Instant::now();

            let bucket = buckets.entry(identifier.to_string()).or_insert_with(|| {
                tracing::debug!("Creating new token bucket for endpoint: {}", identifier);
                TokenBucketState {
                    tokens: burst as f64,
                    last_refill: now,
                    last_response_time: None,
                    rate,
                    burst,
                    initial_burst_used: false,
                }
            });

            // Update bucket configuration if changed
            if (bucket.rate - rate).abs() > f64::EPSILON || bucket.burst != burst {
                tracing::info!(
                    "Updating rate limit for {}: rate {} -> {}, burst {} -> {}",
                    identifier,
                    bucket.rate,
                    rate,
                    bucket.burst,
                    burst
                );
                bucket.rate = rate;
                bucket.burst = burst;
            }

            // Calculate token refill
            let refill_from_time = if bucket.initial_burst_used {
                bucket.last_response_time.unwrap_or(bucket.last_refill)
            } else {
                bucket.last_refill
            };

            let time_passed = now.duration_since(refill_from_time).as_secs_f64();
            let tokens_to_add = time_passed * bucket.rate;
            let new_tokens = (bucket.tokens + tokens_to_add).min(bucket.burst as f64);

            if new_tokens >= (bucket.burst as f64 - 0.5) && bucket.initial_burst_used {
                bucket.initial_burst_used = false;
                bucket.last_response_time = None;
            }

            bucket.tokens = new_tokens;
            bucket.last_refill = now;

            // Enforce minimum interval after initial burst
            if bucket.initial_burst_used {
                if let Some(last_response_time) = bucket.last_response_time {
                    let base_minimum_interval = 1.0 / bucket.rate;
                    let minimum_interval = base_minimum_interval * self.safety_factor;
                    let time_since_response = now.duration_since(last_response_time).as_secs_f64();

                    if time_since_response < minimum_interval {
                        let wait_seconds = minimum_interval - time_since_response;
                        tracing::debug!(
                            "Enforcing minimum interval for {} (safety factor: {:.2}): waiting {:.3}s",
                            identifier,
                            self.safety_factor,
                            wait_seconds
                        );

                        drop(buckets);
                        sleep(Duration::from_secs_f64(wait_seconds)).await;
                        continue;
                    }
                }
            }

            // Check if we have a token available
            if bucket.tokens >= 1.0 {
                bucket.tokens -= 1.0;

                if bucket.burst == 1 || bucket.tokens <= 0.0 {
                    bucket.initial_burst_used = true;
                }

                tracing::debug!(
                    "Token consumed for {}, {:.1} tokens remaining",
                    identifier,
                    bucket.tokens,
                );

                return;
            }

            // Calculate wait time for next token
            let tokens_needed = 1.0 - bucket.tokens;
            let base_wait_seconds = tokens_needed / bucket.rate;
            let wait_seconds = base_wait_seconds * self.safety_factor;

            if !bucket.initial_burst_used {
                bucket.initial_burst_used = true;
            }

            let wait_duration = Duration::from_secs_f64(wait_seconds.max(0.005));

            drop(buckets);
            sleep(wait_duration).await;
        }
    }

    pub async fn check_token_availability(&self, identifier: &str) -> bool {
        let mut buckets = self.buckets.lock().await;
        let now = Instant::now();

        if let Some(bucket) = buckets.get_mut(identifier) {
            let time_passed = now.duration_since(bucket.last_refill).as_secs_f64();
            let tokens_to_add = time_passed * bucket.rate;
            bucket.tokens = (bucket.tokens + tokens_to_add).min(bucket.burst as f64);
            bucket.last_refill = now;

            bucket.tokens >= 1.0
        } else {
            true
        }
    }

    pub async fn get_token_status(&self) -> HashMap<String, (f64, f64, u32)> {
        let mut buckets = self.buckets.lock().await;
        let now = Instant::now();
        let mut status = HashMap::new();

        for (endpoint_key, bucket) in buckets.iter_mut() {
            let time_passed = now.duration_since(bucket.last_refill).as_secs_f64();
            let tokens_to_add = time_passed * bucket.rate;
            bucket.tokens = (bucket.tokens + tokens_to_add).min(bucket.burst as f64);
            bucket.last_refill = now;

            status.insert(
                endpoint_key.clone(),
                (bucket.tokens, bucket.rate, bucket.burst),
            );
        }

        status
    }

    pub async fn reset(&self) {
        let mut buckets = self.buckets.lock().await;
        buckets.clear();
        tracing::debug!("Rate limiter state reset");
    }

    pub async fn active_buckets_count(&self) -> usize {
        let buckets = self.buckets.lock().await;
        buckets.len()
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limiter_subsecond_precision() {
        let limiter = RateLimiter::new();

        // 1. Initial token consumption
        let guard = limiter.wait("sp-campaigns", 10.0, 5).await;
        assert_eq!(limiter.active_buckets_count().await, 1);
        guard.mark_response().await;

        // 2. Check token status
        let status = limiter.get_token_status().await;
        let (tokens, rate, burst) = status.get("sp-campaigns").expect("bucket exists");
        assert_eq!(*rate, 10.0);
        assert_eq!(*burst, 5);
        assert!(*tokens <= 5.0 && *tokens >= 3.9);
    }
}
