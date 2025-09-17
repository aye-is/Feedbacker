// 🌳 Smart Tree API - Integration Endpoints! 🌳
// This module provides Smart Tree MCP integration endpoints
// Created with love by Aye & Hue! ✨

use crate::api::{ApiResponse, AppState};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct VersionInfo {
    pub version: String,
    pub download_url: String,
    pub release_notes: String,
}

pub async fn get_latest_version(State(_app_state): State<AppState>) -> impl IntoResponse {
    let version_info = VersionInfo {
        version: "1.0.0".to_string(),
        download_url: "https://github.com/aye-is/smart-tree/releases/latest".to_string(),
        release_notes: "Latest Smart Tree MCP release".to_string(),
    };

    (
        StatusCode::OK,
        Json(ApiResponse::success(
            "Version info retrieved".to_string(),
            version_info,
        )),
    )
}
