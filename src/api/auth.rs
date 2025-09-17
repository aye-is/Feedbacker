// 🔐 Authentication API - Securing Feedbacker with Style! 🔐
// This module handles user authentication, registration, and session management
// Built with JWT tokens and secure password hashing! 🛡️
// Created with love by Aye & Hue - Making security beautiful and user-friendly! ✨

use anyhow::{Context, Result};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};

use crate::{
    api::{
        utils::{handle_error, validation_error},
        ApiResponse, AppState, ValidateRequest,
    },
    database::models::{User, UserRole},
};

/// 🔐 User login request
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// 📝 User registration request
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub name: String,
    pub password: String,
    pub github_username: Option<String>,
}

/// 🎫 Authentication response with token
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub user: UserInfo,
    pub token: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

/// 👤 User information for responses
#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: uuid::Uuid,
    pub email: String,
    pub name: String,
    pub github_username: Option<String>,
    pub role: UserRole,
    pub email_verified: bool,
}

impl ValidateRequest for LoginRequest {
    fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.email.is_empty() || !self.email.contains('@') {
            errors.push("Valid email is required".to_string());
        }

        if self.password.is_empty() {
            errors.push("Password is required".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl ValidateRequest for RegisterRequest {
    fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.email.is_empty() || !self.email.contains('@') {
            errors.push("Valid email is required".to_string());
        }

        if self.name.trim().is_empty() {
            errors.push("Name is required".to_string());
        }

        if self.password.len() < 8 {
            errors.push("Password must be at least 8 characters".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

/// 🔐 User login endpoint
pub async fn login(
    State(app_state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Response {
    info!("🔐 Login attempt for email: {}", request.email);

    if let Err(errors) = request.validate() {
        let api_response = ApiResponse::<()>::error(
            "validation_error".to_string(),
            "Request validation failed".to_string(),
            Some(serde_json::json!({ "errors": errors })),
        );
        return (StatusCode::BAD_REQUEST, Json(api_response)).into_response();
    }

    match authenticate_user(&app_state, request).await {
        Ok(response) => {
            info!("✅ Login successful for user: {}", response.user.email);
            (
                StatusCode::OK,
                Json(ApiResponse::<AuthResponse>::success(
                    "Login successful".to_string(),
                    response,
                )),
            ).into_response()
        }
        Err(e) => {
            warn!("❌ Login failed: {:#}", e);
            let error_msg = format!("{:#}", e);
            let api_response = ApiResponse::<()>::error(
                "internal_error".to_string(),
                "An internal error occurred".to_string(),
                Some(serde_json::json!({ "details": error_msg })),
            );
            (StatusCode::INTERNAL_SERVER_ERROR, Json(api_response)).into_response()
        }
    }
}

/// 📝 User registration endpoint
pub async fn register(
    State(app_state): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> Response {
    info!("📝 Registration attempt for email: {}", request.email);

    if let Err(errors) = request.validate() {
        let api_response = ApiResponse::<()>::error(
            "validation_error".to_string(),
            "Request validation failed".to_string(),
            Some(serde_json::json!({ "errors": errors })),
        );
        return (StatusCode::BAD_REQUEST, Json(api_response)).into_response();
    }

    match create_user_account(&app_state, request).await {
        Ok(response) => {
            info!(
                "✅ Registration successful for user: {}",
                response.user.email
            );
            (
                StatusCode::CREATED,
                Json(ApiResponse::<AuthResponse>::success(
                    "Registration successful".to_string(),
                    response,
                )),
            ).into_response()
        }
        Err(e) => {
            warn!("❌ Registration failed: {:#}", e);
            let error_msg = format!("{:#}", e);
            let api_response = ApiResponse::<()>::error(
                "internal_error".to_string(),
                "An internal error occurred".to_string(),
                Some(serde_json::json!({ "details": error_msg })),
            );
            (StatusCode::INTERNAL_SERVER_ERROR, Json(api_response)).into_response()
        }
    }
}

/// 🚪 User logout endpoint
pub async fn logout(State(_app_state): State<AppState>) -> impl IntoResponse {
    info!("🚪 User logout requested");

    // TODO: Implement token invalidation when session management is ready
    (
        StatusCode::OK,
        Json(ApiResponse::<()>::success_no_data(
            "Logout successful".to_string(),
        )),
    )
}

// Helper functions

async fn authenticate_user(app_state: &AppState, request: LoginRequest) -> Result<AuthResponse> {
    // TODO: Implement actual authentication logic
    anyhow::bail!("Authentication not implemented yet")
}

async fn create_user_account(
    app_state: &AppState,
    request: RegisterRequest,
) -> Result<AuthResponse> {
    // TODO: Implement actual user creation logic
    anyhow::bail!("User registration not implemented yet")
}
