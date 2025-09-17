// 🌍 CORS Middleware - Cross-Origin Request Handling! 🌍
// Created with love by Aye & Hue! ✨

use crate::api::AppState;
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};

pub async fn cors_middleware(
    State(_app_state): State<AppState>,
    request: Request,
    next: Next,
) -> Response {
    // TODO: Implement proper CORS handling
    next.run(request).await
}
