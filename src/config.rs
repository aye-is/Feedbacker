// ⚙️ Configuration Management for Feedbacker! ⚙️
// This module handles all our settings, environment variables, and configuration files
// Built with love by Aye & Hue - Making configuration as easy as pie! 🥧
// Trisha from Accounting loves organized settings, so we made this EXTRA organized! 📊

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::env;
use std::path::Path;

// 🎯 Main configuration structure - The heart of our settings!
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// 🌐 Server configuration
    pub server: ServerConfig,
    /// 🗄️ Database configuration
    pub database: DatabaseConfig,
    /// 🐙 GitHub integration settings
    pub github: GitHubConfig,
    /// 🤖 LLM provider configurations
    pub llm: LlmConfig,
    /// 🔐 Authentication settings
    pub auth: AuthConfig,
    /// 🚦 Rate limiting configuration
    pub rate_limiting: RateLimitConfig,
    /// 📧 Email notification settings (optional)
    pub email: Option<EmailConfig>,
    /// 📊 Logging configuration
    pub logging: LoggingConfig,
    /// 🔧 Feature flags and toggles
    pub features: FeaturesConfig,
}

// 🌐 Server configuration - Where we listen and how we behave
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// 🎯 Address to bind the server to (e.g., "127.0.0.1:3000")
    pub address: String,
    /// 🕒 Request timeout in seconds
    pub timeout_seconds: u64,
    /// 📏 Maximum request body size in bytes
    pub max_body_size: usize,
    /// 🌍 Environment (development, staging, production)
    pub environment: Environment,
}

// 🗄️ Database configuration - Our data storage settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// 🔗 Database connection URL
    pub url: String,
    /// 🏊‍♂️ Maximum number of connections in the pool
    pub max_connections: u32,
    /// ⏱️ Connection timeout in seconds
    pub connection_timeout_seconds: u64,
    /// 🔄 Enable automatic migrations
    pub auto_migrate: bool,
}

// 🐙 GitHub configuration - Settings for the legendary aye-is user!
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubConfig {
    /// 🤖 GitHub username (should be "aye-is")
    pub username: String,
    /// 🔑 GitHub personal access token
    pub token: String,
    /// 🔐 SSH private key path for git operations
    pub ssh_private_key_path: String,
    /// 🏠 Base URL for GitHub API (for GitHub Enterprise)
    pub api_base_url: String,
    /// 📝 Default commit message template
    pub default_commit_message: String,
    /// 🌿 Default branch name for new branches
    pub default_branch_prefix: String,
}

// 🤖 LLM configuration - Settings for all our AI friends!
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    /// 🧠 OpenAI configuration
    pub openai: Option<OpenAiConfig>,
    /// 🎭 Anthropic configuration
    pub anthropic: Option<AnthropicConfig>,
    /// 🔄 Default provider to use
    pub default_provider: LlmProvider,
    /// ⏱️ Request timeout in seconds
    pub timeout_seconds: u64,
    /// 🔄 Maximum retry attempts
    pub max_retries: u32,
}

// 🧠 OpenAI specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAiConfig {
    /// 🔑 API key for OpenAI
    pub api_key: String,
    /// 🤖 Default model to use (e.g., "gpt-4", "gpt-3.5-turbo")
    pub default_model: String,
    /// 🌡️ Temperature for responses (0.0 to 2.0)
    pub temperature: f32,
    /// 📏 Maximum tokens in response
    pub max_tokens: u32,
}

// 🎭 Anthropic specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicConfig {
    /// 🔑 API key for Anthropic
    pub api_key: String,
    /// 🤖 Default model to use (e.g., "claude-3-sonnet-20240229")
    pub default_model: String,
    /// 📏 Maximum tokens in response
    pub max_tokens: u32,
}

// 🔐 Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// 🔐 JWT secret key for token signing
    pub jwt_secret: String,
    /// ⏱️ JWT token expiration time in hours
    pub token_expiration_hours: u64,
    /// 🧂 Password salt rounds for hashing
    pub password_salt_rounds: u32,
    /// 🔄 Enable user registration
    pub enable_registration: bool,
}

// 🚦 Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// 📊 Requests per minute for general API
    pub requests_per_minute: u32,
    /// 📝 Feedback submissions per hour
    pub feedback_per_hour: u32,
    /// 🎯 Burst size for rate limiting
    pub burst_size: u32,
    /// ⏱️ Rate limit window in seconds
    pub window_seconds: u64,
}

// 📧 Email configuration (optional feature)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailConfig {
    /// 📮 SMTP server hostname
    pub smtp_host: String,
    /// 🚪 SMTP server port
    pub smtp_port: u16,
    /// 👤 SMTP username
    pub smtp_username: String,
    /// 🔑 SMTP password
    pub smtp_password: String,
    /// 📧 From email address
    pub from_email: String,
    /// 🔒 Use TLS encryption
    pub use_tls: bool,
}

// 📊 Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// 📈 Log level (trace, debug, info, warn, error)
    pub level: String,
    /// 📄 Log format (json, pretty, compact)
    pub format: String,
    /// 📁 Log file path (optional)
    pub file_path: Option<String>,
    /// 🔄 Enable request logging
    pub log_requests: bool,
}

// 🔧 Feature flags configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeaturesConfig {
    /// 🔄 Enable background job processing
    pub enable_background_jobs: bool,
    /// 📧 Enable email notifications
    pub enable_email_notifications: bool,
    /// 🎨 Enable web UI
    pub enable_web_ui: bool,
    /// 🐙 Enable GitHub webhooks
    pub enable_github_webhooks: bool,
    /// 📊 Enable metrics collection
    pub enable_metrics: bool,
    /// 🧪 Enable development features
    pub enable_dev_features: bool,
}

// 🌍 Environment enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Development,
    Staging,
    Production,
}

// 🤖 LLM provider enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LlmProvider {
    OpenAi,
    Anthropic,
}

impl Config {
    /// 🚀 Load configuration from environment variables and files
    /// This is the main entry point for configuration loading!
    pub fn load() -> Result<Self> {
        // 🔍 Load environment variables from .env file if it exists
        if Path::new(".env").exists() {
            dotenv::dotenv().context("Failed to load .env file")?;
        }

        // 🏗️ Build configuration from environment variables
        let config = Self {
            server: ServerConfig::load()?,
            database: DatabaseConfig::load()?,
            github: GitHubConfig::load()?,
            llm: LlmConfig::load()?,
            auth: AuthConfig::load()?,
            rate_limiting: RateLimitConfig::load()?,
            email: EmailConfig::load_optional(),
            logging: LoggingConfig::load()?,
            features: FeaturesConfig::load()?,
        };

        // ✅ Validate the configuration
        config.validate()?;

        Ok(config)
    }

    /// ✅ Validate the configuration for any issues
    fn validate(&self) -> Result<()> {
        // 🔍 Check required fields and reasonable values
        if self.server.address.is_empty() {
            anyhow::bail!("Server address cannot be empty");
        }

        if self.database.url.is_empty() {
            anyhow::bail!("Database URL cannot be empty");
        }

        if self.github.token.is_empty() {
            anyhow::bail!("GitHub token cannot be empty");
        }

        if self.auth.jwt_secret.len() < 32 {
            anyhow::bail!("JWT secret must be at least 32 characters long");
        }

        // 🎯 Validate rate limiting values
        if self.rate_limiting.requests_per_minute == 0 {
            anyhow::bail!("Rate limiting requests per minute must be greater than 0");
        }

        // ✅ All validations passed!
        Ok(())
    }

    /// 🌍 Check if we're running in development mode
    pub fn is_development(&self) -> bool {
        self.server.environment == Environment::Development
    }

    /// 🏭 Check if we're running in production mode
    pub fn is_production(&self) -> bool {
        self.server.environment == Environment::Production
    }
}

impl ServerConfig {
    fn load() -> Result<Self> {
        Ok(Self {
            address: env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:3000".to_string()),
            timeout_seconds: env::var("SERVER_TIMEOUT_SECONDS")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .context("Invalid SERVER_TIMEOUT_SECONDS")?,
            max_body_size: env::var("SERVER_MAX_BODY_SIZE")
                .unwrap_or_else(|_| "1048576".to_string()) // 1MB default
                .parse()
                .context("Invalid SERVER_MAX_BODY_SIZE")?,
            environment: env::var("ENVIRONMENT")
                .unwrap_or_else(|_| "development".to_string())
                .parse()
                .unwrap_or(Environment::Development),
        })
    }
}

impl DatabaseConfig {
    fn load() -> Result<Self> {
        Ok(Self {
            url: env::var("DATABASE_URL")
                .context("DATABASE_URL environment variable is required")?,
            max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .context("Invalid DATABASE_MAX_CONNECTIONS")?,
            connection_timeout_seconds: env::var("DATABASE_CONNECTION_TIMEOUT_SECONDS")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .context("Invalid DATABASE_CONNECTION_TIMEOUT_SECONDS")?,
            auto_migrate: env::var("DATABASE_AUTO_MIGRATE")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .context("Invalid DATABASE_AUTO_MIGRATE")?,
        })
    }
}

impl GitHubConfig {
    fn load() -> Result<Self> {
        Ok(Self {
            username: env::var("GITHUB_USERNAME")
                .unwrap_or_else(|_| "aye-is".to_string()),
            token: env::var("GITHUB_TOKEN")
                .context("GITHUB_TOKEN environment variable is required")?,
            ssh_private_key_path: env::var("GITHUB_SSH_PRIVATE_KEY_PATH")
                .unwrap_or_else(|_| "~/.ssh/id_rsa".to_string()),
            api_base_url: env::var("GITHUB_API_BASE_URL")
                .unwrap_or_else(|_| "https://api.github.com".to_string()),
            default_commit_message: env::var("GITHUB_DEFAULT_COMMIT_MESSAGE")
                .unwrap_or_else(|_| "🤖 AI-generated improvement based on user feedback\n\n✨ Generated by Feedbacker with love by Aye & Hue".to_string()),
            default_branch_prefix: env::var("GITHUB_DEFAULT_BRANCH_PREFIX")
                .unwrap_or_else(|_| "feedbacker/".to_string()),
        })
    }
}

impl LlmConfig {
    fn load() -> Result<Self> {
        Ok(Self {
            openai: OpenAiConfig::load_optional(),
            anthropic: AnthropicConfig::load_optional(),
            default_provider: env::var("LLM_DEFAULT_PROVIDER")
                .unwrap_or_else(|_| "openai".to_string())
                .parse()
                .unwrap_or(LlmProvider::OpenAi),
            timeout_seconds: env::var("LLM_TIMEOUT_SECONDS")
                .unwrap_or_else(|_| "60".to_string())
                .parse()
                .context("Invalid LLM_TIMEOUT_SECONDS")?,
            max_retries: env::var("LLM_MAX_RETRIES")
                .unwrap_or_else(|_| "3".to_string())
                .parse()
                .context("Invalid LLM_MAX_RETRIES")?,
        })
    }
}

impl OpenAiConfig {
    fn load_optional() -> Option<Self> {
        env::var("OPENAI_API_KEY").ok().map(|api_key| Self {
            api_key,
            default_model: env::var("OPENAI_DEFAULT_MODEL").unwrap_or_else(|_| "gpt-4".to_string()),
            temperature: env::var("OPENAI_TEMPERATURE")
                .unwrap_or_else(|_| "0.7".to_string())
                .parse()
                .unwrap_or(0.7),
            max_tokens: env::var("OPENAI_MAX_TOKENS")
                .unwrap_or_else(|_| "2000".to_string())
                .parse()
                .unwrap_or(2000),
        })
    }
}

impl AnthropicConfig {
    fn load_optional() -> Option<Self> {
        env::var("ANTHROPIC_API_KEY").ok().map(|api_key| Self {
            api_key,
            default_model: env::var("ANTHROPIC_DEFAULT_MODEL")
                .unwrap_or_else(|_| "claude-3-sonnet-20240229".to_string()),
            max_tokens: env::var("ANTHROPIC_MAX_TOKENS")
                .unwrap_or_else(|_| "2000".to_string())
                .parse()
                .unwrap_or(2000),
        })
    }
}

impl AuthConfig {
    fn load() -> Result<Self> {
        Ok(Self {
            jwt_secret: env::var("JWT_SECRET")
                .context("JWT_SECRET environment variable is required")?,
            token_expiration_hours: env::var("JWT_TOKEN_EXPIRATION_HOURS")
                .unwrap_or_else(|_| "24".to_string())
                .parse()
                .context("Invalid JWT_TOKEN_EXPIRATION_HOURS")?,
            password_salt_rounds: env::var("PASSWORD_SALT_ROUNDS")
                .unwrap_or_else(|_| "12".to_string())
                .parse()
                .context("Invalid PASSWORD_SALT_ROUNDS")?,
            enable_registration: env::var("ENABLE_REGISTRATION")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .context("Invalid ENABLE_REGISTRATION")?,
        })
    }
}

impl RateLimitConfig {
    fn load() -> Result<Self> {
        Ok(Self {
            requests_per_minute: env::var("RATE_LIMIT_REQUESTS_PER_MINUTE")
                .unwrap_or_else(|_| "60".to_string())
                .parse()
                .context("Invalid RATE_LIMIT_REQUESTS_PER_MINUTE")?,
            feedback_per_hour: env::var("RATE_LIMIT_FEEDBACK_PER_HOUR")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .context("Invalid RATE_LIMIT_FEEDBACK_PER_HOUR")?,
            burst_size: env::var("RATE_LIMIT_BURST_SIZE")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .context("Invalid RATE_LIMIT_BURST_SIZE")?,
            window_seconds: env::var("RATE_LIMIT_WINDOW_SECONDS")
                .unwrap_or_else(|_| "60".to_string())
                .parse()
                .context("Invalid RATE_LIMIT_WINDOW_SECONDS")?,
        })
    }
}

impl EmailConfig {
    fn load_optional() -> Option<Self> {
        let smtp_host = env::var("SMTP_HOST").ok()?;
        Some(Self {
            smtp_host,
            smtp_port: env::var("SMTP_PORT")
                .unwrap_or_else(|_| "587".to_string())
                .parse()
                .unwrap_or(587),
            smtp_username: env::var("SMTP_USERNAME").unwrap_or_default(),
            smtp_password: env::var("SMTP_PASSWORD").unwrap_or_default(),
            from_email: env::var("FROM_EMAIL")
                .unwrap_or_else(|_| "noreply@feedbacker.com".to_string()),
            use_tls: env::var("SMTP_USE_TLS")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
        })
    }
}

impl LoggingConfig {
    fn load() -> Result<Self> {
        Ok(Self {
            level: env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
            format: env::var("LOG_FORMAT").unwrap_or_else(|_| "pretty".to_string()),
            file_path: env::var("LOG_FILE_PATH").ok(),
            log_requests: env::var("LOG_REQUESTS")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .context("Invalid LOG_REQUESTS")?,
        })
    }
}

impl FeaturesConfig {
    fn load() -> Result<Self> {
        Ok(Self {
            enable_background_jobs: env::var("ENABLE_BACKGROUND_JOBS")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .context("Invalid ENABLE_BACKGROUND_JOBS")?,
            enable_email_notifications: env::var("ENABLE_EMAIL_NOTIFICATIONS")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .context("Invalid ENABLE_EMAIL_NOTIFICATIONS")?,
            enable_web_ui: env::var("ENABLE_WEB_UI")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .context("Invalid ENABLE_WEB_UI")?,
            enable_github_webhooks: env::var("ENABLE_GITHUB_WEBHOOKS")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .context("Invalid ENABLE_GITHUB_WEBHOOKS")?,
            enable_metrics: env::var("ENABLE_METRICS")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .context("Invalid ENABLE_METRICS")?,
            enable_dev_features: env::var("ENABLE_DEV_FEATURES")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .context("Invalid ENABLE_DEV_FEATURES")?,
        })
    }
}

// 🎯 Implement string parsing for enums
impl std::str::FromStr for Environment {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "development" | "dev" => Ok(Environment::Development),
            "staging" | "stage" => Ok(Environment::Staging),
            "production" | "prod" => Ok(Environment::Production),
            _ => anyhow::bail!("Invalid environment: {}", s),
        }
    }
}

impl std::str::FromStr for LlmProvider {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "openai" | "openai-gpt" => Ok(LlmProvider::OpenAi),
            "anthropic" | "claude" => Ok(LlmProvider::Anthropic),
            _ => anyhow::bail!("Invalid LLM provider: {}", s),
        }
    }
}

// 🧪 Tests - Because Trisha loves when we test our configuration!
#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_environment_parsing() {
        assert_eq!(
            "development".parse::<Environment>().unwrap(),
            Environment::Development
        );
        assert_eq!(
            "staging".parse::<Environment>().unwrap(),
            Environment::Staging
        );
        assert_eq!(
            "production".parse::<Environment>().unwrap(),
            Environment::Production
        );
        println!("✅ Environment parsing test passed!");
    }

    #[test]
    fn test_llm_provider_parsing() {
        assert_eq!(
            "openai".parse::<LlmProvider>().unwrap(),
            LlmProvider::OpenAi
        );
        assert_eq!(
            "anthropic".parse::<LlmProvider>().unwrap(),
            LlmProvider::Anthropic
        );
        println!("✅ LLM provider parsing test passed!");
    }

    #[test]
    fn test_config_validation() {
        // Set up minimal required environment variables for testing
        env::set_var("DATABASE_URL", "postgresql://test:test@localhost/test");
        env::set_var("GITHUB_TOKEN", "test_token");
        env::set_var(
            "JWT_SECRET",
            "this_is_a_very_long_secret_key_for_testing_purposes",
        );

        let config = Config::load();
        assert!(
            config.is_ok(),
            "Config loading should succeed with valid environment"
        );
        println!("✅ Configuration validation test passed!");
    }
}
