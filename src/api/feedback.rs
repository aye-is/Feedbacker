// 📝 Feedback API - The Heart of Feedbacker! 📝
// This module handles feedback submission and management endpoints
// Where user feedback becomes amazing GitHub pull requests! 🚀
// Created with love by Aye & Hue - Making feedback processing magical! ✨
// Trisha from Accounting says this is the most organized feedback system ever! 📊

use anyhow::{Context, Result};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde::{Deserialize, Serialize};
use sqlx::Row;  // 🔧 Added Row trait import for database row access
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::{
    api::{
        utils::{handle_error, not_found_error, validation_error},
        ApiResponse, AppState, PaginatedResponse, PaginationParams, ValidateRequest,
    },
    database::models::{Feedback, FeedbackStats, FeedbackStatus},
};

/// 📝 Feedback submission request structure
/// This is what users send us when they want to improve a repository!
#[derive(Debug, Deserialize)]
pub struct SubmitFeedbackRequest {
    /// 🎯 Target repository in "owner/repo" format
    pub repository: String,
    /// 📝 The actual feedback content - what the user wants to improve
    pub content: String,
    /// 🤖 Preferred LLM provider (optional - will use project default)
    pub llm_provider: Option<String>,
    /// 🔧 Additional metadata for processing (optional)
    pub metadata: Option<serde_json::Value>,
    /// 👤 User information (for anonymous submissions)
    pub user_info: Option<AnonymousUserInfo>,
}

/// 👤 Anonymous user information for feedback without accounts
#[derive(Debug, Deserialize)]
pub struct AnonymousUserInfo {
    /// 📧 Email for notifications (optional)
    pub email: Option<String>,
    /// 👤 Display name (optional)
    pub name: Option<String>,
}

/// 📊 Feedback submission response
#[derive(Debug, Serialize)]
pub struct SubmitFeedbackResponse {
    /// 🆔 Unique feedback ID for tracking
    pub feedback_id: Uuid,
    /// 📋 Current status of the feedback
    pub status: FeedbackStatus,
    /// 🔗 URL to track the feedback progress
    pub tracking_url: String,
    /// ⏰ Estimated processing time in minutes
    pub estimated_processing_time: u32,
}

/// 📊 Detailed feedback information for responses
#[derive(Debug, Serialize)]
pub struct FeedbackDetails {
    /// 🆔 Feedback ID
    pub id: Uuid,
    /// 🎯 Target repository
    pub repository: String,
    /// 📝 Feedback content (truncated for privacy)
    pub content_preview: String,
    /// 📋 Current status
    pub status: FeedbackStatus,
    /// 🌿 GitHub branch name (if created)
    pub branch_name: Option<String>,
    /// 🔗 Pull request URL (if created)
    pub pull_request_url: Option<String>,
    /// 🤖 LLM provider used
    pub llm_provider: Option<String>,
    /// ❌ Error message (if failed)
    pub error_message: Option<String>,
    /// ⏰ When submitted
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// 🔄 Last updated
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// ✅ When completed (if applicable)
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// 🔍 Feedback query parameters for listing
#[derive(Debug, Deserialize)]
pub struct FeedbackQuery {
    /// 📋 Filter by status
    pub status: Option<FeedbackStatus>,
    /// 🎯 Filter by repository
    pub repository: Option<String>,
    /// 👤 Filter by user (admin only)
    pub user_id: Option<Uuid>,
    /// 🤖 Filter by LLM provider
    pub llm_provider: Option<String>,
    /// ⏰ Filter by date range (from)
    pub from_date: Option<chrono::DateTime<chrono::Utc>>,
    /// ⏰ Filter by date range (to)
    pub to_date: Option<chrono::DateTime<chrono::Utc>>,
}

impl ValidateRequest for SubmitFeedbackRequest {
    /// ✅ Validate feedback submission request
    fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // 🎯 Validate repository format
        if self.repository.is_empty() {
            errors.push("Repository cannot be empty".to_string());
        } else if !self.repository.contains('/') || self.repository.split('/').count() != 2 {
            errors.push("Repository must be in 'owner/repo' format".to_string());
        }

        // 📝 Validate content
        if self.content.trim().is_empty() {
            errors.push("Feedback content cannot be empty".to_string());
        } else if self.content.len() > 10000 {
            errors.push("Feedback content cannot exceed 10,000 characters".to_string());
        } else if self.content.len() < 10 {
            errors.push("Feedback content must be at least 10 characters".to_string());
        }

        // 🤖 Validate LLM provider if specified
        if let Some(provider) = &self.llm_provider {
            if !["openai", "anthropic"].contains(&provider.as_str()) {
                errors.push("Invalid LLM provider. Supported: openai, anthropic".to_string());
            }
        }

        // 📧 Validate anonymous user info if provided
        if let Some(user_info) = &self.user_info {
            if let Some(email) = &user_info.email {
                if !email.contains('@') || email.len() > 255 {
                    errors.push("Invalid email address".to_string());
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

/// 📝 Submit new feedback for processing
/// This is the main endpoint where users submit their improvement ideas!
pub async fn submit_feedback(
    State(app_state): State<AppState>,
    Json(request): Json<SubmitFeedbackRequest>,
) -> Response {
    info!(
        "📝 Received feedback submission for repository: {}",
        request.repository
    );

    // ✅ Validate the request
    if let Err(errors) = request.validate() {
        warn!("❌ Validation failed for feedback submission: {:?}", errors);
        let api_response = ApiResponse::<()>::error(
            "validation_error".to_string(),
            "Request validation failed".to_string(),
            Some(serde_json::json!({ "errors": errors })),
        );
        return (StatusCode::BAD_REQUEST, Json(api_response)).into_response();
    }

    // 🔍 Check if the repository is accessible and aye-is is a collaborator
    // TODO: Add repository validation when GitHub module is ready
    // if !github_client.is_collaborator(&request.repository, "aye-is").await? {
    //     return forbidden_error();
    // }

    match create_feedback_record(&app_state, request).await {
        Ok(response) => {
            info!(
                "✅ Feedback submitted successfully: {}",
                response.feedback_id
            );

            // 🚀 Queue the feedback for processing
            // TODO: Add job queuing when background jobs module is ready
            // app_state.job_queue.queue_feedback_processing(response.feedback_id).await?;

            (
                StatusCode::CREATED,
                Json(ApiResponse::<SubmitFeedbackResponse>::success(
                    "Feedback submitted successfully! Processing will begin shortly.".to_string(),
                    response,
                )),
            ).into_response()
        }
        Err(e) => {
            error!("❌ Failed to submit feedback: {:#}", e);
            let error_msg = format!("{:#}", e);
            let api_response = ApiResponse::<()>::error(
                "internal_error".to_string(),
                "An internal error occurred".to_string(),
                Some(serde_json::json!({ "details": error_msg })),
            );
            (StatusCode::INTERNAL_SERVER_ERROR, Json(api_response)).into_response()
        }
    }
}

/// 🔍 Get feedback by ID
/// Allows users to check the status of their submitted feedback
pub async fn get_feedback(
    State(app_state): State<AppState>,
    Path(feedback_id): Path<Uuid>,
) -> Response {
    info!("🔍 Fetching feedback details for ID: {}", feedback_id);

    match fetch_feedback_details(&app_state, feedback_id).await {
        Ok(Some(feedback)) => {
            info!("✅ Found feedback: {}", feedback_id);
            (
                StatusCode::OK,
                Json(ApiResponse::success("Feedback found".to_string(), feedback)),
            ).into_response()
        }
        Ok(None) => {
            warn!("🔍 Feedback not found: {}", feedback_id);
            let api_response = ApiResponse::<()>::error(
                "not_found".to_string(),
                "Feedback not found".to_string(),
                None,
            );
            (StatusCode::NOT_FOUND, Json(api_response)).into_response()
        }
        Err(e) => {
            error!("❌ Failed to fetch feedback {}: {:#}", feedback_id, e);
            let error_msg = format!("{:#}", e);
            let api_response = ApiResponse::<()>::error(
                "internal_error".to_string(),
                "An internal error occurred".to_string(),
                Some(serde_json::json!({ "details": error_msg })),
            );
            (StatusCode::INTERNAL_SERVER_ERROR, Json(api_response)).into_response()
        }
    }
}

/// 📋 List feedback with filtering and pagination
/// Allows users to see all their submitted feedback
pub async fn list_feedback(
    State(app_state): State<AppState>,
    Query(pagination): Query<PaginationParams>,
    Query(query): Query<FeedbackQuery>,
) -> Response {
    info!("📋 Listing feedback with filters: {:?}", query);

    let pagination = pagination.validate();

    match fetch_feedback_list(&app_state, &pagination, &query).await {
        Ok(response) => {
            info!("✅ Retrieved {} feedback items", response.items.len());
            (
                StatusCode::OK,
                Json(ApiResponse::success(
                    "Feedback list retrieved successfully".to_string(),
                    response,
                )),
            ).into_response()
        }
        Err(e) => {
            error!("❌ Failed to list feedback: {:#}", e);
            let error_msg = format!("{:#}", e);
            let api_response = ApiResponse::<()>::error(
                "internal_error".to_string(),
                "An internal error occurred".to_string(),
                Some(serde_json::json!({ "details": error_msg })),
            );
            (StatusCode::INTERNAL_SERVER_ERROR, Json(api_response)).into_response()
        }
    }
}

/// 📊 Get feedback statistics for a user
/// Provides insights into feedback processing success rates
pub async fn get_feedback_stats(
    State(app_state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Response {
    info!("📊 Fetching feedback statistics for user: {}", user_id);

    match Feedback::get_user_stats(&app_state.db_pool, user_id).await {
        Ok(stats) => {
            info!("✅ Retrieved feedback statistics for user: {}", user_id);
            (
                StatusCode::OK,
                Json(ApiResponse::success(
                    "Statistics retrieved successfully".to_string(),
                    stats,
                )),
            ).into_response()
        }
        Err(e) => {
            error!("❌ Failed to get feedback statistics: {:#}", e);
            let error_msg = format!("{:#}", e);
            let api_response = ApiResponse::<()>::error(
                "internal_error".to_string(),
                "An internal error occurred".to_string(),
                Some(serde_json::json!({ "details": error_msg })),
            );
            (StatusCode::INTERNAL_SERVER_ERROR, Json(api_response)).into_response()
        }
    }
}

/// 🔄 Retry failed feedback processing
/// Allows users to retry feedback that failed to process
pub async fn retry_feedback(
    State(app_state): State<AppState>,
    Path(feedback_id): Path<Uuid>,
) -> Response {
    info!("🔄 Retrying feedback processing for ID: {}", feedback_id);

    match retry_feedback_processing(&app_state, feedback_id).await {
        Ok(()) => {
            info!("✅ Feedback retry queued successfully: {}", feedback_id);
            (
                StatusCode::OK,
                Json(ApiResponse::<()>::success_no_data(
                    "Feedback processing retry queued successfully".to_string(),
                )),
            ).into_response()
        }
        Err(e) => {
            error!("❌ Failed to retry feedback {}: {:#}", feedback_id, e);
            let error_msg = format!("{:#}", e);
            let api_response = ApiResponse::<()>::error(
                "internal_error".to_string(),
                "An internal error occurred".to_string(),
                Some(serde_json::json!({ "details": error_msg })),
            );
            (StatusCode::INTERNAL_SERVER_ERROR, Json(api_response)).into_response()
        }
    }
}

// 🔧 Helper functions for the API endpoints

/// ➕ Create a new feedback record in the database
async fn create_feedback_record(
    app_state: &AppState,
    request: SubmitFeedbackRequest,
) -> Result<SubmitFeedbackResponse> {
    // TODO: Get user_id from authentication when auth module is ready
    let user_id = None; // For now, support anonymous feedback

    let feedback = Feedback::create(
        &app_state.db_pool,
        user_id,
        request.repository.clone(),
        request.content,
    )
    .await
    .context("Failed to create feedback record")?;

    let response = SubmitFeedbackResponse {
        feedback_id: feedback.id,
        status: feedback.status,
        tracking_url: format!("/api/feedback/{}", feedback.id),
        estimated_processing_time: 5, // 5 minutes estimate
    };

    Ok(response)
}

/// 🔍 Fetch detailed feedback information
async fn fetch_feedback_details(
    app_state: &AppState,
    feedback_id: Uuid,
) -> Result<Option<FeedbackDetails>> {
    let feedback = Feedback::find_by_id(&app_state.db_pool, feedback_id)
        .await
        .context("Failed to fetch feedback from database")?;

    Ok(feedback.map(|f| FeedbackDetails {
        id: f.id,
        repository: f.repository,
        content_preview: truncate_content(&f.content, 200),
        status: f.status,
        branch_name: f.branch_name,
        pull_request_url: f.pull_request_url,
        llm_provider: f.llm_provider,
        error_message: f.error_message,
        created_at: f.created_at,
        updated_at: f.updated_at,
        completed_at: f.completed_at,
    }))
}

/// 📋 Fetch a paginated list of feedback
async fn fetch_feedback_list(
    app_state: &AppState,
    pagination: &PaginationParams,
    query: &FeedbackQuery,
) -> Result<PaginatedResponse<FeedbackDetails>> {
    // 🏗️ Build the SQL query with filters
    let mut sql_where = Vec::new();
    let mut params = Vec::new();
    let mut param_index = 1;

    if let Some(status) = &query.status {
        sql_where.push(format!("status = ${}", param_index));
        params.push(serde_json::to_value(status)?);
        param_index += 1;
    }

    if let Some(repository) = &query.repository {
        sql_where.push(format!("repository = ${}", param_index));
        params.push(serde_json::to_value(repository)?);
        param_index += 1;
    }

    if let Some(user_id) = &query.user_id {
        sql_where.push(format!("user_id = ${}", param_index));
        params.push(serde_json::to_value(user_id)?);
        param_index += 1;
    }

    let where_clause = if sql_where.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", sql_where.join(" AND "))
    };

    // 📊 Get total count
    let count_query = format!("SELECT COUNT(*) FROM feedback {}", where_clause);
    let total = sqlx::query_scalar::<_, i64>(&count_query)
        .fetch_one(&app_state.db_pool)
        .await
        .context("Failed to get feedback count")?;

    // 📋 Get the actual feedback records
    let order_clause = format!(
        "ORDER BY created_at {} LIMIT {} OFFSET {}",
        match pagination.sort_order {
            crate::api::SortOrder::Asc => "ASC",
            crate::api::SortOrder::Desc => "DESC",
        },
        pagination.limit,
        pagination.offset()
    );

    let query_sql = format!(
        r#"
        SELECT id, repository, content, status, branch_name, pull_request_url,
               llm_provider, error_message, created_at, updated_at, completed_at
        FROM feedback
        {}
        {}
        "#,
        where_clause, order_clause
    );

    let feedback_records = sqlx::query(&query_sql)
        .fetch_all(&app_state.db_pool)
        .await
        .context("Failed to fetch feedback list")?;

    let feedback_details: Vec<FeedbackDetails> = feedback_records
        .into_iter()
        .map(|row| FeedbackDetails {
            id: row.get("id"),
            repository: row.get("repository"),
            content_preview: truncate_content(&row.get::<String, _>("content"), 200),
            status: serde_json::from_value(row.get("status")).unwrap_or(FeedbackStatus::Pending),
            branch_name: row.get("branch_name"),
            pull_request_url: row.get("pull_request_url"),
            llm_provider: row.get("llm_provider"),
            error_message: row.get("error_message"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            completed_at: row.get("completed_at"),
        })
        .collect();

    Ok(PaginatedResponse::new(
        feedback_details,
        pagination.page,
        pagination.limit,
        total as u64,
    ))
}

/// 🔄 Retry failed feedback processing
async fn retry_feedback_processing(app_state: &AppState, feedback_id: Uuid) -> Result<()> {
    // 🔍 First, verify the feedback exists and can be retried
    let feedback = Feedback::find_by_id(&app_state.db_pool, feedback_id)
        .await
        .context("Failed to fetch feedback for retry")?;

    let mut feedback = feedback.ok_or_else(|| anyhow::anyhow!("Feedback not found"))?;

    // 📋 Check if feedback is in a retryable state
    if !matches!(
        feedback.status,
        FeedbackStatus::Failed | FeedbackStatus::Paused
    ) {
        anyhow::bail!(
            "Feedback is not in a retryable state (current status: {:?})",
            feedback.status
        );
    }

    // 🔄 Reset the feedback status to pending
    feedback
        .update_status(&app_state.db_pool, FeedbackStatus::Pending, None)
        .await
        .context("Failed to reset feedback status")?;

    // 🚀 Queue the feedback for processing again
    // TODO: Add job queuing when background jobs module is ready
    // app_state.job_queue.queue_feedback_processing(feedback_id).await?;

    info!("🔄 Feedback {} queued for retry processing", feedback_id);

    Ok(())
}

/// ✂️ Truncate content for preview (privacy-friendly)
fn truncate_content(content: &str, max_length: usize) -> String {
    if content.len() <= max_length {
        content.to_string()
    } else {
        format!("{}...", &content[..max_length])
    }
}

// 🧪 Tests - Because we thoroughly test our feedback API!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_submit_feedback_request_validation() {
        let valid_request = SubmitFeedbackRequest {
            repository: "owner/repo".to_string(),
            content: "This is a valid feedback content that is long enough".to_string(),
            llm_provider: Some("openai".to_string()),
            metadata: None,
            user_info: None,
        };

        assert!(valid_request.validate().is_ok());
        println!("✅ Valid feedback request validation test passed!");

        let invalid_request = SubmitFeedbackRequest {
            repository: "invalid".to_string(),
            content: "short".to_string(),
            llm_provider: Some("invalid_provider".to_string()),
            metadata: None,
            user_info: None,
        };

        let errors = invalid_request.validate().unwrap_err();
        assert!(errors.len() > 0);
        println!("✅ Invalid feedback request validation test passed!");
    }

    #[test]
    fn test_content_truncation() {
        let short_content = "Short content";
        let truncated = truncate_content(short_content, 100);
        assert_eq!(truncated, short_content);

        let long_content = "This is a very long content that should be truncated when it exceeds the maximum length limit";
        let truncated = truncate_content(long_content, 20);
        assert_eq!(truncated, "This is a very long ...".to_string());
        println!("✅ Content truncation test passed!");
    }

    #[test]
    fn test_feedback_response_serialization() {
        let response = SubmitFeedbackResponse {
            feedback_id: Uuid::new_v4(),
            status: FeedbackStatus::Pending,
            tracking_url: "/api/feedback/123".to_string(),
            estimated_processing_time: 5,
        };

        let serialized = serde_json::to_string(&response);
        assert!(serialized.is_ok());
        println!("✅ Feedback response serialization test passed!");
    }
}
