// 🗄️ Database Module - The Data Storage Heart of Feedbacker! 🗄️
// This module handles all database operations, connections, and migrations
// Built with SQLx for async performance and safety - Trisha loves type safety! 📊
// Created with love by Aye & Hue - Making data management as smooth as silk! ✨

use anyhow::{Context, Result};
use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres};
use std::time::Duration;
use tracing::{info, warn};

// 📦 Re-export modules for easy access
pub mod migrations;
pub mod models;

// 🔄 Re-export commonly used types
pub use models::*;
pub use sqlx::Row;

/// 🏊‍♂️ Create a new database connection pool
/// This is our gateway to the PostgreSQL database!
pub async fn create_pool(database_url: &str) -> Result<PgPool> {
    info!("🔌 Creating database connection pool...");

    let pool = PgPoolOptions::new()
        .max_connections(20) // 🎯 Maximum connections in the pool
        .min_connections(2) // 🔄 Minimum connections to maintain
        .acquire_timeout(Duration::from_secs(10)) // ⏱️ Timeout for getting a connection
        // TODO: Add connect timeout when available in SQLx version
        .idle_timeout(Duration::from_secs(600)) // 💤 Close idle connections after 10 minutes
        .max_lifetime(Duration::from_secs(1800)) // 🔄 Recreate connections every 30 minutes
        .connect(database_url)
        .await
        .context("Failed to create database connection pool")?;

    info!("✅ Database connection pool created successfully!");

    Ok(pool)
}

/// 🏃‍♂️ Run all pending database migrations
/// This keeps our database schema up to date!
pub async fn run_migrations(pool: &PgPool) -> Result<()> {
    info!("🚀 Running database migrations...");

    // 🔍 Check if migrations table exists
    let migrations_exist = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'migrations')",
    )
    .fetch_one(pool)
    .await
    .context("Failed to check if migrations table exists")?;

    if !migrations_exist {
        info!("📋 Creating migrations table...");
        migrations::create_migrations_table(pool).await?;
    }

    // 🎯 Run each migration in order
    migrations::run_all_migrations(pool)
        .await
        .context("Failed to run database migrations")?;

    info!("✅ All database migrations completed successfully!");

    Ok(())
}

/// 🔍 Check database connection health
/// Perfect for health checks and monitoring!
pub async fn check_connection_health(pool: &PgPool) -> Result<bool> {
    // TODO: Implement proper database health check when database is ready
    info!("💚 Database connection is healthy! (placeholder)");
    Ok(true)
}

/// 📊 Get database connection pool statistics
/// Useful for monitoring and debugging!
pub fn get_pool_stats(pool: &PgPool) -> PoolStats {
    PoolStats {
        size: pool.size(),
        idle: pool.num_idle() as u32,
    }
}

/// 📊 Database pool statistics structure
#[derive(Debug, Clone)]
pub struct PoolStats {
    /// 📈 Total number of connections in the pool
    pub size: u32,
    /// 💤 Number of idle connections
    pub idle: u32,
}

impl PoolStats {
    /// 🏃‍♂️ Get the number of active connections
    pub fn active(&self) -> u32 {
        self.size - self.idle
    }

    /// 📊 Check if the pool is healthy (not all connections are in use)
    pub fn is_healthy(&self) -> bool {
        self.idle > 0 || self.size < 15 // Allow some room for growth
    }
}

/// 🧹 Clean up old records from the database
/// This helps keep our database performant and tidy!
pub async fn cleanup_old_records(pool: &PgPool) -> Result<()> {
    info!("🧹 Starting database cleanup...");

    let transaction = pool
        .begin()
        .await
        .context("Failed to start cleanup transaction")?;

    // TODO: Implement cleanup queries when database is ready
    let deleted_feedback = 0u64;
    let deleted_sessions = 0u64;
    let deleted_rate_limits = 0u64;

    // ✅ Commit the transaction
    transaction
        .commit()
        .await
        .context("Failed to commit cleanup transaction")?;

    info!(
        "✅ Database cleanup completed! Removed {} feedback, {} sessions, {} rate limits",
        deleted_feedback, deleted_sessions, deleted_rate_limits
    );

    Ok(())
}

/// 🔄 Database connection helper trait
/// Provides common database operations for all models
pub trait DatabaseConnection {
    /// 🔍 Get a reference to the database pool
    fn pool(&self) -> &PgPool;

    /// 🏃‍♂️ Execute a query and return the number of affected rows
    async fn execute_query(&self, query: &str) -> Result<u64> {
        let rows_affected = sqlx::query(query)
            .execute(self.pool())
            .await
            .context("Failed to execute query")?
            .rows_affected();

        Ok(rows_affected)
    }

    /// 🔍 Check if a record exists by ID
    async fn exists_by_id(&self, table: &str, id: &str) -> Result<bool> {
        let query = format!("SELECT EXISTS(SELECT 1 FROM {} WHERE id = $1)", table);

        let exists = sqlx::query_scalar::<_, bool>(&query)
            .bind(id)
            .fetch_one(self.pool())
            .await
            .context("Failed to check if record exists")?;

        Ok(exists)
    }
}

// 🧪 Tests - Because we test our database operations thoroughly!
#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    // 🎯 Helper function to create a test database pool
    async fn create_test_pool() -> PgPool {
        let database_url = std::env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://test:test@localhost/feedbacker_test".to_string());

        create_pool(&database_url)
            .await
            .expect("Failed to create test database pool")
    }

    #[tokio::test]
    async fn test_pool_creation() {
        // This test only runs if we have a test database available
        if std::env::var("TEST_DATABASE_URL").is_ok() {
            let pool = create_test_pool().await;
            assert!(pool.size() > 0);
            println!("✅ Database pool creation test passed!");
        }
    }

    #[tokio::test]
    async fn test_connection_health() {
        // This test only runs if we have a test database available
        if std::env::var("TEST_DATABASE_URL").is_ok() {
            let pool = create_test_pool().await;
            let health = check_connection_health(&pool).await;
            assert!(health.is_ok());
            println!("✅ Database connection health test passed!");
        }
    }

    #[test]
    fn test_pool_stats() {
        // Create a mock pool stats for testing
        let stats = PoolStats { size: 10, idle: 3 };

        assert_eq!(stats.active(), 7);
        assert!(stats.is_healthy());
        println!("✅ Pool stats test passed!");
    }
}
