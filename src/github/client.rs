// 🤖 GitHub API Client - Wrapper for GitHub Operations! 🤖
// Created with love by Aye & Hue! ✨
// Making GitHub automation as smooth as butter! 🧈

use anyhow::{Context, Result};
use octocrab::models::{issues::Issue, Repository};
use octocrab::Octocrab;
use serde_json::json;
use tracing::{error, info, warn};

/// 🐙 GitHub API client wrapper
pub struct GitHubClient {
    octocrab: Octocrab,
}

impl GitHubClient {
    /// 🔧 Create a new GitHub client with authentication
    pub fn new(token: &str) -> Result<Self> {
        let octocrab = Octocrab::builder()
            .personal_token(token.to_string())
            .build()
            .context("Failed to create GitHub client")?;

        Ok(Self { octocrab })
    }

    /// 📝 Add a comment to an issue
    pub async fn add_comment_to_issue(
        &self,
        owner: &str,
        repo: &str,
        issue_number: u32,
        comment: &str,
    ) -> Result<()> {
        info!(
            "💬 Adding comment to issue #{} in {}/{}",
            issue_number, owner, repo
        );

        self.octocrab
            .issues(owner, repo)
            .create_comment(issue_number.into(), comment)
            .await
            .with_context(|| {
                format!(
                    "Failed to add comment to issue #{} in {}/{}",
                    issue_number, owner, repo
                )
            })?;

        info!("✅ Comment added successfully to issue #{}", issue_number);
        Ok(())
    }

    /// 🏷️ Add labels to an issue
    pub async fn add_labels_to_issue(
        &self,
        owner: &str,
        repo: &str,
        issue_number: u32,
        labels: &[String],
    ) -> Result<()> {
        info!(
            "🏷️ Adding labels {:?} to issue #{} in {}/{}",
            labels, issue_number, owner, repo
        );

        self.octocrab
            .issues(owner, repo)
            .add_labels(issue_number.into(), labels)
            .await
            .with_context(|| {
                format!(
                    "Failed to add labels to issue #{} in {}/{}",
                    issue_number, owner, repo
                )
            })?;

        info!("✅ Labels added successfully to issue #{}", issue_number);
        Ok(())
    }

    /// 👤 Assign an issue to a user
    pub async fn assign_issue(
        &self,
        owner: &str,
        repo: &str,
        issue_number: u32,
        assignee: &str,
    ) -> Result<()> {
        info!(
            "👤 Assigning issue #{} to {} in {}/{}",
            issue_number, assignee, owner, repo
        );

        self.octocrab
            .issues(owner, repo)
            .add_assignees(issue_number.into(), &[assignee])
            .await
            .with_context(|| {
                format!(
                    "Failed to assign issue #{} to {} in {}/{}",
                    issue_number, assignee, owner, repo
                )
            })?;

        info!("✅ Issue #{} assigned successfully to {}", issue_number, assignee);
        Ok(())
    }

    /// ✅ Close an issue
    pub async fn close_issue(&self, owner: &str, repo: &str, issue_number: u32) -> Result<()> {
        info!(
            "✅ Closing issue #{} in {}/{}",
            issue_number, owner, repo
        );

        self.octocrab
            .issues(owner, repo)
            .update(issue_number.into())
            .state(octocrab::params::State::Closed)
            .send()
            .await
            .with_context(|| {
                format!(
                    "Failed to close issue #{} in {}/{}",
                    issue_number, owner, repo
                )
            })?;

        info!("✅ Issue #{} closed successfully", issue_number);
        Ok(())
    }

    /// 🔍 Get issue details
    pub async fn get_issue(&self, owner: &str, repo: &str, issue_number: u32) -> Result<Issue> {
        info!(
            "🔍 Fetching issue #{} from {}/{}",
            issue_number, owner, repo
        );

        let issue = self
            .octocrab
            .issues(owner, repo)
            .get(issue_number.into())
            .await
            .with_context(|| {
                format!(
                    "Failed to fetch issue #{} from {}/{}",
                    issue_number, owner, repo
                )
            })?;

        info!("✅ Issue #{} fetched successfully", issue_number);
        Ok(issue)
    }

    /// 📋 List repository issues
    pub async fn list_issues(
        &self,
        owner: &str,
        repo: &str,
        state: Option<&str>,
        labels: Option<&str>,
    ) -> Result<Vec<Issue>> {
        info!("📋 Listing issues from {}/{}", owner, repo);

        let mut issues_handler = self.octocrab.issues(owner, repo);

        if let Some(state) = state {
            issues_handler = issues_handler.state(match state {
                "open" => octocrab::params::State::Open,
                "closed" => octocrab::params::State::Closed,
                _ => octocrab::params::State::All,
            });
        }

        if let Some(labels) = labels {
            issues_handler = issues_handler.labels(&[labels]);
        }

        let page = issues_handler
            .list()
            .send()
            .await
            .with_context(|| format!("Failed to list issues from {}/{}", owner, repo))?;

        info!("✅ Found {} issues in {}/{}", page.items.len(), owner, repo);
        Ok(page.items)
    }

    /// 🔗 Create a pull request
    pub async fn create_pull_request(
        &self,
        owner: &str,
        repo: &str,
        title: &str,
        body: &str,
        head: &str,
        base: &str,
    ) -> Result<octocrab::models::pulls::PullRequest> {
        info!(
            "🔗 Creating pull request from {} to {} in {}/{}",
            head, base, owner, repo
        );

        let pr = self
            .octocrab
            .pulls(owner, repo)
            .create(title, head, base)
            .body(body)
            .send()
            .await
            .with_context(|| {
                format!(
                    "Failed to create pull request from {} to {} in {}/{}",
                    head, base, owner, repo
                )
            })?;

        info!("✅ Pull request #{} created successfully", pr.number);
        Ok(pr)
    }

    /// 🏠 Get repository information
    pub async fn get_repository(&self, owner: &str, repo: &str) -> Result<Repository> {
        info!("🏠 Fetching repository {}/{}", owner, repo);

        let repository = self
            .octocrab
            .repos(owner, repo)
            .get()
            .await
            .with_context(|| format!("Failed to fetch repository {}/{}", owner, repo))?;

        info!("✅ Repository {}/{} fetched successfully", owner, repo);
        Ok(repository)
    }

    /// 🌿 Create a new branch
    pub async fn create_branch(&self, owner: &str, repo: &str, branch_name: &str, from_sha: &str) -> Result<()> {
        info!(
            "🌿 Creating branch {} from {} in {}/{}",
            branch_name, from_sha, owner, repo
        );

        self.octocrab
            .repos(owner, repo)
            .create_ref(&format!("refs/heads/{}", branch_name), from_sha)
            .await
            .with_context(|| {
                format!(
                    "Failed to create branch {} in {}/{}",
                    branch_name, owner, repo
                )
            })?;

        info!("✅ Branch {} created successfully", branch_name);
        Ok(())
    }

    /// 📝 Update file content in repository
    pub async fn update_file(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
        content: &str,
        message: &str,
        branch: &str,
        sha: Option<&str>,
    ) -> Result<()> {
        info!(
            "📝 Updating file {} in branch {} of {}/{}",
            path, branch, owner, repo
        );

        let encoded_content = base64::encode(content);

        let mut request = self
            .octocrab
            .repos(owner, repo)
            .update_file(path, message, &encoded_content, sha.unwrap_or(""));

        if branch != "main" && branch != "master" {
            request = request.branch(branch);
        }

        request
            .send()
            .await
            .with_context(|| {
                format!(
                    "Failed to update file {} in {}/{}",
                    path, owner, repo
                )
            })?;

        info!("✅ File {} updated successfully", path);
        Ok(())
    }

    /// 🔍 Check if user is a collaborator
    pub async fn is_collaborator(&self, owner: &str, repo: &str, username: &str) -> Result<bool> {
        info!(
            "🔍 Checking if {} is a collaborator on {}/{}",
            username, owner, repo
        );

        match self
            .octocrab
            .repos(owner, repo)
            .collaborators()
            .check_permissions(username)
            .await
        {
            Ok(_) => {
                info!("✅ {} is a collaborator on {}/{}", username, owner, repo);
                Ok(true)
            }
            Err(_) => {
                info!("❌ {} is not a collaborator on {}/{}", username, owner, repo);
                Ok(false)
            }
        }
    }
}
