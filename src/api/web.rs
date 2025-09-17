// 🎨 Web UI API - Beautiful Web Interface! 🎨
// This module provides web UI endpoints for the Feedbacker interface
// Created with love by Aye & Hue! ✨

use crate::api::AppState;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};

pub async fn projects_page(State(_app_state): State<AppState>) -> impl IntoResponse {
    Html("<h1>🏠 Projects Dashboard</h1><p>Coming soon...</p>")
}

pub async fn project_detail_page(State(_app_state): State<AppState>) -> impl IntoResponse {
    Html("<h1>📊 Project Details</h1><p>Coming soon...</p>")
}

pub async fn login_page(State(_app_state): State<AppState>) -> impl IntoResponse {
    Html("<h1>🔐 Login</h1><p>Coming soon...</p>")
}

pub async fn register_page(State(_app_state): State<AppState>) -> impl IntoResponse {
    Html("<h1>📝 Register</h1><p>Coming soon...</p>")
}

pub async fn docs_page(State(_app_state): State<AppState>) -> impl IntoResponse {
    Html("<h1>📚 Documentation</h1><p>Coming soon...</p>")
}

pub async fn about_page(State(_app_state): State<AppState>) -> impl IntoResponse {
    Html("<h1>ℹ️ About Feedbacker</h1><p>AI-powered repository management by Aye & Hue!</p>")
}
