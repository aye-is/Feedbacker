# Feedbacker

Feedbacker is a rust webservice that allows users to submit feedback that AI will then update a GIT repository with via a Github user called "aye-is".  It will have ssh access to the repository and will create branches, commits, and pull requests based on the feedback it receives.  Any user that uses this will have to add Aye-is as a collaborator to their repository.  That way Aye is given credit for the work they do.

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


## Smart tree example MCP usage for the Feedbacker project

```rust