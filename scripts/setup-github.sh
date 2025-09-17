#!/bin/bash
# 🚢 GitHub Setup Script for aye-is Account
# This script sets up SSH keys and git configuration for the Feedbacker service

set -e  # Exit on any error

# Colors for beautiful output!
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Emojis for fun!
SHIP="🚢"
CHECK="✅"
KEY="🔑"
ROCKET="🚀"
GITHUB="🐙"

echo -e "${CYAN}${SHIP} Welcome to the Feedbacker GitHub Setup!${NC}"
echo -e "${BLUE}Setting up SSH keys and git configuration for aye-is${NC}"
echo ""

# Function to generate SSH key
generate_ssh_key() {
    echo -e "${YELLOW}${KEY} Generating SSH key for aye-is...${NC}"

    # Check if key already exists
    if [ -f "./keys/aye-is_rsa" ]; then
        echo -e "${YELLOW}SSH key already exists. Backing up...${NC}"
        mv ./keys/aye-is_rsa ./keys/aye-is_rsa.backup.$(date +%s)
        mv ./keys/aye-is_rsa.pub ./keys/aye-is_rsa.pub.backup.$(date +%s)
    fi

    # Generate new SSH key
    ssh-keygen -t rsa -b 4096 -C "aye@8b.is" -f ./keys/aye-is_rsa -N ""

    echo -e "${GREEN}${CHECK} SSH key generated successfully!${NC}"
    echo ""
    echo -e "${CYAN}Public key (add this to GitHub as a deploy key or to aye-is account):${NC}"
    echo -e "${PURPLE}================================================${NC}"
    cat ./keys/aye-is_rsa.pub
    echo -e "${PURPLE}================================================${NC}"
}

# Function to setup git configuration
setup_git_config() {
    echo ""
    echo -e "${YELLOW}${GITHUB} Setting up git configuration...${NC}"

    # Create a local git config for this repository
    git config user.name "aye-is"
    git config user.email "aye@8b.is"

    # Set up SSH command to use our specific key
    git config core.sshCommand "ssh -i $(pwd)/keys/aye-is_rsa -o StrictHostKeyChecking=no"

    echo -e "${GREEN}${CHECK} Git configuration set!${NC}"
}

# Function to test GitHub connection
test_github_connection() {
    echo ""
    echo -e "${YELLOW}${ROCKET} Testing GitHub connection...${NC}"

    # Test SSH connection to GitHub
    if ssh -i ./keys/aye-is_rsa -o StrictHostKeyChecking=no -T git@github.com 2>&1 | grep -q "successfully authenticated"; then
        echo -e "${GREEN}${CHECK} GitHub SSH connection successful!${NC}"
    else
        echo -e "${YELLOW}Note: SSH key needs to be added to GitHub account or repository${NC}"
        echo -e "${CYAN}Add the public key above to:${NC}"
        echo -e "  1. GitHub account settings -> SSH keys (for full access)"
        echo -e "  2. OR Repository settings -> Deploy keys (for specific repo)"
    fi
}

# Function to create git hooks
setup_git_hooks() {
    echo ""
    echo -e "${YELLOW}Setting up git hooks...${NC}"

    # Create hooks directory if it doesn't exist
    mkdir -p .git/hooks

    # Pre-commit hook for running tests
    cat > .git/hooks/pre-commit << 'EOF'
#!/bin/bash
# 🚢 Pre-commit hook - Run checks before committing

echo "🔍 Running pre-commit checks..."

# Check if cargo is available
if command -v cargo &> /dev/null; then
    # Format check
    echo "📝 Checking code formatting..."
    cargo fmt --check

    # Run clippy
    echo "🔎 Running clippy..."
    cargo clippy -- -D warnings

    # Run tests
    echo "🧪 Running tests..."
    cargo test --quiet
fi

echo "✅ All checks passed!"
EOF

    chmod +x .git/hooks/pre-commit
    echo -e "${GREEN}${CHECK} Git hooks configured!${NC}"
}

# Main execution
main() {
    echo -e "${PURPLE}================================================${NC}"
    echo -e "${CYAN}      Feedbacker GitHub Setup for aye-is       ${NC}"
    echo -e "${PURPLE}================================================${NC}"
    echo ""

    # Check if we're in a git repository
    if [ ! -d ".git" ]; then
        echo -e "${RED}Error: Not in a git repository!${NC}"
        echo -e "${YELLOW}Initializing git repository...${NC}"
        git init
        echo -e "${GREEN}${CHECK} Git repository initialized!${NC}"
    fi

    # Generate SSH key
    generate_ssh_key

    # Setup git configuration
    setup_git_config

    # Setup git hooks
    setup_git_hooks

    # Test connection
    test_github_connection

    echo ""
    echo -e "${GREEN}${SHIP} Setup complete!${NC}"
    echo ""
    echo -e "${CYAN}Next steps:${NC}"
    echo -e "  1. Add the SSH public key to GitHub (shown above)"
    echo -e "  2. Set the repository remote: ${YELLOW}git remote add origin git@github.com:aye-is/feedbacker.git${NC}"
    echo -e "  3. Push your code: ${YELLOW}git push -u origin main${NC}"
    echo ""
    echo -e "${PURPLE}Remember: The GitHub token in .env is for API access${NC}"
    echo -e "${PURPLE}The SSH key is for git operations (push/pull)${NC}"
    echo ""
    echo -e "${CYAN}${ROCKET} Happy coding with aye-is! ${ROCKET}${NC}"
}

# Run main function
main