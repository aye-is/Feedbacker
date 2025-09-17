// 💚 Health Check API - Monitoring the Heartbeat of Feedbacker! 💚
// This module provides health check endpoints for monitoring and load balancers
// Built for reliability and observability - because uptime matters! 🚀
// Created with love by Aye & Hue - Making sure our service stays healthy! ✨
// Trisha from Accounting loves when services report their health clearly! 📊

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use tracing::{error, info, warn};

use crate::{
    api::{ApiResponse, AppState},
    database::get_pool_stats,
};

/// 💚 Basic health check response
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    /// ✅ Overall service status
    pub status: HealthStatus,
    /// ⏰ Current timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// 🕒 Service uptime in seconds
    pub uptime_seconds: u64,
    /// 📊 Version information
    pub version: String,
    /// 🌍 Environment (development, staging, production)
    pub environment: String,
}

/// 🏥 Detailed health check response with component status
#[derive(Debug, Serialize)]
pub struct DetailedHealthResponse {
    /// ✅ Overall service status
    pub status: HealthStatus,
    /// ⏰ Current timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// 🕒 Service uptime in seconds
    pub uptime_seconds: u64,
    /// 📊 Version information
    pub version: String,
    /// 🌍 Environment
    pub environment: String,
    /// 🔧 Component health status
    pub components: ComponentHealth,
    /// 📈 Performance metrics
    pub metrics: PerformanceMetrics,
}

/// ✅ Health status enumeration
#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    /// 💚 Everything is working perfectly
    Healthy,
    /// 🟡 Some components have issues but service is functional
    Degraded,
    /// 🔴 Critical components are down
    Unhealthy,
}

/// 🔧 Component health status
#[derive(Debug, Serialize)]
pub struct ComponentHealth {
    /// 🗄️ Database connectivity and performance
    pub database: ComponentStatus,
    /// 🤖 LLM providers availability
    pub llm_providers: LlmProvidersHealth,
    /// 🐙 GitHub API connectivity
    pub github_api: ComponentStatus,
    /// 📧 Email service (if enabled)
    pub email_service: Option<ComponentStatus>,
    /// 🔄 Background job processor
    pub background_jobs: ComponentStatus,
}

/// 🔧 Individual component status
#[derive(Debug, Serialize)]
pub struct ComponentStatus {
    /// ✅ Component health status
    pub status: HealthStatus,
    /// ⏱️ Response time in milliseconds
    pub response_time_ms: Option<u64>,
    /// 💬 Status message
    pub message: String,
    /// 🕒 Last checked timestamp
    pub last_checked: chrono::DateTime<chrono::Utc>,
}

/// 🤖 LLM providers health status
#[derive(Debug, Serialize)]
pub struct LlmProvidersHealth {
    /// 🧠 OpenAI API status
    pub openai: Option<ComponentStatus>,
    /// 🎭 Anthropic API status
    pub anthropic: Option<ComponentStatus>,
}

/// 📈 Performance metrics
#[derive(Debug, Serialize)]
pub struct PerformanceMetrics {
    /// 🗄️ Database connection pool statistics
    pub database_pool: DatabasePoolMetrics,
    /// 💾 Memory usage information
    pub memory: MemoryMetrics,
    /// 📊 Request statistics (if available)
    pub requests: Option<RequestMetrics>,
}

/// 🗄️ Database pool metrics
#[derive(Debug, Serialize)]
pub struct DatabasePoolMetrics {
    /// 📊 Total connections in pool
    pub total_connections: u32,
    /// 💤 Idle connections
    pub idle_connections: u32,
    /// 🏃‍♂️ Active connections
    pub active_connections: u32,
    /// ✅ Pool health status
    pub is_healthy: bool,
}

/// 💾 Memory usage metrics
#[derive(Debug, Serialize)]
pub struct MemoryMetrics {
    /// 📊 Used memory in bytes
    pub used_bytes: u64,
    /// 📈 Peak memory usage in bytes
    pub peak_bytes: u64,
    /// 💾 Available memory in bytes (if available)
    pub available_bytes: Option<u64>,
}

/// 📊 Request metrics
#[derive(Debug, Serialize)]
pub struct RequestMetrics {
    /// 📈 Requests per minute (last minute)
    pub requests_per_minute: u32,
    /// ⏱️ Average response time in milliseconds
    pub avg_response_time_ms: f64,
    /// 🎯 Success rate percentage
    pub success_rate_percent: f64,
}

// 🌟 Global service start time for uptime calculation
lazy_static::lazy_static! {
    static ref SERVICE_START_TIME: Instant = Instant::now();
}

/// 💚 Basic health check endpoint
/// Perfect for load balancers and simple monitoring!
pub async fn health_check(State(app_state): State<AppState>) -> impl IntoResponse {
    info!("💚 Basic health check requested");

    let uptime = SERVICE_START_TIME.elapsed();
    let database_healthy = check_database_health(&app_state).await;

    let status = if database_healthy {
        HealthStatus::Healthy
    } else {
        HealthStatus::Unhealthy
    };

    let response = HealthResponse {
        status,
        timestamp: chrono::Utc::now(),
        uptime_seconds: uptime.as_secs(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        environment: app_state.config.server.environment.to_string(),
    };

    let status_code = match response.status {
        HealthStatus::Healthy => StatusCode::OK,
        HealthStatus::Degraded => StatusCode::OK,
        HealthStatus::Unhealthy => StatusCode::SERVICE_UNAVAILABLE,
    };

    info!("💚 Health check completed - Status: {:?}", response.status);

    (
        status_code,
        Json(ApiResponse::success(
            "Health check completed".to_string(),
            response,
        )),
    )
}

/// 🏥 Detailed health check endpoint
/// Provides comprehensive health information for detailed monitoring!
pub async fn detailed_health_check(State(app_state): State<AppState>) -> impl IntoResponse {
    info!("🏥 Detailed health check requested");

    let uptime = SERVICE_START_TIME.elapsed();
    let components = check_all_components(&app_state).await;
    let metrics = collect_performance_metrics(&app_state).await;

    // 🎯 Determine overall status based on components
    let overall_status = determine_overall_status(&components);

    let response = DetailedHealthResponse {
        status: overall_status,
        timestamp: chrono::Utc::now(),
        uptime_seconds: uptime.as_secs(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        environment: app_state.config.server.environment.to_string(),
        components,
        metrics,
    };

    let status_code = match response.status {
        HealthStatus::Healthy => StatusCode::OK,
        HealthStatus::Degraded => StatusCode::OK,
        HealthStatus::Unhealthy => StatusCode::SERVICE_UNAVAILABLE,
    };

    info!(
        "🏥 Detailed health check completed - Status: {:?}",
        response.status
    );

    (
        status_code,
        Json(ApiResponse::success(
            "Detailed health check completed".to_string(),
            response,
        )),
    )
}

/// 🔄 Readiness probe endpoint
/// Kubernetes-style readiness probe for deployment orchestration
pub async fn readiness_probe(State(app_state): State<AppState>) -> impl IntoResponse {
    info!("🔄 Readiness probe requested");

    let database_ready = check_database_health(&app_state).await;

    if database_ready {
        info!("🔄 Service is ready");
        (
            StatusCode::OK,
            Json(serde_json::json!({
                "status": "ready",
                "timestamp": chrono::Utc::now()
            })),
        )
    } else {
        warn!("🔄 Service is not ready - database unavailable");
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "status": "not_ready",
                "reason": "database_unavailable",
                "timestamp": chrono::Utc::now()
            })),
        )
    }
}

/// 🔥 Liveness probe endpoint
/// Kubernetes-style liveness probe for container health
pub async fn liveness_probe() -> impl IntoResponse {
    info!("🔥 Liveness probe requested");

    // 🎯 Simple liveness check - if we can respond, we're alive!
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "status": "alive",
            "timestamp": chrono::Utc::now(),
            "uptime_seconds": SERVICE_START_TIME.elapsed().as_secs()
        })),
    )
}

// 🔧 Helper functions for health checks

/// 🗄️ Check database health
async fn check_database_health(app_state: &AppState) -> bool {
    match crate::database::check_connection_health(&app_state.db_pool).await {
        Ok(healthy) => healthy,
        Err(e) => {
            error!("❌ Database health check failed: {:#}", e);
            false
        }
    }
}

/// 🔧 Check all system components
async fn check_all_components(app_state: &AppState) -> ComponentHealth {
    let now = chrono::Utc::now();

    // 🗄️ Database health check
    let database_start = Instant::now();
    let database_healthy = check_database_health(app_state).await;
    let database_response_time = database_start.elapsed();

    let database = ComponentStatus {
        status: if database_healthy {
            HealthStatus::Healthy
        } else {
            HealthStatus::Unhealthy
        },
        response_time_ms: Some(database_response_time.as_millis() as u64),
        message: if database_healthy {
            "Database connection is healthy".to_string()
        } else {
            "Database connection failed".to_string()
        },
        last_checked: now,
    };

    // 🤖 LLM providers health check
    let llm_providers = LlmProvidersHealth {
        openai: check_openai_health(app_state).await,
        anthropic: check_anthropic_health(app_state).await,
    };

    // 🐙 GitHub API health check
    let github_api = check_github_health(app_state).await;

    // 📧 Email service health check (if enabled)
    let email_service = if app_state.config.email.is_some() {
        Some(check_email_health(app_state).await)
    } else {
        None
    };

    // 🔄 Background jobs health check
    let background_jobs = check_background_jobs_health(app_state).await;

    ComponentHealth {
        database,
        llm_providers,
        github_api,
        email_service,
        background_jobs,
    }
}

/// 🧠 Check OpenAI API health
async fn check_openai_health(app_state: &AppState) -> Option<ComponentStatus> {
    if app_state.config.llm.openai.is_none() {
        return None;
    }

    let now = chrono::Utc::now();

    // TODO: Implement actual OpenAI health check when LLM module is ready
    // For now, return a placeholder
    Some(ComponentStatus {
        status: HealthStatus::Healthy,
        response_time_ms: Some(150),
        message: "OpenAI API connection not implemented yet".to_string(),
        last_checked: now,
    })
}

/// 🎭 Check Anthropic API health
async fn check_anthropic_health(app_state: &AppState) -> Option<ComponentStatus> {
    if app_state.config.llm.anthropic.is_none() {
        return None;
    }

    let now = chrono::Utc::now();

    // TODO: Implement actual Anthropic health check when LLM module is ready
    Some(ComponentStatus {
        status: HealthStatus::Healthy,
        response_time_ms: Some(200),
        message: "Anthropic API connection not implemented yet".to_string(),
        last_checked: now,
    })
}

/// 🐙 Check GitHub API health
async fn check_github_health(app_state: &AppState) -> ComponentStatus {
    let now = chrono::Utc::now();

    // TODO: Implement actual GitHub health check when GitHub module is ready
    ComponentStatus {
        status: HealthStatus::Healthy,
        response_time_ms: Some(100),
        message: "GitHub API connection not implemented yet".to_string(),
        last_checked: now,
    }
}

/// 📧 Check email service health
async fn check_email_health(app_state: &AppState) -> ComponentStatus {
    let now = chrono::Utc::now();

    // TODO: Implement actual email health check when email module is ready
    ComponentStatus {
        status: HealthStatus::Healthy,
        response_time_ms: Some(50),
        message: "Email service not implemented yet".to_string(),
        last_checked: now,
    }
}

/// 🔄 Check background jobs health
async fn check_background_jobs_health(app_state: &AppState) -> ComponentStatus {
    let now = chrono::Utc::now();

    // TODO: Implement actual background jobs health check when jobs module is ready
    ComponentStatus {
        status: HealthStatus::Healthy,
        response_time_ms: Some(25),
        message: "Background jobs processor not implemented yet".to_string(),
        last_checked: now,
    }
}

/// 📈 Collect performance metrics
async fn collect_performance_metrics(app_state: &AppState) -> PerformanceMetrics {
    let pool_stats = get_pool_stats(&app_state.db_pool);

    let database_pool = DatabasePoolMetrics {
        total_connections: pool_stats.size,
        idle_connections: pool_stats.idle,
        active_connections: pool_stats.active(),
        is_healthy: pool_stats.is_healthy(),
    };

    let memory = MemoryMetrics {
        used_bytes: get_memory_usage(),
        peak_bytes: get_peak_memory_usage(),
        available_bytes: get_available_memory(),
    };

    PerformanceMetrics {
        database_pool,
        memory,
        requests: None, // TODO: Implement request metrics
    }
}

/// 🎯 Determine overall status from component health
fn determine_overall_status(components: &ComponentHealth) -> HealthStatus {
    let mut critical_unhealthy = false;
    let mut degraded = false;

    // 🗄️ Database is critical
    if components.database.status == HealthStatus::Unhealthy {
        critical_unhealthy = true;
    } else if components.database.status == HealthStatus::Degraded {
        degraded = true;
    }

    // 🐙 GitHub API is critical
    if components.github_api.status == HealthStatus::Unhealthy {
        critical_unhealthy = true;
    } else if components.github_api.status == HealthStatus::Degraded {
        degraded = true;
    }

    // 🤖 LLM providers - if all are unhealthy, that's critical
    let llm_all_unhealthy = components
        .llm_providers
        .openai
        .as_ref()
        .map(|s| s.status == HealthStatus::Unhealthy)
        .unwrap_or(true)
        && components
            .llm_providers
            .anthropic
            .as_ref()
            .map(|s| s.status == HealthStatus::Unhealthy)
            .unwrap_or(true);

    if llm_all_unhealthy {
        critical_unhealthy = true;
    }

    if critical_unhealthy {
        HealthStatus::Unhealthy
    } else if degraded {
        HealthStatus::Degraded
    } else {
        HealthStatus::Healthy
    }
}

/// 💾 Get current memory usage (placeholder implementation)
fn get_memory_usage() -> u64 {
    // TODO: Implement actual memory usage tracking
    // For now, return a placeholder value
    1024 * 1024 * 64 // 64 MB
}

/// 📈 Get peak memory usage (placeholder implementation)
fn get_peak_memory_usage() -> u64 {
    // TODO: Implement actual peak memory tracking
    1024 * 1024 * 128 // 128 MB
}

/// 💾 Get available memory (placeholder implementation)
fn get_available_memory() -> Option<u64> {
    // TODO: Implement actual available memory detection
    Some(1024 * 1024 * 1024 * 2) // 2 GB
}

impl ToString for crate::config::Environment {
    fn to_string(&self) -> String {
        match self {
            crate::config::Environment::Development => "development".to_string(),
            crate::config::Environment::Staging => "staging".to_string(),
            crate::config::Environment::Production => "production".to_string(),
        }
    }
}

// 🧪 Tests - Because healthy services need healthy tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_status_serialization() {
        let healthy = HealthStatus::Healthy;
        let serialized = serde_json::to_string(&healthy).unwrap();
        assert_eq!(serialized, "\"healthy\"");

        let unhealthy = HealthStatus::Unhealthy;
        let serialized = serde_json::to_string(&unhealthy).unwrap();
        assert_eq!(serialized, "\"unhealthy\"");

        println!("✅ Health status serialization test passed!");
    }

    #[test]
    fn test_overall_status_determination() {
        let healthy_components = ComponentHealth {
            database: ComponentStatus {
                status: HealthStatus::Healthy,
                response_time_ms: Some(50),
                message: "OK".to_string(),
                last_checked: chrono::Utc::now(),
            },
            llm_providers: LlmProvidersHealth {
                openai: Some(ComponentStatus {
                    status: HealthStatus::Healthy,
                    response_time_ms: Some(100),
                    message: "OK".to_string(),
                    last_checked: chrono::Utc::now(),
                }),
                anthropic: None,
            },
            github_api: ComponentStatus {
                status: HealthStatus::Healthy,
                response_time_ms: Some(75),
                message: "OK".to_string(),
                last_checked: chrono::Utc::now(),
            },
            email_service: None,
            background_jobs: ComponentStatus {
                status: HealthStatus::Healthy,
                response_time_ms: Some(25),
                message: "OK".to_string(),
                last_checked: chrono::Utc::now(),
            },
        };

        let overall = determine_overall_status(&healthy_components);
        assert_eq!(overall, HealthStatus::Healthy);
        println!("✅ Overall status determination test passed!");
    }

    #[test]
    fn test_memory_metrics() {
        let memory = MemoryMetrics {
            used_bytes: 1024 * 1024 * 64,
            peak_bytes: 1024 * 1024 * 128,
            available_bytes: Some(1024 * 1024 * 1024),
        };

        let serialized = serde_json::to_string(&memory);
        assert!(serialized.is_ok());
        println!("✅ Memory metrics test passed!");
    }
}
