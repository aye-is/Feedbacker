// 🪝 Webhooks API - GitHub Integration Events! 🪝
// This module handles GitHub webhook endpoints
// Created with love by Aye & Hue! ✨

use crate::api::{ApiResponse, AppState};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GitHubWebhookPayload {
    pub action: String,
    pub repository: serde_json::Value,
    pub pull_request: Option<serde_json::Value>,
}

pub async fn github_webhook(
    State(_app_state): State<AppState>,
    Json(_payload): Json<GitHubWebhookPayload>,
) -> impl IntoResponse {
    // TODO: Implement GitHub webhook processing
    (
        StatusCode::OK,
        Json(ApiResponse::<()>::success_no_data(  // 🔧 Added explicit type annotation
            "Webhook processed".to_string(),
        )),
    )
}
