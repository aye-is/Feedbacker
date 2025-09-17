// 📊 Logging Middleware - Request Tracking! 📊
// Created with love by Aye & Hue! ✨

use crate::api::AppState;
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};

pub async fn logging_middleware(
    State(_app_state): State<AppState>,
    request: Request,
    next: Next,
) -> Response {
    // TODO: Implement request logging
    next.run(request).await
}
