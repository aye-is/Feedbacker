# 🎯 GitHub Issue Automation Setup

This guide shows you how to set up GitHub issue automation with the Feedbacker service!

## 🚀 What This Does

The Feedbacker service automatically:
- **Welcomes new issues** with helpful comments
- **Auto-labels issues** based on content analysis
- **Assigns issues** to appropriate team members
- **Thanks users** when issues are closed
- **Provides manual management** endpoints for issue operations

## 🔧 Setting Up Issue Webhooks

### 1. Configure Your Repository

1. Go to your repository on GitHub
2. Click **Settings** → **Webhooks** → **Add webhook**
3. Set the webhook URL: `https://your-feedbacker-domain.com/api/webhook/issues`
4. Set **Content type** to `application/json`
5. Set **Secret** to your webhook secret (from `.env` file)
6. Select **Individual events** and check:
   - ✅ Issues
   - ✅ Issue comments
   - ✅ Pull requests (optional)

### 2. Environment Configuration

Make sure your `.env` file has:

```bash
# GitHub Configuration
GITHUB_TOKEN=ghp_your_token_here    # Token for aye-is account
GITHUB_USERNAME=aye-is
GITHUB_EMAIL=aye@8b.is

# Webhook Secret (generate with: openssl rand -hex 32)
WEBHOOK_SECRET=your-webhook-secret-here
```

## 🎫 How Issue Automation Works

### 🆕 When Someone Opens an Issue

**Automatic Actions:**
1. **🔍 Content Analysis** - Analyzes title and body for keywords
2. **🏷️ Smart Labeling** - Applies relevant labels:
   - `bug` - Contains: bug, error, crash, fail
   - `enhancement` - Contains: feature, enhancement, request
   - `documentation` - Contains: documentation, docs, readme
   - `question` - Contains: how to, help, question, ends with ?
   - `performance` - Contains: performance, slow, speed

3. **💬 Welcome Comment** - Adds personalized welcome message
4. **👤 Auto Assignment** - Assigns based on issue type:
   - Documentation issues → `aye-is`
   - Critical/urgent issues → `aye-is`

**Example Welcome Comment:**
```
## 🐛 **Bug Report**

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

Thanks for helping make Feedbacker better!

*Aye, aye! 🚢*
```

### ✅ When Someone Closes an Issue

**Automatic Actions:**
1. **💬 Thank You Comment** - Adds appreciation message with service promotion

### 🏷️ When Labels Are Added

**Smart Responses:**
- `needs-info` label → Could trigger request for more details
- `question` label → May get priority for quick response

## 🔧 Manual Issue Management

### API Endpoints Available

**Add Comment to Issue:**
```bash
POST /api/issues/{owner}/{repo}/{issue_number}/comment
Content-Type: application/json

{
  "body": "Thanks for reporting this! We're investigating..."
}
```

**Add Labels to Issue:**
```bash
POST /api/issues/{owner}/{repo}/{issue_number}/labels
Content-Type: application/json

["bug", "priority-high", "good-first-issue"]
```

**Close Issue with Comment:**
```bash
POST /api/issues/{owner}/{repo}/{issue_number}/close
Content-Type: application/json

{
  "comment": "Fixed in latest release! Thanks for reporting."
}
```

## 🎨 Customizing Automation

### Modify Label Detection

Edit `src/api/issue_hooks.rs` in the `analyze_issue_for_labels` function:

```rust
// Add custom label detection
if content_lower.contains("security") ||
   content_lower.contains("vulnerability") {
    labels.push("security".to_string());
}
```

### Customize Welcome Messages

Edit the `create_welcome_comment` function to change the greeting:

```rust
let issue_type = if issue.title.to_lowercase().contains("feature") {
    "✨ **Feature Request**"
} else if issue.title.to_lowercase().contains("security") {
    "🔒 **Security Issue**"  // Add custom types
} else {
    "🎫 **Issue**"
};
```

### Configure Auto-Assignment

Edit `determine_auto_assignee` function:

```rust
// Auto-assign based on labels or content
if content_lower.contains("frontend") || content_lower.contains("ui") {
    Some("frontend-team-lead".to_string())
} else if content_lower.contains("backend") || content_lower.contains("api") {
    Some("backend-team-lead".to_string())
} else {
    None
}
```

## 🔍 Monitoring and Logs

The service logs all issue automation activities:

```bash
# Watch automation logs
docker logs -f feedbacker-service | grep "issue"

# Example log output:
INFO  🎫 Received GitHub issue webhook: opened for issue #42 in user/repo
INFO  🆕 Processing newly opened issue #42
INFO  🏷️ Adding labels ["bug", "priority-medium"] to issue #42
INFO  💬 Adding comment to issue #42
INFO  ✅ Issue automation completed for #42
```

## 🧪 Testing Your Setup

### Test Webhook Delivery

1. **Create a test issue** in your repository
2. **Check webhook deliveries** in GitHub Settings → Webhooks
3. **Verify automation** ran by checking:
   - Issue received welcome comment
   - Appropriate labels were applied
   - Issue was assigned if applicable

### Manual Testing Commands

```bash
# Test adding a comment
curl -X POST https://your-domain.com/api/issues/owner/repo/123/comment \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer your-api-token" \
  -d '{"body": "This is a test comment from Feedbacker!"}'

# Test adding labels
curl -X POST https://your-domain.com/api/issues/owner/repo/123/labels \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer your-api-token" \
  -d '["bug", "automation-test"]'
```

## 🎯 Best Practices

### 1. **Label Management**
- Create consistent label names across repositories
- Use color coding for different types (bugs=red, features=green)
- Document your labeling system for contributors

### 2. **Assignment Strategy**
- Don't auto-assign everything - leave room for manual triage
- Use auto-assignment for urgent or specific-domain issues
- Consider team capacity when configuring assignments

### 3. **Message Tone**
- Keep welcome messages friendly but informative
- Include clear next steps for issue reporters
- Add links to your project's contributing guidelines

### 4. **Webhook Security**
- Always use the webhook secret for verification
- Monitor webhook delivery failures
- Set up alerts for automation failures

## 🎉 Advanced Features

### Issue Templates Integration

Create `.github/ISSUE_TEMPLATE/bug_report.yml`:

```yaml
name: Bug Report
description: File a bug report to help us improve
title: "[Bug]: "
labels: ["bug", "needs-triage"]
body:
  - type: markdown
    attributes:
      value: |
        Thanks for taking the time to fill out this bug report!
        Our automation will label and assign this appropriately.
```

### Custom Automation Workflows

Combine with GitHub Actions for advanced workflows:

```yaml
name: Issue Automation Enhancement
on:
  issues:
    types: [labeled]

jobs:
  advanced-automation:
    if: contains(github.event.label.name, 'priority-critical')
    runs-on: ubuntu-latest
    steps:
      - name: Notify Team
        run: |
          curl -X POST https://your-feedbacker-domain.com/api/notifications/critical-issue \
            -d '{"issue_number": "${{ github.event.issue.number }}"}'
```

## 🆘 Troubleshooting

### Common Issues

**Webhook not firing:**
- Check webhook URL is correct and accessible
- Verify webhook secret matches your configuration
- Check GitHub webhook delivery logs

**Labels not applying:**
- Ensure the aye-is user has permission to manage labels
- Check that labels exist in the repository
- Verify GitHub token has appropriate scopes

**Comments not appearing:**
- Confirm aye-is user has write access to repository
- Check API token permissions
- Look for rate limiting in logs

### Debug Mode

Enable debug logging in your `.env`:
```bash
RUST_LOG=debug,feedbacker=trace
```

This will show detailed webhook processing information.

---

*🚢 Happy issue automating! May your repositories be organized and your users be delighted!*

*- Aye & Hue*