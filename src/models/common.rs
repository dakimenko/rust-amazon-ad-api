use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Generic API response wrapper containing parsed payload, headers,
/// pagination tokens, and rate limit information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub payload: T,
    #[serde(skip)]
    pub headers: HashMap<String, String>,
    #[serde(skip)]
    pub next_token: Option<String>,
    #[serde(skip)]
    pub rate_limit: Option<RateLimitInfo>,
}

/// Rate limit information extracted from response headers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitInfo {
    pub limit: Option<f64>,
    pub remaining: Option<u32>,
    pub reset: Option<i64>,
}

impl<T> ApiResponse<T> {
    /// Build an ApiResponse from raw parts: headers map, parsed payload,
    /// and the raw JSON body string (for extracting body-level `nextCursor`).
    pub fn from_parts(payload: T, headers: HashMap<String, String>, body_str: &str) -> Self {
        // Try to extract next_token from response headers (normalized lowercase)
        let mut next_token = headers
            .get("nexttoken")
            .or_else(|| headers.get("x-amzn-next-token"))
            .cloned();

        // Fallback: extract `nextCursor` from JSON body (Advertising API v3)
        if next_token.is_none() {
            if let Ok(json) = serde_json::from_str::<Value>(body_str) {
                if let Some(cursor) = json.get("nextCursor").and_then(|v| v.as_str()) {
                    if !cursor.is_empty() {
                        next_token = Some(cursor.to_string());
                    }
                }
            }
        }

        // Extract rate limit from x-ad-api-rate-limit-* headers (new format)
        let rate_limit = {
            let limit = headers
                .get("x-ad-api-rate-limit-limit")
                .and_then(|v| v.parse::<f64>().ok());
            let remaining = headers
                .get("x-ad-api-rate-limit-remaining")
                .and_then(|v| v.parse::<u32>().ok());
            let reset = headers
                .get("x-ad-api-rate-limit-reset")
                .and_then(|v| v.parse::<i64>().ok());

            // Fallback to x-amzn-ratelimit-* headers (old format)
            let limit = limit.or_else(|| {
                headers
                    .get("x-amzn-ratelimit-limit")
                    .and_then(|v| v.parse::<f64>().ok())
            });
            let remaining = remaining.or_else(|| {
                headers
                    .get("x-amzn-ratelimit-remaining")
                    .and_then(|v| v.parse::<u32>().ok())
            });
            let reset = reset.or_else(|| {
                headers
                    .get("x-amzn-ratelimit-reset")
                    .and_then(|v| v.parse::<i64>().ok())
            });

            if limit.is_some() || remaining.is_some() {
                Some(RateLimitInfo {
                    limit,
                    remaining,
                    reset,
                })
            } else {
                None
            }
        };

        ApiResponse {
            payload,
            headers,
            next_token,
            rate_limit,
        }
    }

    /// Check if there are more pages available.
    pub fn has_next(&self) -> bool {
        self.next_token.is_some()
    }
}

/// Standard pagination cursor from Amazon API responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationCursor {
    #[serde(rename = "nextCursor", default)]
    pub next_cursor: Option<String>,
    #[serde(default)]
    pub count: Option<i64>,
}
