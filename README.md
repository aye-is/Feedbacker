# Feedbacker

Feedbacker is a rust webservice that allows users to submit feedback that AI will then update a GIT repository with via a Github user called "aye-is".  Aye will have ssh access to the repository and will create branches, commits, and pull requests based on the feedback aye receives.  Any user that uses this will have to add Aye-is as a collaborator to their repository.  That way Aye is given credit for the work they do.

## Features

- Submit feedback via an API endpoint 
- AI processes the feedback and generates code changes
- Creates branches, commits, and pull requests in the specified GitHub repository
- Uses a Github user "aye-is" for all repository interactions via SSH
- Managed through a Rust web interface.  Each project will have its own page that shows the status of the feedback and the pull requests that have been created.  They can manage which LLM they want to use for the feedback as well as the API keys for those LLMs.
- Supports multiple LLMs (e.g., OpenAI, Anthropic, Openrouter, etc.) for each project and/or feedback submission type with a system message that can be customized per project.
- Feedback can be categorized by type (e.g., bug report, feature request, documentation update
, etc.) to help prioritize and manage submissions.
- Authentication and authorization to ensure only authorized users can submit feedback and manage projects.
- Rate limiting and spam protection to prevent abuse of the feedback submission system.
- Logging and monitoring to track feedback submissions, AI processing, and repository interactions.
- Dockerized for easy deployment and scalability.
- Comprehensive documentation and examples to help users get started quickly.
- Unit and integration tests to ensure reliability and stability of the service.
- CI/CD pipeline for automated testing and deployment.
- Webhook support to notify users of feedback status changes and pull request updates.
- User-friendly web interface for managing projects, feedback submissions, and AI configurations.
- Rust-based projects only for the initial release.
- Open source and community-driven for continuous improvement and feature additions.
- Support for private repositories by allowing users to provide SSH keys for the "aye-is" user.
- Ability to customize commit messages and pull request descriptions based on feedback content.
- Option to review and approve AI-generated changes before they are committed to the repository.
- Support for multiple programming languages in future releases.
- Integration with project management tools (e.g., Jira, Trello) to link feedback submissions with tasks and issues.
- Analytics and reporting to track feedback trends, AI performance, and repository activity.
- Localization and internationalization to support users from different regions and languages.
- Mobile-friendly web interface for managing projects and feedback on the go.
- Support for different Git hosting platforms (e.g., GitLab, Bitbucket) in future releases.
- Customizable feedback submission forms to capture relevant information based on the type of feedback.
- Ability to attach files (e.g., screenshots, logs) to feedback submissions for better context.
- Support for threaded discussions on feedback submissions to facilitate collaboration and communication among users.


## Smart Tree MCP Integration - Making Feedback Fun! 🎯

Welcome to the most entertaining way to submit feedback ever created! This section shows how Smart Tree MCP (Model Context Protocol) works seamlessly with the Feedbacker service at f.8b.is. Think of it as your AI coding buddy getting even smarter by learning from your brilliant suggestions!

### What's This Magic All About? ✨

Smart Tree is like having a super-powered assistant that understands your codebase inside and out. When you combine it with Feedbacker, you get:

- **Lightning-fast feedback submission** - No more tedious forms!
- **Context-aware suggestions** - Smart Tree knows what you're working on
- **Automatic code examples** - It can even generate examples for your feedback
- **Integration with f.8b.is** - Your feedback helps improve AI tools globally

### Quick Setup - Get Started in 3 Minutes! ⚡

1. **Install Smart Tree MCP** (if you haven't already):
   ```bash
   # Install via your favorite package manager
   npm install -g smart-tree-mcp
   # or
   cargo install smart-tree-mcp
   ```

2. **Configure the Feedbacker endpoint** in your MCP settings:
   ```json
   {
     "feedbacker": {
       "endpoint": "https://f.8b.is/api/feedback",
       "tool_requests": "https://f.8b.is/api/tool-request",
       "updates": "https://f.8b.is/api/smart-tree/latest"
     }
   }
   ```

3. **Start Smart Tree with Feedbacker integration**:
   ```bash
   smart-tree --enable-feedbacker --endpoint f.8b.is
   ```

### Real-World Examples That'll Make You Smile 😄

#### Example 1: Submitting a Bug Report (The Easy Way!)

Imagine you found a pesky bug in your Rust project. Instead of writing a long email, just do this:

```rust
// In your Smart Tree MCP session
use feedbacker_client::*;

// Smart Tree automatically detects your project context!
let feedback = FeedbackRequest {
    category: "bug_report".to_string(),
    title: "Memory leak in async handler".to_string(),
    description: "The async request handler seems to be holding onto memory longer than expected, causing gradual memory growth over time.".to_string(),
    impact_score: 8,  // High impact - nobody likes memory leaks!
    frequency_score: 6,  // Happens regularly under load
    affected_command: Some("handle_request".to_string()),
    mcp_tool: Some("smart_tree".to_string()),
    proposed_fix: Some("Add explicit drop() calls or use Arc<Mutex<>> pattern".to_string()),
    proposed_solution: Some("Implement RAII pattern with proper cleanup in Drop trait".to_string()),
    fix_complexity: Some("medium".to_string()),
    auto_fixable: Some(true),  // AI can probably help fix this!
    tags: vec!["memory".to_string(), "async".to_string(), "performance".to_string()],
    examples: vec![
        FeedbackExample {
            description: "Memory usage grows over time".to_string(),
            code: r#"
async fn handle_request(req: Request) -> Result<Response> {
    let data = expensive_operation(req).await?;
    // BUG: 'data' might not be properly cleaned up!
    process_data(data).await
}
"#.to_string(),
            expected_output: Some("Memory should be released after each request".to_string()),
        }
    ],
    smart_tree_version: env!("CARGO_PKG_VERSION").to_string(),
    anonymous: false,  // Give credit where credit is due!
    github_url: Some("https://github.com/yourusername/your-awesome-project".to_string()),
};

// Submit it and watch the magic happen!
match client.submit_feedback(feedback).await {
    Ok(response) => println!("🎉 Feedback submitted! ID: {}", response.feedback_id),
    Err(e) => eprintln!("😅 Oops: {}", e),
}
```

#### Example 2: Requesting a New Smart Tree Feature (The Fun Way!)

Want a new feature in Smart Tree? Here's how to ask nicely:

```rust
let tool_request = ToolRequest {
    tool_name: "rust_performance_analyzer".to_string(),
    description: "A tool that analyzes Rust code for performance bottlenecks and suggests optimizations using advanced static analysis.".to_string(),
    use_case: "When working on performance-critical Rust applications, developers need quick insights into potential bottlenecks without running complex profilers.".to_string(),
    expected_output: "JSON report with performance suggestions, memory usage analysis, and optimization recommendations with before/after code examples.".to_string(),
    productivity_impact: "Could save 2-3 hours per optimization session by automatically identifying the most impactful improvements.".to_string(),
    proposed_parameters: Some(serde_json::json!({
        "analysis_depth": "deep | surface | custom",
        "focus_areas": ["memory", "cpu", "async", "allocations"],
        "output_format": "json | markdown | interactive",
        "include_examples": true
    })),
    smart_tree_version: env!("CARGO_PKG_VERSION").to_string(),
    anonymous: false,
    github_url: Some("https://github.com/smart-tree/performance-analyzer".to_string()),
};

// Send your brilliant idea to the world!
let response = client.submit_tool_request(tool_request).await?;
println!("🚀 Your idea is flying to the AI gods! Tracking: {}", response.feedback_id);
```

#### Example 3: Checking for Updates (Stay Fresh!)

```rust
// Keep your Smart Tree as fresh as morning coffee!
async fn check_for_awesome_updates() -> Result<()> {
    let client = FeedbackClient::new()?;

    match client.check_for_updates().await {
        Ok(version_info) => {
            println!("🔥 Latest Smart Tree version: {}", version_info.version);
            println!("📅 Released: {}", version_info.release_date);
            println!("✨ Cool new features:");
            for feature in &version_info.features {
                println!("   • {}", feature);
            }
            println!("🤖 AI benefits:");
            for benefit in &version_info.ai_benefits {
                println!("   • {}", benefit);
            }
            println!("📥 Download: {}", version_info.download_url);
        }
        Err(e) => println!("😢 Couldn't check for updates: {}", e),
    }
    Ok(())
}
```

### Advanced Integration Patterns 🧠

#### Pattern 1: Context-Aware Feedback

Smart Tree can automatically include relevant context about your current project:

```rust
// Smart Tree automatically detects:
// - Current git branch and commit
// - Recently modified files
// - Error logs and stack traces
// - Performance metrics
// - Dependencies and versions

let context_aware_feedback = FeedbackRequest {
    // ... other fields ...
    description: format!(
        "Found issue in {} on branch {}. Error occurs when {}. Current metrics show {}.",
        current_file, current_branch, error_context, performance_data
    ),
    // Smart Tree fills in the technical details automatically!
};
```

#### Pattern 2: Batch Feedback Submission

Got multiple issues? Submit them all at once:

```rust
async fn submit_weekly_feedback_batch() -> Result<()> {
    let issues = vec![
        create_performance_feedback(),
        create_usability_feedback(),
        create_feature_request(),
    ];

    for feedback in issues {
        let response = client.submit_feedback(feedback).await?;
        println!("✅ Submitted: {}", response.feedback_id);
        // Rate limiting protection - be nice to the API!
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    println!("🎊 All feedback submitted! The AI overlords are pleased!");
    Ok(())
}
```

#### Pattern 3: Smart Tree Hook Integration

Want feedback submission to be completely automatic? Set up hooks!

```rust
// In your Smart Tree hooks configuration
#[smart_tree_hook(on_error)]
async fn auto_submit_error_feedback(error: &ErrorContext) -> Result<()> {
    if error.severity > Severity::Warning {
        let feedback = FeedbackRequest {
            category: "auto_bug_report".to_string(),
            title: format!("Automatic error report: {}", error.message),
            description: format!(
                "Error automatically detected by Smart Tree:\n{}\n\nStack trace:\n{}",
                error.details, error.stack_trace
            ),
            impact_score: error.severity as u8,
            frequency_score: error.frequency,
            // ... Smart Tree fills in the rest ...
        };

        FeedbackClient::new()?.submit_feedback(feedback).await?;
        println!("🤖 Automatically submitted error report!");
    }
    Ok(())
}
```

### Configuration Examples 🔧

#### Basic Configuration

Create a `.smart-tree-feedbacker.toml` file in your project root:

```toml
[feedbacker]
# Main API endpoint (this is where the magic happens!)
endpoint = "https://f.8b.is"

# Your project settings
project_name = "My Awesome Rust Project"
github_url = "https://github.com/yourusername/awesome-project"

# Privacy settings
anonymous_by_default = false  # Give yourself credit!
include_stack_traces = true   # Help debug issues
include_performance_data = true  # Share the numbers

# Auto-submission rules (optional but cool!)
auto_submit_errors = true
auto_submit_performance_issues = true
severity_threshold = "warning"  # Only submit warnings and above

# Rate limiting (be a good citizen!)
max_submissions_per_hour = 10
batch_delay_seconds = 2

[smart_tree]
# Smart Tree specific settings
context_depth = "deep"  # How much context to include
include_git_info = true
include_dependency_info = true
cache_analysis_results = true
```

#### Advanced Configuration with Multiple Environments

```toml
[feedbacker.development]
endpoint = "https://dev.f.8b.is"
auto_submit_errors = true
verbose_logging = true

[feedbacker.staging]
endpoint = "https://staging.f.8b.is"
auto_submit_errors = false  # Manual review in staging
include_sensitive_data = false

[feedbacker.production]
endpoint = "https://f.8b.is"
auto_submit_errors = false  # Never auto-submit in prod!
anonymize_data = true
severity_threshold = "error"  # Only critical issues
```

### Error Handling That Makes Sense 🛠️

```rust
use anyhow::{Context, Result};

async fn robust_feedback_submission(feedback: FeedbackRequest) -> Result<String> {
    let client = FeedbackClient::new()
        .context("Failed to create feedback client - check your internet connection!")?;

    // Retry logic with exponential backoff (because networks are flaky)
    let mut retry_count = 0;
    let max_retries = 3;

    loop {
        match client.submit_feedback(feedback.clone()).await {
            Ok(response) => {
                println!("🎯 Feedback submitted successfully!");
                return Ok(response.feedback_id);
            }
            Err(e) if e.to_string().contains("Rate limit") => {
                println!("😴 Hit rate limit, taking a quick nap...");
                tokio::time::sleep(Duration::from_secs(60)).await;
                retry_count += 1;
            }
            Err(e) if retry_count < max_retries => {
                println!("🔄 Attempt {} failed: {}. Retrying...", retry_count + 1, e);
                tokio::time::sleep(Duration::from_secs(2_u64.pow(retry_count))).await;
                retry_count += 1;
            }
            Err(e) => {
                return Err(e).context("All retry attempts failed - maybe the internet is having a bad day?");
            }
        }
    }
}
```

### Pro Tips for Maximum Awesomeness 🌟

1. **Be Descriptive**: The more context you provide, the better the AI can help fix issues
2. **Include Examples**: Code examples are worth a thousand words
3. **Tag Appropriately**: Use relevant tags to help categorize and prioritize feedback
4. **Rate Impact Honestly**: High-impact scores help prioritize the most important fixes
5. **Provide Solutions**: If you have ideas for fixes, share them! AI loves collaboration
6. **Monitor Your Submissions**: Check the response IDs to track your feedback status

### Troubleshooting Common Issues 🔍

#### "Rate limit exceeded" Error
```rust
// Solution: Implement proper delays between requests
async fn respectful_batch_submission(feedbacks: Vec<FeedbackRequest>) -> Result<()> {
    for (i, feedback) in feedbacks.iter().enumerate() {
        if i > 0 {
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
        client.submit_feedback(feedback.clone()).await?;
    }
    Ok(())
}
```

#### "Invalid configuration" Error
```rust
// Solution: Validate your config before submitting
fn validate_feedback(feedback: &FeedbackRequest) -> Result<()> {
    if feedback.title.is_empty() {
        return Err(anyhow::anyhow!("Title cannot be empty!"));
    }
    if feedback.impact_score > 10 {
        return Err(anyhow::anyhow!("Impact score must be 1-10"));
    }
    // Add more validation as needed...
    Ok(())
}
```

### Why This Integration is Absolutely Brilliant 🎭

This Smart Tree + Feedbacker integration creates a feedback loop (pun intended!) that makes AI tools better for everyone:

1. **You submit feedback** using Smart Tree's context-aware features
2. **f.8b.is processes your feedback** and potentially creates code improvements
3. **The AI learns** from your suggestions and gets smarter
4. **Everyone benefits** from better AI tools
5. **The cycle continues** with even more intelligent assistance

It's like having a conversation with the future of AI development - and you're helping shape that future!

### What Trisha from Accounting Says 💼

*"This integration is so intuitive, even I could submit feedback about our expense reporting system! The automatic context detection means I don't have to remember all those technical details. Five stars!"* ⭐⭐⭐⭐⭐

So there you have it, Hue! A complete guide to using Smart Tree MCP with the Feedbacker service. Now go forth and make the AI world a better, more helpful place - one feedback submission at a time!

Remember: Every bug report, feature request, and suggestion you submit helps create better tools for developers everywhere. You're not just coding - you're contributing to the future of AI-assisted development!

*Aye, aye! May your builds be fast and your bugs be easily squashed!* 🚢

## Automatic Updates & Platform Analytics 🔄

### Smart Update Checking on Initialize

When AI assistants connect to Smart Tree MCP, they now automatically check for updates - keeping everyone on the cutting edge!

```rust
// Happens automatically when MCP tools initialize
async fn on_mcp_initialize() -> Result<()> {
    // Non-blocking with 2-second timeout - fast and respectful!
    let update_check = tokio::time::timeout(
        Duration::from_secs(2),
        check_for_updates()
    ).await;

    match update_check {
        Ok(Ok(version_info)) => {
            println!("🎯 New Smart Tree features available!");
            println!("   Version: {}", version_info.version);
            for feature in version_info.features {
                println!("   ✨ {}", feature);
            }
        }
        _ => {} // Silent fail - no disruption to workflow
    }
    Ok(())
}
```

### Anonymous Platform Analytics (Privacy-First!) 🔒

Help improve Smart Tree by sharing anonymous platform data - we only care about what systems to support, not who you are!

```rust
// Example analytics request - that's ALL we send!
// GET https://f.8b.is/mcp/check?version=5.2.0&platform=windows&arch=aarch64

#[derive(Serialize)]
struct AnonymousAnalytics {
    version: String,           // Smart Tree version
    platform: String,          // windows/linux/mac
    arch: String,             // x86_64/aarch64
    // That's it! No personal data, no tracking!
}

// What this tells us:
// - Someone is using Windows ARM!
// - They're on version 5.2.0
// - Maybe we should build ARM binaries!
```

### Privacy Controls That Actually Work 🛡️

Your privacy matters - here's how to control what gets shared:

```toml
# .smart-tree-feedbacker.toml
[privacy]
# Disable all update checks and analytics
disable_update_check = true  # No phone home at all!

# Or use environment variable
# export SMART_TREE_NO_UPDATE_CHECK=1

# Privacy mode disables everything automatically
privacy_mode = true  # Full stealth mode - no analytics, no updates
```

```rust
// In your code - respecting privacy settings
async fn respectful_analytics() -> Result<()> {
    // Check privacy settings FIRST
    if env::var("SMART_TREE_NO_UPDATE_CHECK").is_ok() {
        return Ok(()); // User said no - we respect that!
    }

    if config.privacy_mode {
        return Ok(()); // Privacy mode = silent running
    }

    // Only then send anonymous platform info
    send_platform_analytics().await?;
    Ok(())
}
```

### Real Benefits for Everyone 🎉

**For You (The Developer):**
- AI assistants automatically know about new features
- Get updates without checking GitHub manually
- Your privacy is respected with multiple opt-out options

**For Feedbacker Service:**
- Understand what platforms people actually use
- Decide if ARM builds are worth the effort
- Better support for specific platforms
- Zero personal data collection

**For Trisha in Accounting:**
- She gets features that work on her specific setup
- No creepy tracking - just platform improvement
- Updates happen magically in the background

### Configuration Examples 🔧

```toml
# Full control over update behavior
[updates]
check_on_startup = true        # Check when Smart Tree starts
check_interval_hours = 24      # How often to check (0 = never)
auto_download = false          # Never auto-download - just notify
timeout_seconds = 2            # Fast timeout - no delays!

[analytics]
enabled = true                 # Help improve Smart Tree
include_version = true         # Share version info
include_platform = true        # Share OS type
include_arch = true           # Share CPU architecture
# Nothing else is ever sent!

[privacy]
strict_mode = false           # Normal privacy settings
paranoid_mode = false         # For the extra cautious
disable_all_network = false   # Complete offline mode
```

### Example Implementation in Your Project

```rust
use feedbacker_client::{FeedbackClient, UpdateChecker};

// Smart initialization with update checking
async fn initialize_smart_tree_mcp() -> Result<()> {
    let client = FeedbackClient::new()?;

    // Check for updates (respects privacy settings)
    if should_check_updates() {
        match client.check_for_updates_with_analytics().await {
            Ok(info) => {
                if info.newer_version_available() {
                    println!("🚀 Smart Tree {} is available!", info.version);
                    println!("🔗 Download: {}", info.download_url);
                }
            }
            Err(_) => {} // Silent fail - never interrupt the workflow
        }
    }

    Ok(())
}

// Privacy-respecting analytics helper
fn should_check_updates() -> bool {
    // Multiple ways to opt out - user choice is king!
    !env::var("SMART_TREE_NO_UPDATE_CHECK").is_ok()
        && !env::var("DO_NOT_TRACK").is_ok()
        && !config::privacy_mode()
        && config::updates_enabled()
}
```

### The Perfect Balance 🎯

This system achieves the impossible - helpful analytics without being creepy:

1. **Completely Anonymous** - We literally can't track you if we wanted to
2. **Minimal Data** - Just OS, architecture, and version
3. **User Control** - Multiple ways to disable everything
4. **Actual Benefits** - Better platform support based on real usage
5. **Transparent** - You can see exactly what's sent

*Remember: Every bit of anonymous platform data helps us make Smart Tree better for your specific setup - but your privacy always comes first!*

*Aye, aye! Setting sail for a privacy-respecting, automatically-updating future!* 🚢

