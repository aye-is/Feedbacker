#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
// 🚢 Welcome to Feedbacker - The AI-Powered Repository Management Service! 🚢
// Created with love by Aye and Hue - Making GitHub PRs as smooth as butter!
// This file is the heart of our service - treat it with care, comment it well,
// and remember: Trisha from Accounting is watching (and she loves good documentation)! 📚✨

use anyhow::{Context, Result};
use axum::{
    http::StatusCode,
    middleware as axum_middleware,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tokio::signal;
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, cors::CorsLayer, trace::TraceLayer};
use tracing::{error, info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// 🎯 Import all our amazing modules that we're about to create!
mod api; // 📡 API routes for feedback submission and management
mod auth; // 🔐 Authentication and authorization magic
mod config; // ⚙️  Configuration management (because settings matter!)
mod database; // 🗄️  Database operations and connections
mod github; // 🐙 GitHub integration for the legendary aye-is user
mod jobs; // 🔄 Background job processing for async operations
mod llm; // 🤖 LLM integration (OpenAI, Anthropic, and friends!)
mod middleware; // 🛡️  Custom middleware for rate limiting and security
mod models; // 📊 Data models and structures
mod utils; // 🔧 Utility functions and helpers

use config::Config;
use middleware::{auth::auth_middleware, rate_limiting::rate_limit_middleware};

// 🎊 The main function - Where the magic begins! 🎊
#[tokio::main]
async fn main() -> Result<()> {
    // 🌈 Initialize our beautiful logging system
    // Because knowing what's happening is half the battle!
    init_logging()?;

    // 🎨 Display our fabulous startup banner
    display_startup_banner();

    // ⚙️ Load configuration from environment and files
    let config = Config::load()
        .context("Failed to load configuration - check your environment variables!")?;

    info!("🚀 Configuration loaded successfully!");
    info!("🎯 Server will listen on: {}", config.server.address);
    info!(
        "🗄️ Database URL: {}",
        mask_database_url(&config.database.url)
    );

    // 🔗 Initialize database connection pool
    let db_pool = database::create_pool(&config.database.url)
        .await
        .context("Failed to create database connection pool")?;

    // 🏃‍♂️ Run database migrations (keeping things up to date!)
    database::run_migrations(&db_pool)
        .await
        .context("Failed to run database migrations")?;

    info!("✅ Database connection established and migrations complete!");

    // 🎯 Create our amazing application state
    let app_state = api::AppState::new(config.clone(), db_pool);

    // 🏗️ Build our beautiful Axum router
    let app = create_router(app_state, &config).context("Failed to create router")?;

    // 🎧 Set up our server address
    let addr: SocketAddr = config
        .server
        .address
        .parse()
        .context("Invalid server address in configuration")?;

    info!("🎉 Starting Feedbacker service on {}", addr);
    info!("🌟 Ready to process feedback and create amazing PRs!");

    // 🚀 Launch the server with graceful shutdown
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .context("Failed to bind to address")?;

    info!("🎊 Feedbacker is now LIVE and ready for action! 🎊");

    // 🛡️ Run the server with graceful shutdown handling
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("Server error occurred")?;

    info!("👋 Feedbacker service shutting down gracefully. Thanks for using our service!");

    Ok(())
}

// 🌈 Initialize our beautiful logging system
// This makes debugging a joy instead of a chore!
fn init_logging() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "feedbacker=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    Ok(())
}

// 🎨 Display our fabulous startup banner
// Because every great service needs a great entrance!
fn display_startup_banner() {
    println!("\n{}", "=".repeat(80));
    println!("🚢 ⚓ FEEDBACKER - AI-Powered Repository Management ⚓ 🚢");
    println!("{}", "=".repeat(80));
    println!("🤖 Built with Rust, Axum, and lots of ❤️  by Aye & Hue");
    println!("📧 Contact: aye@8b.is | hue@8b.is");
    println!("🌟 Making GitHub PRs as smooth as Elvis's dance moves!");
    println!("💝 Special thanks to Trisha from Accounting for keeping us organized!");
    println!("{}", "=".repeat(80));
    println!("🎯 Features:");
    println!("   📡 AI-driven feedback processing");
    println!("   🐙 Automatic GitHub PR creation");
    println!("   🔐 Secure authentication & rate limiting");
    println!("   🤖 Multi-LLM support (OpenAI, Anthropic, etc.)");
    println!("   🎨 Beautiful web interface");
    println!("{}", "=".repeat(80));
    println!();
}

// 🏗️ Create our amazing Axum router with all the bells and whistles
fn create_router(app_state: api::AppState, config: &Config) -> Result<Router> {
    // 🎯 Create the main API router
    let api_router = Router::new()
        // 📝 Feedback submission endpoint - the heart of our service!
        .route("/api/feedback", post(api::feedback::submit_feedback))
        // 📊 Status and health check endpoints
        .route("/api/health", get(api::health::health_check))
        .route(
            "/api/status/:project_id",
            get(api::status::get_project_status),
        )
        // 🔍 Project management endpoints
        .route("/api/projects", get(api::projects::list_projects))
        .route("/api/projects/:id", get(api::projects::get_project))
        // 🐙 GitHub webhook endpoint for status updates
        .route("/api/webhook/github", post(api::webhooks::github_webhook))
        // 🎯 GitHub issue automation webhooks
        .route("/api/webhook/issues", post(api::issue_hooks::github_issue_webhook))
        // 🔧 Manual issue management endpoints
        .route("/api/issues/:owner/:repo/:issue_number/comment", post(api::issue_hooks::add_issue_comment))
        .route("/api/issues/:owner/:repo/:issue_number/labels", post(api::issue_hooks::add_issue_labels))
        .route("/api/issues/:owner/:repo/:issue_number/close", post(api::issue_hooks::close_issue_with_comment))
        // 🤖 Smart Tree integration endpoint
        .route(
            "/api/smart-tree/latest",
            get(api::smart_tree::get_latest_version),
        )
        // 🔐 Authentication endpoints
        .route("/api/auth/login", post(api::auth::login))
        .route("/api/auth/logout", post(api::auth::logout))
        .route("/api/auth/register", post(api::auth::register));

    // 🎨 Create the web UI router for our beautiful interface
    let web_router = Router::new()
        // 🏠 Home page - welcome to Feedbacker!
        .route("/", get(web_home))
        // 📊 Project dashboard
        .route("/projects", get(api::web::projects_page))
        .route("/projects/:id", get(api::web::project_detail_page))
        // 🔐 Authentication pages
        .route("/login", get(api::web::login_page))
        .route("/register", get(api::web::register_page))
        // 📚 Documentation and help
        .route("/docs", get(api::web::docs_page))
        .route("/about", get(api::web::about_page));

    // 🛡️ Apply middleware layers (like adding layers to a delicious cake!)
    let app = Router::new()
        .merge(api_router)
        .merge(web_router)
        .layer(
            ServiceBuilder::new()
                // 📊 Tracing layer for request logging
                .layer(TraceLayer::new_for_http())
                // 🗜️ Compression for faster responses
                .layer(CompressionLayer::new())
                // 🌍 CORS support for web clients
                .layer(CorsLayer::permissive()) // TODO: Make this more restrictive in production
                // 🚦 Rate limiting to prevent abuse
                .layer(axum_middleware::from_fn_with_state(
                    app_state.clone(),
                    rate_limit_middleware,
                ))
                // 🔐 Authentication middleware for protected routes
                .layer(axum_middleware::from_fn_with_state(
                    app_state.clone(),
                    auth_middleware,
                )),
        )
        .with_state(app_state);

    info!("🎉 Router created successfully with all middleware layers!");

    Ok(app)
}

// 🏠 Home page handler - Our beautiful welcome page!
async fn web_home() -> impl IntoResponse {
    Html(
        r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>🚢 Feedbacker - AI-Powered Repository Management</title>
    <style>
        body {
            font-family: 'Courier New', monospace;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            margin: 0;
            padding: 20px;
            min-height: 100vh;
        }
        .container {
            max-width: 800px;
            margin: 0 auto;
            text-align: center;
            background: rgba(255,255,255,0.1);
            padding: 40px;
            border-radius: 15px;
            backdrop-filter: blur(10px);
        }
        h1 {
            font-size: 3em;
            margin-bottom: 20px;
            text-shadow: 2px 2px 4px rgba(0,0,0,0.3);
        }
        .feature {
            margin: 20px 0;
            padding: 15px;
            background: rgba(255,255,255,0.1);
            border-radius: 8px;
        }
        a {
            color: #ffd700;
            text-decoration: none;
            font-weight: bold;
        }
        a:hover {
            text-decoration: underline;
        }
        .button {
            display: inline-block;
            padding: 12px 24px;
            background: #ffd700;
            color: #333;
            text-decoration: none;
            border-radius: 25px;
            margin: 10px;
            transition: transform 0.2s;
        }
        .button:hover {
            transform: translateY(-2px);
            box-shadow: 0 4px 8px rgba(0,0,0,0.3);
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>🚢 Welcome to Feedbacker! ⚓</h1>
        <p style="font-size: 1.2em; margin-bottom: 30px;">
            AI-Powered Repository Management - Making GitHub PRs as smooth as Elvis's dance moves! 🕺
        </p>

        <div class="feature">
            <h3>🤖 AI-Driven Feedback Processing</h3>
            <p>Submit feedback and watch our AI create beautiful, meaningful pull requests automatically!</p>
        </div>

        <div class="feature">
            <h3>🐙 GitHub Integration</h3>
            <p>Seamless integration with GitHub via our dedicated aye-is user account.</p>
        </div>

        <div class="feature">
            <h3>🔐 Secure & Fast</h3>
            <p>Built with Rust for speed and security. Rate limiting and authentication included!</p>
        </div>

        <div style="margin-top: 40px;">
            <a href="/projects" class="button">📊 View Projects</a>
            <a href="/docs" class="button">📚 Documentation</a>
            <a href="/about" class="button">ℹ️ About</a>
        </div>

        <p style="margin-top: 40px; font-size: 0.9em; opacity: 0.8;">
            Built with ❤️ by Aye & Hue | Special thanks to Trisha from Accounting! 📝
        </p>
    </div>
</body>
</html>
    "#,
    )
}

// 🛡️ Graceful shutdown signal handler
// Because even the best services need to shut down gracefully!
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            warn!("🛑 Received Ctrl+C signal, initiating graceful shutdown...");
        },
        _ = terminate => {
            warn!("🛑 Received terminate signal, initiating graceful shutdown...");
        },
    }

    info!("🎉 Shutdown signal received. Cleaning up resources...");
}

// 🔐 Utility function to mask sensitive database URLs in logs
// Because security is important, even in logs!
fn mask_database_url(url: &str) -> String {
    if let Some(at_pos) = url.find('@') {
        if let Some(protocol_end) = url.find("://") {
            let protocol_part = &url[..protocol_end + 3];
            let host_part = &url[at_pos..];
            format!("{}***:***{}", protocol_part, host_part)
        } else {
            "***masked***".to_string()
        }
    } else {
        url.to_string()
    }
}

// 🧪 Tests - Because we test everything like good developers!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_database_url() {
        let url = "postgresql://user:password@localhost:5432/feedbacker";
        let masked = mask_database_url(url);
        assert!(!masked.contains("password"));
        assert!(masked.contains("localhost"));
        println!("✅ Database URL masking works perfectly!");
    }

    #[tokio::test]
    async fn test_logging_initialization() {
        // This test ensures our logging setup doesn't panic
        let result = init_logging();
        assert!(result.is_ok());
        println!("✅ Logging initialization test passed!");
    }
}
