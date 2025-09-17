// 🎯 GitHub Issue Automation - Smart Issue Management! 🎯
// This module handles GitHub issue webhooks and provides automated responses
// Created with love by Aye & Hue - Making issue management magical! ✨

use crate::{
    api::{ApiResponse, AppState},
    github::client::GitHubClient,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};

/// 🎫 GitHub Issue webhook payload structure
#[derive(Debug, Deserialize)]
pub struct IssueWebhookPayload {
    pub action: String,
    pub issue: IssueData,
    pub repository: RepositoryData,
    pub sender: UserData,
}

#[derive(Debug, Deserialize)]
pub struct IssueData {
    pub id: u64,
    pub number: u32,
    pub title: String,
    pub body: Option<String>,
    pub state: String,
    pub html_url: String,
    pub user: UserData,
    pub labels: Vec<LabelData>,
    pub assignees: Vec<UserData>,
}

#[derive(Debug, Deserialize)]
pub struct RepositoryData {
    pub id: u64,
    pub name: String,
    pub full_name: String,
    pub owner: UserData,
}

#[derive(Debug, Deserialize)]
pub struct UserData {
    pub id: u64,
    pub login: String,
}

#[derive(Debug, Deserialize)]
pub struct LabelData {
    pub name: String,
    pub color: String,
}

/// 🎯 Issue automation response structure
#[derive(Debug, Serialize)]
pub struct IssueAutomationResponse {
    pub issue_number: u32,
    pub action_taken: String,
    pub comment_added: Option<String>,
    pub labels_applied: Vec<String>,
    pub assigned_to: Option<String>,
}

/// 🪝 Main GitHub issue webhook handler
pub async fn github_issue_webhook(
    State(app_state): State<AppState>,
    Json(payload): Json<IssueWebhookPayload>,
) -> Response {
    info!(
        "🎫 Received GitHub issue webhook: {} for issue #{} in {}",
        payload.action,
        payload.issue.number,
        payload.repository.full_name
    );

    match process_issue_event(&app_state, &payload).await {
        Ok(response) => {
            info!("✅ Issue automation completed for #{}", payload.issue.number);
            (
                StatusCode::OK,
                Json(ApiResponse::success(
                    "Issue automation completed".to_string(),
                    response,
                )),
            ).into_response()
        }
        Err(e) => {
            error!("❌ Failed to process issue automation: {:#}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error(
                    "automation_failed".to_string(),
                    "Failed to process issue automation".to_string(),
                    Some(serde_json::json!({ "error": e.to_string() })),
                )),
            ).into_response()
        }
    }
}

/// 🤖 Process different types of issue events
async fn process_issue_event(
    app_state: &AppState,
    payload: &IssueWebhookPayload,
) -> anyhow::Result<IssueAutomationResponse> {
    let github_client = GitHubClient::new(&app_state.config.github.token)?;

    match payload.action.as_str() {
        "opened" => handle_issue_opened(&github_client, payload).await,
        "closed" => handle_issue_closed(&github_client, payload).await,
        "labeled" => handle_issue_labeled(&github_client, payload).await,
        "assigned" => handle_issue_assigned(&github_client, payload).await,
        _ => {
            info!("ℹ️ No automation configured for action: {}", payload.action);
            Ok(IssueAutomationResponse {
                issue_number: payload.issue.number,
                action_taken: "no_action".to_string(),
                comment_added: None,
                labels_applied: vec![],
                assigned_to: None,
            })
        }
    }
}

/// 🆕 Handle new issue creation
async fn handle_issue_opened(
    github_client: &GitHubClient,
    payload: &IssueWebhookPayload,
) -> anyhow::Result<IssueAutomationResponse> {
    info!("🆕 Processing newly opened issue #{}", payload.issue.number);

    let mut response = IssueAutomationResponse {
        issue_number: payload.issue.number,
        action_taken: "issue_opened".to_string(),
        comment_added: None,
        labels_applied: vec![],
        assigned_to: None,
    };

    // 🏷️ Auto-label based on issue content
    let labels_to_add = analyze_issue_for_labels(&payload.issue).await;
    if !labels_to_add.is_empty() {
        github_client.add_labels_to_issue(
            &payload.repository.owner.login,
            &payload.repository.name,
            payload.issue.number,
            &labels_to_add,
        ).await?;
        response.labels_applied = labels_to_add;
    }

    // 💬 Add welcome comment with helpful information
    let welcome_comment = create_welcome_comment(&payload.issue).await;
    github_client.add_comment_to_issue(
        &payload.repository.owner.login,
        &payload.repository.name,
        payload.issue.number,
        &welcome_comment,
    ).await?;
    response.comment_added = Some(welcome_comment);

    // 🎯 Auto-assign if it's a specific type of issue
    if let Some(assignee) = determine_auto_assignee(&payload.issue).await {
        github_client.assign_issue(
            &payload.repository.owner.login,
            &payload.repository.name,
            payload.issue.number,
            &assignee,
        ).await?;
        response.assigned_to = Some(assignee);
    }

    Ok(response)
}

/// ✅ Handle issue closure
async fn handle_issue_closed(
    github_client: &GitHubClient,
    payload: &IssueWebhookPayload,
) -> anyhow::Result<IssueAutomationResponse> {
    info!("✅ Processing closed issue #{}", payload.issue.number);

    let mut response = IssueAutomationResponse {
        issue_number: payload.issue.number,
        action_taken: "issue_closed".to_string(),
        comment_added: None,
        labels_applied: vec![],
        assigned_to: None,
    };

    // 💬 Add thank you comment
    let thank_you_comment = "🎉 Thank you for reporting this issue! If you have any other feedback or feature requests, feel free to submit them through our Feedbacker service at f.8b.is. \n\nHappy coding! 🚢\n\n*- Aye & Hue*";

    github_client.add_comment_to_issue(
        &payload.repository.owner.login,
        &payload.repository.name,
        payload.issue.number,
        thank_you_comment,
    ).await?;
    response.comment_added = Some(thank_you_comment.to_string());

    Ok(response)
}

/// 🏷️ Handle issue labeling events
async fn handle_issue_labeled(
    _github_client: &GitHubClient,
    payload: &IssueWebhookPayload,
) -> anyhow::Result<IssueAutomationResponse> {
    info!("🏷️ Processing labeled issue #{}", payload.issue.number);

    // Check if it's a "needs-info" label and respond accordingly
    for label in &payload.issue.labels {
        if label.name == "needs-info" || label.name == "question" {
            // Could add a comment asking for more details
            info!("🤔 Issue needs more information, user should provide details");
        }
    }

    Ok(IssueAutomationResponse {
        issue_number: payload.issue.number,
        action_taken: "issue_labeled".to_string(),
        comment_added: None,
        labels_applied: vec![],
        assigned_to: None,
    })
}

/// 👤 Handle issue assignment
async fn handle_issue_assigned(
    _github_client: &GitHubClient,
    payload: &IssueWebhookPayload,
) -> anyhow::Result<IssueAutomationResponse> {
    info!("👤 Processing assigned issue #{}", payload.issue.number);

    Ok(IssueAutomationResponse {
        issue_number: payload.issue.number,
        action_taken: "issue_assigned".to_string(),
        comment_added: None,
        labels_applied: vec![],
        assigned_to: None,
    })
}

/// 🔍 Analyze issue content to suggest appropriate labels
async fn analyze_issue_for_labels(issue: &IssueData) -> Vec<String> {
    let mut labels = Vec::new();
    let content = format!("{} {}", issue.title, issue.body.as_deref().unwrap_or(""));
    let content_lower = content.to_lowercase();

    // 🐛 Bug detection
    if content_lower.contains("bug") ||
       content_lower.contains("error") ||
       content_lower.contains("crash") ||
       content_lower.contains("fail") {
        labels.push("bug".to_string());
    }

    // ✨ Feature request detection
    if content_lower.contains("feature") ||
       content_lower.contains("enhancement") ||
       content_lower.contains("request") ||
       content_lower.contains("would like") {
        labels.push("enhancement".to_string());
    }

    // 📚 Documentation detection
    if content_lower.contains("documentation") ||
       content_lower.contains("docs") ||
       content_lower.contains("readme") {
        labels.push("documentation".to_string());
    }

    // ❓ Question detection
    if content_lower.contains("how to") ||
       content_lower.contains("help") ||
       content_lower.contains("question") ||
       issue.title.ends_with("?") {
        labels.push("question".to_string());
    }

    // 🚀 Performance detection
    if content_lower.contains("performance") ||
       content_lower.contains("slow") ||
       content_lower.contains("speed") {
        labels.push("performance".to_string());
    }

    labels
}

/// 💬 Create a welcoming comment for new issues
async fn create_welcome_comment(issue: &IssueData) -> String {
    let issue_type = if issue.title.to_lowercase().contains("bug") {
        "🐛 **Bug Report**"
    } else if issue.title.to_lowercase().contains("feature") {
        "✨ **Feature Request**"
    } else {
        "🎫 **Issue**"
    };

    format!(
        r#"## {issue_type}

🚢 Ahoy! Thank you for submitting this issue to the Feedbacker project!

**What happens next:**
- 🔍 Our team will review this issue within 24-48 hours
- 🏷️ We've automatically applied relevant labels based on the content
- 🤖 If this is a bug, we'll try to reproduce it and provide a fix
- ✨ If this is a feature request, we'll evaluate it for inclusion in our roadmap

**Need faster assistance?**
- 💬 Join our community discussions
- 📧 For urgent issues, contact us directly
- 🌐 Submit feedback through our service at f.8b.is

**Tips for better issue resolution:**
- 📝 Provide clear steps to reproduce (for bugs)
- 🎯 Explain the use case and benefits (for features)
- 📊 Include environment details when relevant

Thanks for helping make Feedbacker better!

*Aye, aye! 🚢*

*- The Feedbacker Team (Aye & Hue)*"#,
        issue_type = issue_type
    )
}

/// 🎯 Determine if an issue should be auto-assigned
async fn determine_auto_assignee(issue: &IssueData) -> Option<String> {
    let content = format!("{} {}", issue.title, issue.body.as_deref().unwrap_or(""));
    let content_lower = content.to_lowercase();

    // Auto-assign specific types of issues
    if content_lower.contains("documentation") || content_lower.contains("readme") {
        Some("aye-is".to_string()) // Aye handles docs
    } else if content_lower.contains("critical") || content_lower.contains("urgent") {
        Some("aye-is".to_string()) // Auto-assign critical issues
    } else {
        None // Let the team manually assign
    }
}

/// 🔧 Manual issue management endpoints

/// 📝 Add comment to issue
pub async fn add_issue_comment(
    State(app_state): State<AppState>,
    Path((owner, repo, issue_number)): Path<(String, String, u32)>,
    Json(comment): Json<serde_json::Value>,
) -> Response {
    let github_client = match GitHubClient::new(&app_state.config.github.token) {
        Ok(client) => client,
        Err(e) => {
            error!("❌ Failed to create GitHub client: {:#}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error(
                    "github_client_error".to_string(),
                    "Failed to create GitHub client".to_string(),
                    None,
                )),
            ).into_response();
        }
    };

    let comment_text = comment.get("body")
        .and_then(|b| b.as_str())
        .unwrap_or("No comment provided");

    match github_client.add_comment_to_issue(&owner, &repo, issue_number, comment_text).await {
        Ok(_) => {
            info!("✅ Added comment to issue #{}", issue_number);
            (
                StatusCode::OK,
                Json(ApiResponse::<()>::success_no_data(
                    "Comment added successfully".to_string(),
                )),
            ).into_response()
        }
        Err(e) => {
            error!("❌ Failed to add comment: {:#}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error(
                    "comment_failed".to_string(),
                    "Failed to add comment".to_string(),
                    Some(serde_json::json!({ "error": e.to_string() })),
                )),
            ).into_response()
        }
    }
}

/// 🏷️ Add labels to issue
pub async fn add_issue_labels(
    State(app_state): State<AppState>,
    Path((owner, repo, issue_number)): Path<(String, String, u32)>,
    Json(labels): Json<Vec<String>>,
) -> Response {
    let github_client = match GitHubClient::new(&app_state.config.github.token) {
        Ok(client) => client,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error(
                    "github_client_error".to_string(),
                    "Failed to create GitHub client".to_string(),
                    None,
                )),
            ).into_response();
        }
    };

    match github_client.add_labels_to_issue(&owner, &repo, issue_number, &labels).await {
        Ok(_) => {
            info!("✅ Added labels to issue #{}: {:?}", issue_number, labels);
            (
                StatusCode::OK,
                Json(ApiResponse::<()>::success_no_data(
                    "Labels added successfully".to_string(),
                )),
            ).into_response()
        }
        Err(e) => {
            error!("❌ Failed to add labels: {:#}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error(
                    "labels_failed".to_string(),
                    "Failed to add labels".to_string(),
                    Some(serde_json::json!({ "error": e.to_string() })),
                )),
            ).into_response()
        }
    }
}

/// ✅ Close issue with comment
pub async fn close_issue_with_comment(
    State(app_state): State<AppState>,
    Path((owner, repo, issue_number)): Path<(String, String, u32)>,
    Json(payload): Json<serde_json::Value>,
) -> Response {
    let github_client = match GitHubClient::new(&app_state.config.github.token) {
        Ok(client) => client,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error(
                    "github_client_error".to_string(),
                    "Failed to create GitHub client".to_string(),
                    None,
                )),
            ).into_response();
        }
    };

    // Add final comment
    if let Some(comment) = payload.get("comment").and_then(|c| c.as_str()) {
        if let Err(e) = github_client.add_comment_to_issue(&owner, &repo, issue_number, comment).await {
            warn!("⚠️ Failed to add closing comment: {:#}", e);
        }
    }

    // Close the issue
    match github_client.close_issue(&owner, &repo, issue_number).await {
        Ok(_) => {
            info!("✅ Closed issue #{}", issue_number);
            (
                StatusCode::OK,
                Json(ApiResponse::<()>::success_no_data(
                    "Issue closed successfully".to_string(),
                )),
            ).into_response()
        }
        Err(e) => {
            error!("❌ Failed to close issue: {:#}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error(
                    "close_failed".to_string(),
                    "Failed to close issue".to_string(),
                    Some(serde_json::json!({ "error": e.to_string() })),
                )),
            ).into_response()
        }
    }
}