// 🛡️ Security Headers Middleware - Protection Headers! 🛡️
// Created with love by Aye & Hue! ✨

use crate::api::AppState;
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};

pub async fn security_headers_middleware(
    State(_app_state): State<AppState>,
    request: Request,
    next: Next,
) -> Response {
    // TODO: Implement security headers
    next.run(request).await
}
