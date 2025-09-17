// 📊 Database Models - The Data Structures of Feedbacker! 📊
// This module defines all our database models and their relationships
// Built with SQLx and Serde for type safety and serialization magic! ✨
// Created with love by Aye & Hue - Making data beautiful and organized! 🎨
// Trisha from Accounting says these are the most organized models she's ever seen! 📋

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

// 📝 Feedback Model - The heart of our system!
// This represents user feedback that gets processed into GitHub PRs
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Feedback {
    /// 🆔 Unique identifier for this feedback
    pub id: Uuid,
    /// 👤 User who submitted the feedback
    pub user_id: Option<Uuid>,
    /// 🎯 Target repository (format: "owner/repo")
    pub repository: String,
    /// 📝 The actual feedback content
    pub content: String,
    /// 📋 Current status of the feedback processing
    pub status: FeedbackStatus,
    /// 🌿 GitHub branch name (if created)
    pub branch_name: Option<String>,
    /// 🔗 Pull request URL (if created)
    pub pull_request_url: Option<String>,
    /// 🤖 LLM provider used for processing
    pub llm_provider: Option<String>,
    /// 📊 Processing metadata (JSON)
    pub metadata: Option<serde_json::Value>,
    /// ❌ Error message (if processing failed)
    pub error_message: Option<String>,
    /// ⏰ When this feedback was submitted
    pub created_at: DateTime<Utc>,
    /// 🔄 When this feedback was last updated
    pub updated_at: DateTime<Utc>,
    /// ✅ When processing was completed (if applicable)
    pub completed_at: Option<DateTime<Utc>>,
}

// 📋 Feedback Status Enum - Track where we are in the process!
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "feedback_status", rename_all = "lowercase")]
pub enum FeedbackStatus {
    /// 📥 Just received, waiting for processing
    Pending,
    /// 🔄 Currently being processed by AI
    Processing,
    /// 🤖 AI analysis complete, creating GitHub changes
    GeneratingChanges,
    /// 🐙 Creating branch and pull request
    CreatingPullRequest,
    /// ✅ Successfully completed with PR created
    Completed,
    /// ❌ Failed during processing
    Failed,
    /// ⏸️ Paused (waiting for user input or manual intervention)
    Paused,
}

// 👤 User Model - Our amazing users who provide feedback!
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    /// 🆔 Unique identifier for this user
    pub id: Uuid,
    /// 📧 User's email address
    pub email: String,
    /// 👤 User's display name
    pub name: String,
    /// 🐙 GitHub username (optional)
    pub github_username: Option<String>,
    /// 🔐 Hashed password
    pub password_hash: String,
    /// ✅ Whether the user's email is verified
    pub email_verified: bool,
    /// 👑 User role (admin, user, etc.)
    pub role: UserRole,
    /// 🚫 Whether the user account is active
    pub is_active: bool,
    /// ⏰ When the user account was created
    pub created_at: DateTime<Utc>,
    /// 🔄 When the user account was last updated
    pub updated_at: DateTime<Utc>,
    /// 🕒 When the user last logged in
    pub last_login_at: Option<DateTime<Utc>>,
}

// 👑 User Role Enum - Different levels of access
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    /// 🎯 Regular user
    User,
    /// 🛠️ Administrator with extra privileges
    Admin,
    /// 🔧 Service account for automation
    Service,
}

// 🏠 Project Model - GitHub repositories we manage
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Project {
    /// 🆔 Unique identifier for this project
    pub id: Uuid,
    /// 👤 Owner of the project
    pub owner_id: Uuid,
    /// 📦 Repository name (format: "owner/repo")
    pub repository: String,
    /// 📝 Project description
    pub description: Option<String>,
    /// 🤖 Default LLM provider for this project
    pub default_llm_provider: Option<String>,
    /// 💬 Custom system message for AI processing
    pub system_message: Option<String>,
    /// ⚙️ Project configuration (JSON)
    pub config: Option<serde_json::Value>,
    /// 🚫 Whether the project is active
    pub is_active: bool,
    /// ⏰ When the project was registered
    pub created_at: DateTime<Utc>,
    /// 🔄 When the project was last updated
    pub updated_at: DateTime<Utc>,
    /// 🕒 When we last interacted with this project
    pub last_activity_at: Option<DateTime<Utc>>,
}

// 🎫 User Session Model - Track user sessions securely
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserSession {
    /// 🆔 Unique session identifier
    pub id: Uuid,
    /// 👤 User this session belongs to
    pub user_id: Uuid,
    /// 🔑 JWT token (hashed)
    pub token_hash: String,
    /// 🌐 User's IP address
    pub ip_address: Option<String>,
    /// 🖥️ User agent string
    pub user_agent: Option<String>,
    /// ⏰ When the session was created
    pub created_at: DateTime<Utc>,
    /// ⏰ When the session expires
    pub expires_at: DateTime<Utc>,
    /// 🕒 When the session was last used
    pub last_used_at: DateTime<Utc>,
}

// 🚦 Rate Limit Model - Prevent abuse and ensure fair usage
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RateLimit {
    /// 🆔 Unique identifier (usually IP address or user ID)
    pub id: String,
    /// 🎯 Type of rate limit (api, feedback, etc.)
    pub limit_type: String,
    /// 📊 Current request count
    pub request_count: i32,
    /// ⏰ When the limit window started
    pub window_start: DateTime<Utc>,
    /// 🕒 When the last request was made
    pub last_request: DateTime<Utc>,
}

// 🔔 Notification Model - Keep users informed
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Notification {
    /// 🆔 Unique identifier for this notification
    pub id: Uuid,
    /// 👤 User this notification is for
    pub user_id: Uuid,
    /// 📋 Type of notification
    pub notification_type: NotificationType,
    /// 📝 Notification title
    pub title: String,
    /// 📄 Notification content
    pub content: String,
    /// 🔗 Related entity ID (feedback, project, etc.)
    pub related_id: Option<Uuid>,
    /// 👀 Whether the user has read this notification
    pub is_read: bool,
    /// ⏰ When the notification was created
    pub created_at: DateTime<Utc>,
    /// 👀 When the notification was read
    pub read_at: Option<DateTime<Utc>>,
}

// 🔔 Notification Type Enum
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "notification_type", rename_all = "lowercase")]
pub enum NotificationType {
    /// ✅ Feedback processing completed
    FeedbackCompleted,
    /// ❌ Feedback processing failed
    FeedbackFailed,
    /// 🐙 Pull request created
    PullRequestCreated,
    /// 🔄 System update
    SystemUpdate,
    /// ⚠️ Warning or important notice
    Warning,
}

// 🏭 Implementation blocks for our models
impl Feedback {
    /// ➕ Create a new feedback record
    pub async fn create(
        pool: &PgPool,
        user_id: Option<Uuid>,
        repository: String,
        content: String,
    ) -> Result<Self> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        // TODO: Implement proper query when database is set up
        // For now, return a placeholder feedback object
        let feedback = Feedback {
            id,
            user_id,
            repository,
            content,
            status: FeedbackStatus::Pending,
            branch_name: None,
            pull_request_url: None,
            llm_provider: None,
            metadata: None,
            error_message: None,
            created_at: now,
            updated_at: now,
            completed_at: None,
        };

        Ok(feedback)
    }

    /// 🔍 Find feedback by ID
    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>> {
        // TODO: Implement proper query when database is set up
        let feedback: Option<Feedback> = None;

        Ok(feedback)
    }

    /// 🔄 Update feedback status
    pub async fn update_status(
        &mut self,
        pool: &PgPool,
        status: FeedbackStatus,
        error_message: Option<String>,
    ) -> Result<()> {
        let now = Utc::now();
        let completed_at = if matches!(status, FeedbackStatus::Completed | FeedbackStatus::Failed) {
            Some(now)
        } else {
            None
        };

        // TODO: Implement proper query when database is set up

        self.status = status;
        self.error_message = error_message;
        self.updated_at = now;
        self.completed_at = completed_at;

        Ok(())
    }

    /// 📊 Get feedback statistics for a user
    pub async fn get_user_stats(pool: &PgPool, user_id: Uuid) -> Result<FeedbackStats> {
        // TODO: Implement proper query when database is set up
        Ok(FeedbackStats {
            total: 0,
            pending: 0,
            processing: 0,
            completed: 0,
            failed: 0,
        })
    }
}

// 📊 Feedback Statistics Structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackStats {
    pub total: u32,
    pub pending: u32,
    pub processing: u32,
    pub completed: u32,
    pub failed: u32,
}

impl User {
    /// ➕ Create a new user
    pub async fn create(
        pool: &PgPool,
        email: String,
        name: String,
        password_hash: String,
    ) -> Result<Self> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        // TODO: Implement proper query when database is set up
        let user = User {
            id,
            email,
            name,
            github_username: None,
            password_hash,
            email_verified: false,
            role: UserRole::User,
            is_active: true,
            created_at: now,
            updated_at: now,
            last_login_at: None,
        };

        Ok(user)
    }

    /// 🔍 Find user by email
    pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<Self>> {
        // TODO: Implement proper query when database is set up
        let user: Option<User> = None;

        Ok(user)
    }
}

impl Project {
    /// ➕ Create a new project
    pub async fn create(
        pool: &PgPool,
        owner_id: Uuid,
        repository: String,
        description: Option<String>,
    ) -> Result<Self> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        // TODO: Implement proper query when database is set up
        let project = Project {
            id,
            owner_id,
            repository,
            description,
            default_llm_provider: None,
            system_message: None,
            config: None,
            is_active: true,
            created_at: now,
            updated_at: now,
            last_activity_at: None,
        };

        Ok(project)
    }
}

// 🧪 Tests - Making sure our models work perfectly!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feedback_status_serialization() {
        let status = FeedbackStatus::Processing;
        let serialized = serde_json::to_string(&status).unwrap();
        assert_eq!(serialized, "\"processing\"");
        println!("✅ Feedback status serialization test passed!");
    }

    #[test]
    fn test_user_role_serialization() {
        let role = UserRole::Admin;
        let serialized = serde_json::to_string(&role).unwrap();
        assert_eq!(serialized, "\"admin\"");
        println!("✅ User role serialization test passed!");
    }

    #[test]
    fn test_feedback_stats() {
        let stats = FeedbackStats {
            total: 10,
            pending: 2,
            processing: 1,
            completed: 6,
            failed: 1,
        };

        assert_eq!(stats.total, 10);
        assert_eq!(
            stats.pending + stats.processing + stats.completed + stats.failed,
            10
        );
        println!("✅ Feedback stats test passed!");
    }
}
