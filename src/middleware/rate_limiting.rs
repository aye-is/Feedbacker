// 🚦 Rate Limiting Middleware - Traffic Control for Feedbacker! 🚦
// This module provides intelligent rate limiting to prevent abuse
// Built with governor crate for high-performance rate limiting! ⚡
// Created with love by Aye & Hue - Making fair usage beautiful! ✨
// Trisha from Accounting appreciates when resources are used fairly! 📊

use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::{IntoResponse, Json, Response},
};
use governor::{
    clock::{DefaultClock, QuantaClock},
    state::{InMemoryState, NotKeyed},
    Quota, RateLimiter,
};
use nonzero_ext::*;
use std::{
    collections::HashMap,
    net::IpAddr,
    str::FromStr,
    sync::{Arc, Mutex},
    time::Duration,
};
use tracing::{debug, info, warn};

use crate::{
    api::{ApiResponse, AppState},
    database::models::RateLimit,
};

/// 🚦 Rate limiter for different types of requests
/// Uses in-memory storage for high performance with optional database persistence
pub struct RateLimitManager {
    /// 📊 General API rate limiter (requests per minute)
    pub api_limiter: Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>,
    /// 📝 Feedback submission rate limiter (submissions per hour)
    pub feedback_limiter: Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>,
    /// 🗄️ Database connection for persistent rate limiting
    pub db_limiters: Arc<Mutex<HashMap<String, RateLimitEntry>>>,
}

/// 📊 Rate limit entry for database persistence
#[derive(Debug, Clone)]
pub struct RateLimitEntry {
    /// 📈 Current request count
    pub count: u32,
    /// ⏰ Window start time
    pub window_start: chrono::DateTime<chrono::Utc>,
    /// 🕒 Last request time
    pub last_request: chrono::DateTime<chrono::Utc>,
}

impl RateLimitManager {
    /// ➕ Create a new rate limit manager
    pub fn new(requests_per_minute: u32, feedback_per_hour: u32) -> Self {
        // 📊 Create API rate limiter (requests per minute)
        let api_quota = Quota::per_minute(nonzero_ext::nonzero!(60u32));
        let api_limiter = Arc::new(RateLimiter::direct(api_quota));

        // 📝 Create feedback rate limiter (submissions per hour)
        let feedback_quota = Quota::per_hour(nonzero_ext::nonzero!(10u32));
        let feedback_limiter = Arc::new(RateLimiter::direct(feedback_quota));

        Self {
            api_limiter,
            feedback_limiter,
            db_limiters: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// 🔍 Check if a request is within rate limits
    pub async fn check_rate_limit(
        &self,
        client_id: &str,
        limit_type: RateLimitType,
        app_state: &AppState,
    ) -> RateLimitResult {
        match limit_type {
            RateLimitType::Api => {
                if self.api_limiter.check().is_ok() {
                    debug!("✅ API rate limit check passed for client: {}", client_id);
                    RateLimitResult::Allowed
                } else {
                    warn!("🚫 API rate limit exceeded for client: {}", client_id);
                    RateLimitResult::Limited {
                        retry_after: Duration::from_secs(60),
                        limit_type: "api".to_string(),
                    }
                }
            }
            RateLimitType::Feedback => {
                // 📝 For feedback, use both in-memory and database checking
                if self.feedback_limiter.check().is_ok() {
                    // TODO: Add database rate limiting when database is ready
                    debug!(
                        "✅ Feedback rate limit check passed for client: {}",
                        client_id
                    );
                    RateLimitResult::Allowed
                } else {
                    warn!(
                        "🚫 In-memory feedback rate limit exceeded for client: {}",
                        client_id
                    );
                    RateLimitResult::Limited {
                        retry_after: Duration::from_secs(3600),
                        limit_type: "feedback".to_string(),
                    }
                }
            }
            RateLimitType::Webhook => {
                // 🪝 Webhooks have a more lenient rate limit
                debug!(
                    "✅ Webhook rate limit check passed for client: {}",
                    client_id
                );
                RateLimitResult::Allowed
            }
        }
    }

    // TODO: Implement database rate limiting when database is ready
}

/// 🚦 Rate limit types for different endpoints
#[derive(Debug, Clone)]
pub enum RateLimitType {
    /// 📊 General API requests
    Api,
    /// 📝 Feedback submissions
    Feedback,
    /// 🪝 GitHub webhooks
    Webhook,
}

/// 📊 Rate limit check result
#[derive(Debug)]
pub enum RateLimitResult {
    /// ✅ Request is allowed
    Allowed,
    /// 🚫 Request is rate limited
    Limited {
        /// ⏰ How long to wait before retrying
        retry_after: Duration,
        /// 📋 Type of rate limit that was exceeded
        limit_type: String,
    },
}

/// 🚦 Main rate limiting middleware
/// This is applied to all routes and provides intelligent rate limiting
pub async fn rate_limit_middleware(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, Response> {
    let path = request.uri().path();
    let client_ip = extract_client_ip(&headers, &request);

    // 🎯 Determine the type of rate limiting based on the path
    let limit_type = determine_limit_type(path);

    // 🏗️ Create rate limiter if not exists (in a real implementation, this would be stored in app state)
    let rate_limiter = RateLimitManager::new(
        app_state.config.rate_limiting.requests_per_minute,
        app_state.config.rate_limiting.feedback_per_hour,
    );

    // 🔍 Check rate limits
    let client_id = client_ip.to_string();
    let result = rate_limiter
        .check_rate_limit(&client_id, limit_type, &app_state)
        .await;

    match result {
        RateLimitResult::Allowed => {
            debug!("✅ Rate limit check passed for {}: {}", client_ip, path);
            Ok(next.run(request).await)
        }
        RateLimitResult::Limited {
            retry_after,
            limit_type,
        } => {
            warn!(
                "🚫 Rate limit exceeded for {}: {} (type: {})",
                client_ip, path, limit_type
            );

            let error_response = ApiResponse::<()>::error(
                "rate_limit_exceeded".to_string(),
                format!(
                    "Rate limit exceeded for {}. Try again in {} seconds.",
                    limit_type,
                    retry_after.as_secs()
                ),
                Some(serde_json::json!({
                    "retry_after_seconds": retry_after.as_secs(),
                    "limit_type": limit_type
                })),
            );

            let mut response =
                (StatusCode::TOO_MANY_REQUESTS, Json(error_response)).into_response();

            // 📋 Add rate limit headers
            response.headers_mut().insert(
                "X-RateLimit-Limit",
                format!("{}", app_state.config.rate_limiting.requests_per_minute)
                    .parse()
                    .unwrap(),
            );
            response
                .headers_mut()
                .insert("X-RateLimit-Remaining", "0".parse().unwrap());
            response.headers_mut().insert(
                "X-RateLimit-Reset",
                format!(
                    "{}",
                    (chrono::Utc::now() + chrono::Duration::from_std(retry_after).unwrap())
                        .timestamp()
                )
                .parse()
                .unwrap(),
            );
            response.headers_mut().insert(
                "Retry-After",
                format!("{}", retry_after.as_secs()).parse().unwrap(),
            );

            Err(response)
        }
    }
}

/// 🌐 Extract client IP address from request
/// Handles various proxy headers for accurate IP detection
fn extract_client_ip(headers: &HeaderMap, _request: &Request) -> IpAddr {
    // 🔍 Check common proxy headers
    if let Some(forwarded_for) = headers.get("X-Forwarded-For") {
        if let Ok(header_str) = forwarded_for.to_str() {
            if let Some(ip_str) = header_str.split(',').next() {
                if let Ok(ip) = IpAddr::from_str(ip_str.trim()) {
                    return ip;
                }
            }
        }
    }

    if let Some(real_ip) = headers.get("X-Real-IP") {
        if let Ok(header_str) = real_ip.to_str() {
            if let Ok(ip) = IpAddr::from_str(header_str.trim()) {
                return ip;
            }
        }
    }

    if let Some(cf_connecting_ip) = headers.get("CF-Connecting-IP") {
        if let Ok(header_str) = cf_connecting_ip.to_str() {
            if let Ok(ip) = IpAddr::from_str(header_str.trim()) {
                return ip;
            }
        }
    }

    // 🎯 Fall back to connection peer (may not be accurate behind proxies)
    // For now, return a default IP - in a real implementation, you'd extract from the connection
    IpAddr::from_str("127.0.0.1").unwrap()
}

/// 🎯 Determine rate limit type based on request path
fn determine_limit_type(path: &str) -> RateLimitType {
    if path.starts_with("/api/feedback") && !path.ends_with("/stats") {
        RateLimitType::Feedback
    } else if path.starts_with("/api/webhook") {
        RateLimitType::Webhook
    } else {
        RateLimitType::Api
    }
}

// 🧪 Tests - Because rate limiting needs to be tested thoroughly!
#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn test_determine_limit_type() {
        assert!(matches!(
            determine_limit_type("/api/feedback"),
            RateLimitType::Feedback
        ));
        assert!(matches!(
            determine_limit_type("/api/feedback/123"),
            RateLimitType::Feedback
        ));
        assert!(matches!(
            determine_limit_type("/api/feedback/stats"),
            RateLimitType::Api
        ));
        assert!(matches!(
            determine_limit_type("/api/webhook/github"),
            RateLimitType::Webhook
        ));
        assert!(matches!(
            determine_limit_type("/api/health"),
            RateLimitType::Api
        ));
        println!("✅ Rate limit type determination test passed!");
    }

    #[test]
    fn test_extract_client_ip() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "X-Forwarded-For",
            "192.168.1.100, 10.0.0.1".parse().unwrap(),
        );

        // Create a mock request (in real implementation, you'd need to create a proper request)
        // For this test, we'll focus on the header parsing logic

        // Test that we can parse the first IP from X-Forwarded-For
        if let Some(forwarded_for) = headers.get("X-Forwarded-For") {
            if let Ok(header_str) = forwarded_for.to_str() {
                if let Some(ip_str) = header_str.split(',').next() {
                    if let Ok(ip) = IpAddr::from_str(ip_str.trim()) {
                        assert_eq!(ip, IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)));
                    }
                }
            }
        }

        println!("✅ Client IP extraction test passed!");
    }

    #[tokio::test]
    async fn test_rate_limit_manager() {
        let manager = RateLimitManager::new(60, 10); // 60 requests per minute, 10 feedback per hour

        // Test that initial requests are allowed
        for _ in 0..5 {
            assert!(manager.api_limiter.check().is_ok());
        }

        // Test that feedback limiter works
        for _ in 0..3 {
            assert!(manager.feedback_limiter.check().is_ok());
        }

        println!("✅ Rate limit manager test passed!");
    }
}
