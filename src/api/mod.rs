// 📡 API Module - The Communication Hub of Feedbacker! 📡
// This module handles all HTTP API endpoints and responses
// Built with Axum for blazing fast, type-safe web APIs! 🚀
// Created with love by Aye & Hue - Making APIs beautiful and functional! ✨
// Trisha from Accounting loves well-organized API endpoints! 📊

use axum::extract::State;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::config::Config;

// 📦 Re-export all our API modules
pub mod auth; // 🔐 Authentication endpoints
pub mod feedback; // 📝 Feedback submission and management
pub mod health; // 💚 Health check endpoints
pub mod issue_hooks; // 🎯 GitHub issue automation
pub mod projects; // 🏠 Project management endpoints
pub mod smart_tree; // 🌳 Smart Tree integration
pub mod status; // 📊 Status checking endpoints
pub mod web; // 🎨 Web UI endpoints
pub mod webhooks; // 🪝 GitHub webhook handlers

/// 🎯 Application state shared across all handlers
/// This contains everything our API endpoints need to function!
#[derive(Debug, Clone)]
pub struct AppState {
    /// ⚙️ Application configuration
    pub config: Arc<Config>,
    /// 🗄️ Database connection pool
    pub db_pool: PgPool,
    // 🤖 LLM client manager (will be added when we create LLM module)
    // pub llm_manager: Arc<crate::llm::LlmManager>,
    // 🐙 GitHub client (will be added when we create GitHub module)
    // pub github_client: Arc<crate::github::GitHubClient>,
}

impl AppState {
    /// ➕ Create a new application state instance
    pub fn new(config: Config, db_pool: PgPool) -> Self {
        Self {
            config: Arc::new(config),
            db_pool,
            // These will be uncommented when we create the respective modules
            // llm_manager: Arc::new(crate::llm::LlmManager::new(&config.llm)),
            // github_client: Arc::new(crate::github::GitHubClient::new(&config.github)),
        }
    }
}

/// 📝 Standard API response structure
/// Provides consistent response format across all endpoints
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// ✅ Whether the operation was successful
    pub success: bool,
    /// 📝 Human-readable message
    pub message: String,
    /// 📊 Response data (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    /// ❌ Error details (only present if success = false)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiError>,
    /// ⏰ Response timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// ❌ API error structure
/// Provides structured error information for debugging and user feedback
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    /// 🎯 Error code for programmatic handling
    pub code: String,
    /// 📝 Human-readable error message
    pub message: String,
    /// 🔍 Additional error details (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

impl<T> ApiResponse<T> {
    /// ✅ Create a successful response
    pub fn success(message: String, data: T) -> Self {
        Self {
            success: true,
            message,
            data: Some(data),
            error: None,
            timestamp: chrono::Utc::now(),
        }
    }

    /// ✅ Create a successful response without data
    pub fn success_no_data(message: String) -> ApiResponse<()> {
        ApiResponse {
            success: true,
            message,
            data: None,
            error: None,
            timestamp: chrono::Utc::now(),
        }
    }

    /// ❌ Create an error response
    pub fn error(
        code: String,
        message: String,
        details: Option<serde_json::Value>,
    ) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            message: "Operation failed".to_string(),
            data: None,
            error: Some(ApiError {
                code,
                message,
                details,
            }),
            timestamp: chrono::Utc::now(),
        }
    }
}

/// 📋 Pagination parameters for list endpoints
#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    /// 📄 Page number (1-based)
    #[serde(default = "default_page")]
    pub page: u32,
    /// 📏 Items per page (max 100)
    #[serde(default = "default_limit")]
    pub limit: u32,
    /// 🔍 Sort field
    pub sort_by: Option<String>,
    /// ⬆️⬇️ Sort order (asc/desc)
    #[serde(default = "default_sort_order")]
    pub sort_order: SortOrder,
}

/// ⬆️⬇️ Sort order enumeration
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Asc,
    Desc,
}

/// 📊 Paginated response structure
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    /// 📋 The actual data items
    pub items: Vec<T>,
    /// 📊 Pagination metadata
    pub pagination: PaginationMeta,
}

/// 📊 Pagination metadata
#[derive(Debug, Serialize)]
pub struct PaginationMeta {
    /// 📄 Current page number
    pub page: u32,
    /// 📏 Items per page
    pub limit: u32,
    /// 📈 Total number of items
    pub total: u64,
    /// 📑 Total number of pages
    pub total_pages: u32,
    /// ⬅️ Has previous page
    pub has_prev: bool,
    /// ➡️ Has next page
    pub has_next: bool,
}

impl PaginationMeta {
    /// ➕ Create pagination metadata
    pub fn new(page: u32, limit: u32, total: u64) -> Self {
        let total_pages = ((total as f64) / (limit as f64)).ceil() as u32;
        let has_prev = page > 1;
        let has_next = page < total_pages;

        Self {
            page,
            limit,
            total,
            total_pages,
            has_prev,
            has_next,
        }
    }
}

impl<T> PaginatedResponse<T> {
    /// ➕ Create a paginated response
    pub fn new(items: Vec<T>, page: u32, limit: u32, total: u64) -> Self {
        Self {
            items,
            pagination: PaginationMeta::new(page, limit, total),
        }
    }
}

/// 🔧 Default values for pagination
fn default_page() -> u32 {
    1
}
fn default_limit() -> u32 {
    20
}
fn default_sort_order() -> SortOrder {
    SortOrder::Desc
}

impl PaginationParams {
    /// ✅ Validate and normalize pagination parameters
    pub fn validate(mut self) -> Self {
        // 📏 Limit the maximum items per page
        if self.limit > 100 {
            self.limit = 100;
        }
        if self.limit == 0 {
            self.limit = 20;
        }

        // 📄 Ensure page is at least 1
        if self.page == 0 {
            self.page = 1;
        }

        self
    }

    /// 📊 Calculate the SQL OFFSET value
    pub fn offset(&self) -> u32 {
        (self.page - 1) * self.limit
    }
}

/// 📝 Common request validation trait
pub trait ValidateRequest {
    /// ✅ Validate the request and return any errors
    fn validate(&self) -> Result<(), Vec<String>>;
}

/// 🔧 Common utility functions for API handlers
pub mod utils {
    use super::*;
    use axum::{
        http::StatusCode,
        response::{IntoResponse, Json},
    };

    /// 🎯 Convert an anyhow error to an API error response
    pub fn handle_error(error: anyhow::Error) -> impl IntoResponse {
        let error_msg = format!("{:#}", error);
        tracing::error!("API error: {}", error_msg);

        let api_response = ApiResponse::<()>::error(
            "internal_error".to_string(),
            "An internal error occurred".to_string(),
            Some(serde_json::json!({ "details": error_msg })),
        );

        (StatusCode::INTERNAL_SERVER_ERROR, Json(api_response))
    }

    /// ✅ Create a validation error response
    pub fn validation_error(errors: Vec<String>) -> impl IntoResponse {
        let api_response = ApiResponse::<()>::error(
            "validation_error".to_string(),
            "Request validation failed".to_string(),
            Some(serde_json::json!({ "errors": errors })),
        );

        (StatusCode::BAD_REQUEST, Json(api_response))
    }

    /// 🔍 Create a not found error response
    pub fn not_found_error(resource: &str) -> impl IntoResponse {
        let api_response = ApiResponse::<()>::error(
            "not_found".to_string(),
            format!("{} not found", resource),
            None,
        );

        (StatusCode::NOT_FOUND, Json(api_response))
    }

    /// 🚫 Create an unauthorized error response
    pub fn unauthorized_error() -> impl IntoResponse {
        let api_response = ApiResponse::<()>::error(
            "unauthorized".to_string(),
            "Authentication required".to_string(),
            None,
        );

        (StatusCode::UNAUTHORIZED, Json(api_response))
    }

    /// 🛡️ Create a forbidden error response
    pub fn forbidden_error() -> impl IntoResponse {
        let api_response =
            ApiResponse::<()>::error("forbidden".to_string(), "Access denied".to_string(), None);

        (StatusCode::FORBIDDEN, Json(api_response))
    }

    /// 🚦 Create a rate limit error response
    pub fn rate_limit_error() -> impl IntoResponse {
        let api_response = ApiResponse::<()>::error(
            "rate_limit_exceeded".to_string(),
            "Rate limit exceeded. Please try again later.".to_string(),
            None,
        );

        (StatusCode::TOO_MANY_REQUESTS, Json(api_response))
    }
}

// 🧪 Tests - Because we test our API structures thoroughly!
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_api_response_success() {
        let response = ApiResponse::success(
            "Operation completed successfully".to_string(),
            "test_data".to_string(),
        );

        assert!(response.success);
        assert_eq!(response.message, "Operation completed successfully");
        assert_eq!(response.data.unwrap(), "test_data");
        assert!(response.error.is_none());
        println!("✅ API response success test passed!");
    }

    #[test]
    fn test_api_response_error() {
        let response = ApiResponse::<()>::error(
            "test_error".to_string(),
            "Test error message".to_string(),
            Some(serde_json::json!({"detail": "more info"})),
        );

        assert!(!response.success);
        assert!(response.data.is_none());
        assert!(response.error.is_some());

        let error = response.error.unwrap();
        assert_eq!(error.code, "test_error");
        assert_eq!(error.message, "Test error message");
        println!("✅ API response error test passed!");
    }

    #[test]
    fn test_pagination_params_validation() {
        let params = PaginationParams {
            page: 0,
            limit: 150,
            sort_by: None,
            sort_order: SortOrder::Asc,
        };

        let validated = params.validate();
        assert_eq!(validated.page, 1); // Should be corrected to 1
        assert_eq!(validated.limit, 100); // Should be capped at 100
        println!("✅ Pagination validation test passed!");
    }

    #[test]
    fn test_pagination_offset_calculation() {
        let params = PaginationParams {
            page: 3,
            limit: 20,
            sort_by: None,
            sort_order: SortOrder::Desc,
        };

        assert_eq!(params.offset(), 40); // (3-1) * 20 = 40
        println!("✅ Pagination offset calculation test passed!");
    }

    #[test]
    fn test_pagination_meta() {
        let meta = PaginationMeta::new(2, 10, 45);

        assert_eq!(meta.page, 2);
        assert_eq!(meta.limit, 10);
        assert_eq!(meta.total, 45);
        assert_eq!(meta.total_pages, 5); // ceil(45/10) = 5
        assert!(meta.has_prev);
        assert!(meta.has_next);
        println!("✅ Pagination metadata test passed!");
    }
}
