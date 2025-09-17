// 🏃‍♂️ Database Migrations - Keeping Our Schema Up to Date! 🏃‍♂️
// This module handles all database schema changes and migrations
// Built with SQLx for safe, transactional schema updates! 🔒
// Created with love by Aye & Hue - Making database evolution smooth as butter! ✨
// Trisha from Accounting loves organized, versioned schema changes! 📋

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use tracing::{info, warn};

/// 📋 Migration structure - Represents a single database migration
#[derive(Debug, Clone)]
pub struct Migration {
    /// 🆔 Unique migration identifier (timestamp + name)
    pub id: String,
    /// 📝 Human-readable description
    pub description: String,
    /// ⬆️ SQL to apply the migration
    pub up_sql: String,
    /// ⬇️ SQL to rollback the migration (optional)
    pub down_sql: Option<String>,
}

/// 📋 Create the migrations tracking table
/// This table keeps track of which migrations have been applied
pub async fn create_migrations_table(pool: &PgPool) -> Result<()> {
    info!("📋 Creating migrations tracking table...");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS migrations (
            id VARCHAR(255) PRIMARY KEY,
            description TEXT NOT NULL,
            applied_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            checksum VARCHAR(64) NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await
    .context("Failed to create migrations table")?;

    info!("✅ Migrations tracking table created successfully!");
    Ok(())
}

/// 🏃‍♂️ Run all pending migrations
/// This applies any migrations that haven't been run yet
pub async fn run_all_migrations(pool: &PgPool) -> Result<()> {
    info!("🚀 Starting migration process...");

    let migrations = get_all_migrations();
    let applied_migrations = get_applied_migrations(pool).await?;

    let mut applied_count = 0;

    for migration in migrations {
        if !applied_migrations.contains(&migration.id) {
            info!(
                "📝 Applying migration: {} - {}",
                migration.id, migration.description
            );
            apply_migration(pool, &migration)
                .await
                .with_context(|| format!("Failed to apply migration {}", migration.id))?;
            applied_count += 1;
        } else {
            info!("⏭️ Skipping already applied migration: {}", migration.id);
        }
    }

    if applied_count > 0 {
        info!("✅ Applied {} new migrations successfully!", applied_count);
    } else {
        info!("✅ Database schema is up to date - no migrations needed!");
    }

    Ok(())
}

/// 📝 Apply a single migration
async fn apply_migration(pool: &PgPool, migration: &Migration) -> Result<()> {
    let mut transaction = pool
        .begin()
        .await
        .context("Failed to start migration transaction")?;

    // 🔧 Execute the migration SQL
    sqlx::query(&migration.up_sql)
        .execute(&mut *transaction)
        .await
        .context("Failed to execute migration SQL")?;

    // 📋 Record that this migration was applied
    let checksum = calculate_checksum(&migration.up_sql);
    sqlx::query("INSERT INTO migrations (id, description, checksum) VALUES ($1, $2, $3)")
        .bind(&migration.id)
        .bind(&migration.description)
        .bind(&checksum)
        .execute(&mut *transaction)
        .await
        .context("Failed to record migration in tracking table")?;

    // ✅ Commit the transaction
    transaction
        .commit()
        .await
        .context("Failed to commit migration transaction")?;

    info!("✅ Migration {} applied successfully!", migration.id);
    Ok(())
}

/// 🔍 Get list of already applied migrations
async fn get_applied_migrations(pool: &PgPool) -> Result<Vec<String>> {
    let rows = sqlx::query("SELECT id FROM migrations ORDER BY applied_at")
        .fetch_all(pool)
        .await
        .context("Failed to fetch applied migrations")?;

    let applied_migrations = rows
        .into_iter()
        .map(|row| row.get::<String, _>("id"))
        .collect();

    Ok(applied_migrations)
}

/// 🔢 Calculate checksum for migration integrity
fn calculate_checksum(sql: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(sql.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// 📚 Get all available migrations in order
/// This is where we define all our database schema changes!
pub fn get_all_migrations() -> Vec<Migration> {
    vec![
        // 🏗️ Migration 1: Create user-related tables
        Migration {
            id: "20240101000001_create_users_table".to_string(),
            description: "Create users and user_sessions tables".to_string(),
            up_sql: r#"
                -- 👤 Users table - Our amazing users!
                CREATE TABLE users (
                    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                    email VARCHAR(255) UNIQUE NOT NULL,
                    name VARCHAR(255) NOT NULL,
                    github_username VARCHAR(255) UNIQUE,
                    password_hash VARCHAR(255) NOT NULL,
                    email_verified BOOLEAN NOT NULL DEFAULT FALSE,
                    role user_role NOT NULL DEFAULT 'user',
                    is_active BOOLEAN NOT NULL DEFAULT TRUE,
                    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                    last_login_at TIMESTAMPTZ
                );

                -- 🎫 User sessions table - Secure session management
                CREATE TABLE user_sessions (
                    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                    token_hash VARCHAR(255) NOT NULL,
                    ip_address INET,
                    user_agent TEXT,
                    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                    expires_at TIMESTAMPTZ NOT NULL,
                    last_used_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
                );

                -- 🔍 Create indexes for better performance
                CREATE INDEX idx_users_email ON users(email);
                CREATE INDEX idx_users_github_username ON users(github_username);
                CREATE INDEX idx_user_sessions_user_id ON user_sessions(user_id);
                CREATE INDEX idx_user_sessions_expires_at ON user_sessions(expires_at);
            "#
            .to_string(),
            down_sql: Some(
                r#"
                DROP TABLE IF EXISTS user_sessions;
                DROP TABLE IF EXISTS users;
            "#
                .to_string(),
            ),
        },
        // 🏗️ Migration 2: Create enum types
        Migration {
            id: "20240101000002_create_enum_types".to_string(),
            description: "Create custom enum types for the application".to_string(),
            up_sql: r#"
                -- 📋 Create feedback status enum
                CREATE TYPE feedback_status AS ENUM (
                    'pending',
                    'processing',
                    'generating_changes',
                    'creating_pull_request',
                    'completed',
                    'failed',
                    'paused'
                );

                -- 👑 Create user role enum
                CREATE TYPE user_role AS ENUM (
                    'user',
                    'admin',
                    'service'
                );

                -- 🔔 Create notification type enum
                CREATE TYPE notification_type AS ENUM (
                    'feedback_completed',
                    'feedback_failed',
                    'pull_request_created',
                    'system_update',
                    'warning'
                );
            "#
            .to_string(),
            down_sql: Some(
                r#"
                DROP TYPE IF EXISTS notification_type;
                DROP TYPE IF EXISTS user_role;
                DROP TYPE IF EXISTS feedback_status;
            "#
                .to_string(),
            ),
        },
        // 🏗️ Migration 3: Create projects table
        Migration {
            id: "20240101000003_create_projects_table".to_string(),
            description: "Create projects table for repository management".to_string(),
            up_sql: r#"
                -- 🏠 Projects table - GitHub repositories we manage
                CREATE TABLE projects (
                    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                    owner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                    repository VARCHAR(255) NOT NULL,
                    description TEXT,
                    default_llm_provider VARCHAR(50),
                    system_message TEXT,
                    config JSONB,
                    is_active BOOLEAN NOT NULL DEFAULT TRUE,
                    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                    last_activity_at TIMESTAMPTZ,

                    -- 🎯 Ensure unique repository per owner
                    UNIQUE(owner_id, repository)
                );

                -- 🔍 Create indexes for better performance
                CREATE INDEX idx_projects_owner_id ON projects(owner_id);
                CREATE INDEX idx_projects_repository ON projects(repository);
                CREATE INDEX idx_projects_is_active ON projects(is_active);
                CREATE INDEX idx_projects_last_activity_at ON projects(last_activity_at);
            "#
            .to_string(),
            down_sql: Some(
                r#"
                DROP TABLE IF EXISTS projects;
            "#
                .to_string(),
            ),
        },
        // 🏗️ Migration 4: Create feedback table
        Migration {
            id: "20240101000004_create_feedback_table".to_string(),
            description: "Create feedback table for user submissions".to_string(),
            up_sql: r#"
                -- 📝 Feedback table - The heart of our system!
                CREATE TABLE feedback (
                    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
                    repository VARCHAR(255) NOT NULL,
                    content TEXT NOT NULL,
                    status feedback_status NOT NULL DEFAULT 'pending',
                    branch_name VARCHAR(255),
                    pull_request_url TEXT,
                    llm_provider VARCHAR(50),
                    metadata JSONB,
                    error_message TEXT,
                    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                    completed_at TIMESTAMPTZ
                );

                -- 🔍 Create indexes for better performance
                CREATE INDEX idx_feedback_user_id ON feedback(user_id);
                CREATE INDEX idx_feedback_repository ON feedback(repository);
                CREATE INDEX idx_feedback_status ON feedback(status);
                CREATE INDEX idx_feedback_created_at ON feedback(created_at);
                CREATE INDEX idx_feedback_completed_at ON feedback(completed_at);
            "#
            .to_string(),
            down_sql: Some(
                r#"
                DROP TABLE IF EXISTS feedback;
            "#
                .to_string(),
            ),
        },
        // 🏗️ Migration 5: Create rate limiting and notifications tables
        Migration {
            id: "20240101000005_create_utility_tables".to_string(),
            description: "Create rate limiting and notifications tables".to_string(),
            up_sql: r#"
                -- 🚦 Rate limiting table - Prevent abuse
                CREATE TABLE rate_limits (
                    id VARCHAR(255) PRIMARY KEY,
                    limit_type VARCHAR(50) NOT NULL,
                    request_count INTEGER NOT NULL DEFAULT 0,
                    window_start TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                    last_request TIMESTAMPTZ NOT NULL DEFAULT NOW()
                );

                -- 🔔 Notifications table - Keep users informed
                CREATE TABLE notifications (
                    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                    notification_type notification_type NOT NULL,
                    title VARCHAR(255) NOT NULL,
                    content TEXT NOT NULL,
                    related_id UUID,
                    is_read BOOLEAN NOT NULL DEFAULT FALSE,
                    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                    read_at TIMESTAMPTZ
                );

                -- 🔍 Create indexes for better performance
                CREATE INDEX idx_rate_limits_limit_type ON rate_limits(limit_type);
                CREATE INDEX idx_rate_limits_last_request ON rate_limits(last_request);
                CREATE INDEX idx_notifications_user_id ON notifications(user_id);
                CREATE INDEX idx_notifications_is_read ON notifications(is_read);
                CREATE INDEX idx_notifications_created_at ON notifications(created_at);
            "#
            .to_string(),
            down_sql: Some(
                r#"
                DROP TABLE IF EXISTS notifications;
                DROP TABLE IF EXISTS rate_limits;
            "#
                .to_string(),
            ),
        },
        // 🏗️ Migration 6: Create webhooks and job queues
        Migration {
            id: "20240101000006_create_webhooks_and_jobs".to_string(),
            description: "Create webhooks and background job processing tables".to_string(),
            up_sql: r#"
                -- 🪝 Webhooks table - GitHub integration
                CREATE TABLE webhooks (
                    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
                    event_type VARCHAR(100) NOT NULL,
                    payload JSONB NOT NULL,
                    processed BOOLEAN NOT NULL DEFAULT FALSE,
                    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                    processed_at TIMESTAMPTZ
                );

                -- 🔄 Background jobs table - Async processing
                CREATE TABLE background_jobs (
                    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                    job_type VARCHAR(100) NOT NULL,
                    payload JSONB NOT NULL,
                    status VARCHAR(50) NOT NULL DEFAULT 'pending',
                    retries INTEGER NOT NULL DEFAULT 0,
                    max_retries INTEGER NOT NULL DEFAULT 3,
                    error_message TEXT,
                    scheduled_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                    started_at TIMESTAMPTZ,
                    completed_at TIMESTAMPTZ,
                    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
                );

                -- 🔍 Create indexes for better performance
                CREATE INDEX idx_webhooks_project_id ON webhooks(project_id);
                CREATE INDEX idx_webhooks_event_type ON webhooks(event_type);
                CREATE INDEX idx_webhooks_processed ON webhooks(processed);
                CREATE INDEX idx_background_jobs_status ON background_jobs(status);
                CREATE INDEX idx_background_jobs_scheduled_at ON background_jobs(scheduled_at);
                CREATE INDEX idx_background_jobs_job_type ON background_jobs(job_type);
            "#
            .to_string(),
            down_sql: Some(
                r#"
                DROP TABLE IF EXISTS background_jobs;
                DROP TABLE IF EXISTS webhooks;
            "#
                .to_string(),
            ),
        },
        // 🏗️ Migration 7: Add triggers for updated_at columns
        Migration {
            id: "20240101000007_add_updated_at_triggers".to_string(),
            description: "Add triggers to automatically update updated_at columns".to_string(),
            up_sql: r#"
                -- 🔄 Function to update the updated_at column
                CREATE OR REPLACE FUNCTION update_updated_at_column()
                RETURNS TRIGGER AS $$
                BEGIN
                    NEW.updated_at = NOW();
                    RETURN NEW;
                END;
                $$ language 'plpgsql';

                -- 📝 Add triggers to all tables with updated_at columns
                CREATE TRIGGER update_users_updated_at
                    BEFORE UPDATE ON users
                    FOR EACH ROW
                    EXECUTE FUNCTION update_updated_at_column();

                CREATE TRIGGER update_projects_updated_at
                    BEFORE UPDATE ON projects
                    FOR EACH ROW
                    EXECUTE FUNCTION update_updated_at_column();

                CREATE TRIGGER update_feedback_updated_at
                    BEFORE UPDATE ON feedback
                    FOR EACH ROW
                    EXECUTE FUNCTION update_updated_at_column();
            "#
            .to_string(),
            down_sql: Some(
                r#"
                DROP TRIGGER IF EXISTS update_feedback_updated_at ON feedback;
                DROP TRIGGER IF EXISTS update_projects_updated_at ON projects;
                DROP TRIGGER IF EXISTS update_users_updated_at ON users;
                DROP FUNCTION IF EXISTS update_updated_at_column();
            "#
                .to_string(),
            ),
        },
    ]
}

/// 🔙 Rollback a specific migration (for development/testing)
pub async fn rollback_migration(pool: &PgPool, migration_id: &str) -> Result<()> {
    warn!("⚠️ Rolling back migration: {}", migration_id);

    let migrations = get_all_migrations();
    let migration = migrations
        .iter()
        .find(|m| m.id == migration_id)
        .context("Migration not found")?;

    if let Some(down_sql) = &migration.down_sql {
        let mut transaction = pool
            .begin()
            .await
            .context("Failed to start rollback transaction")?;

        // 🔄 Execute the rollback SQL
        sqlx::query(down_sql)
            .execute(&mut *transaction)
            .await
            .context("Failed to execute rollback SQL")?;

        // 🗑️ Remove the migration record
        sqlx::query("DELETE FROM migrations WHERE id = $1")
            .bind(migration_id)
            .execute(&mut *transaction)
            .await
            .context("Failed to remove migration record")?;

        // ✅ Commit the transaction
        transaction
            .commit()
            .await
            .context("Failed to commit rollback transaction")?;

        info!("✅ Migration {} rolled back successfully!", migration_id);
    } else {
        anyhow::bail!("Migration {} does not have rollback SQL", migration_id);
    }

    Ok(())
}

// 🧪 Tests - Because we test our migrations thoroughly!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum_calculation() {
        let sql = "CREATE TABLE test (id INT PRIMARY KEY);";
        let checksum1 = calculate_checksum(sql);
        let checksum2 = calculate_checksum(sql);

        // Should be consistent
        assert_eq!(checksum1, checksum2);

        // Should be different for different SQL
        let different_sql = "CREATE TABLE test2 (id INT PRIMARY KEY);";
        let checksum3 = calculate_checksum(different_sql);
        assert_ne!(checksum1, checksum3);

        println!("✅ Checksum calculation test passed!");
    }

    #[test]
    fn test_migration_ordering() {
        let migrations = get_all_migrations();

        // Ensure migrations are in order
        for i in 1..migrations.len() {
            assert!(migrations[i - 1].id < migrations[i].id);
        }

        println!("✅ Migration ordering test passed!");
    }

    #[test]
    fn test_all_migrations_have_descriptions() {
        let migrations = get_all_migrations();

        for migration in migrations {
            assert!(!migration.description.is_empty());
            assert!(!migration.up_sql.is_empty());
        }

        println!("✅ Migration completeness test passed!");
    }
}
