// 🛡️ Middleware Module - The Guardian Angels of Feedbacker! 🛡️
// This module provides security, rate limiting, and request processing middleware
// Built with Axum tower middleware for maximum performance! 🚀
// Created with love by Aye & Hue - Making security beautiful and effective! ✨
// Trisha from Accounting loves when security is both strong and organized! 🔐

pub mod auth; // 🔐 Authentication middleware
pub mod cors; // 🌍 CORS handling middleware
pub mod logging; // 📊 Request logging middleware
pub mod rate_limiting; // 🚦 Rate limiting middleware
pub mod security; // 🛡️ Security headers middleware

// Re-export commonly used middleware functions
pub use auth::auth_middleware;
pub use cors::cors_middleware;
pub use logging::logging_middleware;
pub use rate_limiting::rate_limit_middleware;
pub use security::security_headers_middleware;
