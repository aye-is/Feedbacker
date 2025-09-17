# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Feedbacker is a Rust web service that enables AI-driven repository management through user feedback. The service processes feedback submissions and automatically creates branches, commits, and pull requests via a GitHub user called "aye-is" with SSH access.

## Architecture

The project is in its initial planning stage with a comprehensive feature roadmap outlined in README.md. The core component currently implemented is:

- **Feedback API Client** (`examples/feedback_client.rs`): Handles communication with f.8t.is for feedback submission and version checking
  - Endpoints:
    - POST https://f.8t.is/api/feedback
    - GET https://f.8t.is/api/smart-tree/latest
  - Built with reqwest, serde, and anyhow for error handling

## Development Setup

Since this is a Rust project in early stages without a Cargo.toml yet, when implementing:

1. **Initialize Rust project**: Create `Cargo.toml` with appropriate dependencies based on the feedback client example
2. **Required dependencies** will include:
   - `reqwest` for HTTP client
   - `serde` and `serde_json` for serialization
   - `anyhow` for error handling
   - Web framework (likely `axum` or `actix-web` based on the service requirements)

## Key Features to Implement

The README outlines extensive features that need implementation:
- Web service with API endpoints for feedback submission
- GitHub integration via SSH for the "aye-is" user
- Multi-LLM support with customizable system messages per project
- Authentication/authorization system
- Rate limiting and spam protection
- Web interface for project management
- Docker containerization
- CI/CD pipeline

## Testing Approach

The feedback client includes basic unit tests. Expand testing to cover:
- Unit tests for all modules
- Integration tests for API endpoints
- Mock tests for GitHub operations
- End-to-end tests for the full feedback-to-PR workflow

## Important Context

- The service is designed to be Rust-only for initial release
- Each project gets its own management page showing feedback status and PRs
- Users must add "aye-is" as a collaborator to their repositories
- Support for private repositories via SSH keys
- Webhook support for status updates