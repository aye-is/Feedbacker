// 🔐 Authentication Middleware - The Security Guardian of Feedbacker! 🔐
// This module provides JWT-based authentication and authorization
// Built with secure defaults and flexible permissions! 🛡️
// Created with love by Aye & Hue - Making security elegant and effective! ✨
// Trisha from Accounting trusts this module to keep everything safe! 🔒

use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::{IntoResponse, Json, Response},
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use tracing::{debug, error, warn};
use uuid::Uuid;

use crate::{
    api::{ApiResponse, AppState},
    database::models::{User, UserRole},
};

/// 🎫 JWT Claims structure
/// Contains all the information we need about an authenticated user
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// 👤 User ID
    pub sub: String, // Subject (user ID)
    /// 📧 User email
    pub email: String,
    /// 👤 User name
    pub name: String,
    /// 👑 User role
    pub role: UserRole,
    /// ⏰ Token expiration time (Unix timestamp)
    pub exp: usize,
    /// ⏰ Token issued at (Unix timestamp)
    pub iat: usize,
    /// 🎯 Token issuer
    pub iss: String,
}

/// 👤 Authenticated user information
/// This is what gets passed to handlers that require authentication
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    /// 🆔 User ID
    pub id: Uuid,
    /// 📧 Email address
    pub email: String,
    /// 👤 Display name
    pub name: String,
    /// 👑 User role
    pub role: UserRole,
    /// 🎫 Original JWT claims (for additional validation if needed)
    pub claims: Claims,
}

impl AuthenticatedUser {
    /// 👑 Check if user has admin privileges
    pub fn is_admin(&self) -> bool {
        matches!(self.role, UserRole::Admin)
    }

    /// 🔧 Check if user is a service account
    pub fn is_service(&self) -> bool {
        matches!(self.role, UserRole::Service)
    }

    /// 🎯 Check if user has specific permission
    pub fn has_permission(&self, permission: Permission) -> bool {
        match permission {
            Permission::ReadFeedback => true, // All authenticated users can read their own feedback
            Permission::SubmitFeedback => true, // All authenticated users can submit feedback
            Permission::ManageProjects => matches!(self.role, UserRole::Admin | UserRole::Service),
            Permission::ViewAllFeedback => matches!(self.role, UserRole::Admin),
            Permission::ManageUsers => matches!(self.role, UserRole::Admin),
            Permission::SystemAdmin => matches!(self.role, UserRole::Admin),
        }
    }
}

/// 🎯 Permission enumeration for fine-grained access control
#[derive(Debug, Clone, PartialEq)]
pub enum Permission {
    /// 👀 Read feedback (own feedback for users, all for admins)
    ReadFeedback,
    /// 📝 Submit new feedback
    SubmitFeedback,
    /// 🏠 Manage projects (create, update, delete)
    ManageProjects,
    /// 📊 View all feedback (admin only)
    ViewAllFeedback,
    /// 👥 Manage users (admin only)
    ManageUsers,
    /// ⚙️ System administration (admin only)
    SystemAdmin,
}

/// 🔐 Main authentication middleware
/// Validates JWT tokens and populates request with user information
pub async fn auth_middleware(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, Response> {
    let path = request.uri().path();

    // 🎯 Check if this path requires authentication
    if is_public_path(path) {
        debug!("✅ Public path accessed: {}", path);
        return Ok(next.run(request).await);
    }

    // 🔍 Extract token from headers
    let token = match extract_token_from_headers(&headers) {
        Some(token) => token,
        None => {
            warn!(
                "🚫 Missing authentication token for protected path: {}",
                path
            );
            return Err(unauthorized_response("Authentication token required"));
        }
    };

    // ✅ Validate the JWT token
    match validate_jwt_token(&token, &app_state.config.auth.jwt_secret).await {
        Ok(claims) => {
            // 🔍 Optionally verify user still exists and is active
            match verify_user_active(&claims, &app_state).await {
                Ok(user) => {
                    debug!(
                        "✅ Authentication successful for user: {} ({})",
                        user.email, user.id
                    );

                    // 🎯 Check permissions for this specific path
                    if let Some(required_permission) = get_required_permission(path) {
                        if !user.has_permission(required_permission) {
                            warn!(
                                "🚫 Insufficient permissions for user {} on path: {}",
                                user.email, path
                            );
                            return Err(forbidden_response("Insufficient permissions"));
                        }
                    }

                    // 📦 Add user to request extensions so handlers can access it
                    request.extensions_mut().insert(user);

                    Ok(next.run(request).await)
                }
                Err(e) => {
                    error!("❌ User verification failed: {:#}", e);
                    Err(unauthorized_response("Invalid user or account disabled"))
                }
            }
        }
        Err(e) => {
            warn!("🚫 JWT validation failed for path {}: {:#}", path, e);
            Err(unauthorized_response("Invalid or expired token"))
        }
    }
}

/// 🔍 Check if a path is public (doesn't require authentication)
fn is_public_path(path: &str) -> bool {
    let public_paths = [
        "/",                      // Home page
        "/api/health",            // Health checks
        "/api/readiness",         // Readiness probe
        "/api/liveness",          // Liveness probe
        "/api/auth/login",        // Login endpoint
        "/api/auth/register",     // Registration endpoint
        "/api/webhook/github",    // GitHub webhooks (authenticated differently)
        "/api/smart-tree/latest", // Smart Tree version check
        "/about",                 // About page
        "/docs",                  // Documentation
        "/login",                 // Login page
        "/register",              // Registration page
    ];

    // 🎯 Check exact matches
    if public_paths.contains(&path) {
        return true;
    }

    // 🎯 Check prefixes for public endpoints
    let public_prefixes = [
        "/static/", // Static assets
        "/assets/", // Assets
        "/favicon", // Favicon
    ];

    public_prefixes
        .iter()
        .any(|prefix| path.starts_with(prefix))
}

/// 🔍 Extract JWT token from request headers
fn extract_token_from_headers(headers: &HeaderMap) -> Option<String> {
    // 🔍 Check Authorization header with Bearer scheme
    if let Some(auth_header) = headers.get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                return Some(auth_str[7..].to_string());
            }
        }
    }

    // 🔍 Check X-API-Key header as fallback
    if let Some(api_key_header) = headers.get("X-API-Key") {
        if let Ok(api_key) = api_key_header.to_str() {
            return Some(api_key.to_string());
        }
    }

    None
}

/// ✅ Validate JWT token and extract claims
async fn validate_jwt_token(token: &str, secret: &str) -> anyhow::Result<Claims> {
    let decoding_key = DecodingKey::from_secret(secret.as_ref());
    let mut validation = Validation::new(Algorithm::HS256);

    // 🎯 Set validation parameters
    validation.set_issuer(&["feedbacker"]);
    validation.validate_exp = true;
    validation.validate_nbf = false; // We don't use "not before"

    let token_data = decode::<Claims>(token, &decoding_key, &validation)
        .map_err(|e| anyhow::anyhow!("JWT validation failed: {}", e))?;

    Ok(token_data.claims)
}

/// 🔍 Verify that the user still exists and is active
async fn verify_user_active(
    claims: &Claims,
    app_state: &AppState,
) -> anyhow::Result<AuthenticatedUser> {
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|e| anyhow::anyhow!("Invalid user ID in token: {}", e))?;

    // TODO: Implement proper user verification when database is ready
    let user: Option<User> = None;

    match user {
        Some(user) => {
            // ✅ User exists and is active
            Ok(AuthenticatedUser {
                id: user.id,
                email: user.email,
                name: user.name,
                role: user.role,
                claims: claims.clone(),
            })
        }
        None => {
            anyhow::bail!("User not found or inactive");
        }
    }
}

/// 🎯 Get required permission for a specific path
fn get_required_permission(path: &str) -> Option<Permission> {
    // 🗺️ Map paths to required permissions
    if path.starts_with("/api/admin/") {
        return Some(Permission::SystemAdmin);
    }

    if path.starts_with("/api/users/") && path != "/api/users/me" {
        return Some(Permission::ManageUsers);
    }

    if path.starts_with("/api/projects/") && !path.contains("/feedback") {
        return Some(Permission::ManageProjects);
    }

    if path == "/api/feedback/all" {
        return Some(Permission::ViewAllFeedback);
    }

    // 📝 Most feedback endpoints just require basic authentication
    if path.starts_with("/api/feedback/") {
        return Some(Permission::ReadFeedback);
    }

    // 🎯 Default: no specific permission required beyond authentication
    None
}

/// 🚫 Create unauthorized error response
fn unauthorized_response(message: &str) -> Response {
    let error_response =
        ApiResponse::<()>::error("unauthorized".to_string(), message.to_string(), None);

    (StatusCode::UNAUTHORIZED, Json(error_response)).into_response()
}

/// 🛡️ Create forbidden error response
fn forbidden_response(message: &str) -> Response {
    let error_response =
        ApiResponse::<()>::error("forbidden".to_string(), message.to_string(), None);

    (StatusCode::FORBIDDEN, Json(error_response)).into_response()
}

/// 🎫 JWT token creation utilities (for the auth module to use)
pub mod jwt_utils {
    use super::*;
    use jsonwebtoken::{encode, EncodingKey, Header};

    /// ➕ Create a new JWT token for a user
    pub fn create_jwt_token(
        user: &User,
        secret: &str,
        expiration_hours: u64,
    ) -> anyhow::Result<String> {
        let now = chrono::Utc::now();
        let exp = (now + chrono::Duration::hours(expiration_hours as i64)).timestamp() as usize;
        let iat = now.timestamp() as usize;

        let claims = Claims {
            sub: user.id.to_string(),
            email: user.email.clone(),
            name: user.name.clone(),
            role: user.role.clone(),
            exp,
            iat,
            iss: "feedbacker".to_string(),
        };

        let header = Header::new(Algorithm::HS256);
        let encoding_key = EncodingKey::from_secret(secret.as_ref());

        encode(&header, &claims, &encoding_key)
            .map_err(|e| anyhow::anyhow!("Failed to create JWT token: {}", e))
    }

    /// 🔄 Refresh a JWT token (create a new one with extended expiration)
    pub fn refresh_jwt_token(
        claims: &Claims,
        secret: &str,
        expiration_hours: u64,
    ) -> anyhow::Result<String> {
        let now = chrono::Utc::now();
        let exp = (now + chrono::Duration::hours(expiration_hours as i64)).timestamp() as usize;

        let new_claims = Claims {
            sub: claims.sub.clone(),
            email: claims.email.clone(),
            name: claims.name.clone(),
            role: claims.role.clone(),
            exp,
            iat: now.timestamp() as usize,
            iss: "feedbacker".to_string(),
        };

        let header = Header::new(Algorithm::HS256);
        let encoding_key = EncodingKey::from_secret(secret.as_ref());

        encode(&header, &new_claims, &encoding_key)
            .map_err(|e| anyhow::anyhow!("Failed to refresh JWT token: {}", e))
    }
}

// 🧪 Tests - Because authentication security needs thorough testing!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_public_path() {
        assert!(is_public_path("/"));
        assert!(is_public_path("/api/health"));
        assert!(is_public_path("/api/auth/login"));
        assert!(is_public_path("/static/css/style.css"));
        assert!(is_public_path("/favicon.ico"));

        assert!(!is_public_path("/api/feedback"));
        assert!(!is_public_path("/api/projects"));
        assert!(!is_public_path("/dashboard"));
        println!("✅ Public path detection test passed!");
    }

    #[test]
    fn test_extract_token_from_headers() {
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", "Bearer abc123xyz".parse().unwrap());

        let token = extract_token_from_headers(&headers);
        assert_eq!(token, Some("abc123xyz".to_string()));

        // Test X-API-Key fallback
        let mut headers2 = HeaderMap::new();
        headers2.insert("X-API-Key", "api_key_123".parse().unwrap());

        let token2 = extract_token_from_headers(&headers2);
        assert_eq!(token2, Some("api_key_123".to_string()));

        println!("✅ Token extraction test passed!");
    }

    #[test]
    fn test_permission_checking() {
        let admin_user = AuthenticatedUser {
            id: Uuid::new_v4(),
            email: "admin@example.com".to_string(),
            name: "Admin User".to_string(),
            role: UserRole::Admin,
            claims: Claims {
                sub: "123".to_string(),
                email: "admin@example.com".to_string(),
                name: "Admin User".to_string(),
                role: UserRole::Admin,
                exp: 0,
                iat: 0,
                iss: "feedbacker".to_string(),
            },
        };

        assert!(admin_user.has_permission(Permission::SystemAdmin));
        assert!(admin_user.has_permission(Permission::ViewAllFeedback));
        assert!(admin_user.has_permission(Permission::ManageUsers));

        let regular_user = AuthenticatedUser {
            id: Uuid::new_v4(),
            email: "user@example.com".to_string(),
            name: "Regular User".to_string(),
            role: UserRole::User,
            claims: Claims {
                sub: "456".to_string(),
                email: "user@example.com".to_string(),
                name: "Regular User".to_string(),
                role: UserRole::User,
                exp: 0,
                iat: 0,
                iss: "feedbacker".to_string(),
            },
        };

        assert!(!regular_user.has_permission(Permission::SystemAdmin));
        assert!(!regular_user.has_permission(Permission::ViewAllFeedback));
        assert!(regular_user.has_permission(Permission::SubmitFeedback));

        println!("✅ Permission checking test passed!");
    }

    #[test]
    fn test_get_required_permission() {
        assert_eq!(
            get_required_permission("/api/admin/settings"),
            Some(Permission::SystemAdmin)
        );
        assert_eq!(
            get_required_permission("/api/users/123"),
            Some(Permission::ManageUsers)
        );
        assert_eq!(get_required_permission("/api/users/me"), None);
        assert_eq!(
            get_required_permission("/api/projects/create"),
            Some(Permission::ManageProjects)
        );
        assert_eq!(
            get_required_permission("/api/feedback/all"),
            Some(Permission::ViewAllFeedback)
        );
        assert_eq!(
            get_required_permission("/api/feedback/123"),
            Some(Permission::ReadFeedback)
        );

        println!("✅ Required permission mapping test passed!");
    }
}
