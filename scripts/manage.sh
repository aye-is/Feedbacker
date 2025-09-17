#!/bin/bash

# 🚢 ⚓ FEEDBACKER MANAGEMENT SCRIPT ⚓ 🚢
# The most colorful, helpful, and organized management script ever created!
# Built with love by Aye & Hue - Making project management as fun as sailing! 🌊
# Trisha from Accounting says this is the most beautiful script she's ever seen! 📊✨

# 🎨 Color definitions for maximum visual appeal!
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
WHITE='\033[1;37m'
ORANGE='\033[0;33m'
PINK='\033[1;35m'
LIGHT_BLUE='\033[1;34m'
LIGHT_GREEN='\033[1;32m'
BOLD='\033[1m'
NC='\033[0m' # No Color

# 🎯 Emoji collection for maximum fun!
SHIP="🚢"
ANCHOR="⚓"
STAR="⭐"
ROCKET="🚀"
GEAR="⚙️"
CHECK="✅"
CROSS="❌"
WARNING="⚠️"
INFO="ℹ️"
SPARKLES="✨"
HEART="❤️"
HAMMER="🔨"
WRENCH="🔧"
FIRE="🔥"
LIGHTNING="⚡"
RAINBOW="🌈"
TADA="🎉"
COFFEE="☕"
COMPUTER="💻"

# 📊 Project configuration
PROJECT_NAME="Feedbacker"
PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CARGO_TOML="$PROJECT_DIR/Cargo.toml"
ENV_FILE="$PROJECT_DIR/.env"
LOG_DIR="$PROJECT_DIR/logs"

# 🎨 Beautiful banner function
show_banner() {
    echo -e "${CYAN}${BOLD}"
    echo "════════════════════════════════════════════════════════════════════════════════"
    echo "  ${SHIP} ${ANCHOR} FEEDBACKER PROJECT MANAGEMENT ${ANCHOR} ${SHIP}"
    echo "════════════════════════════════════════════════════════════════════════════════"
    echo -e "${PURPLE}  ${SPARKLES} AI-Powered Repository Management - Built with Rust & Love! ${SPARKLES}"
    echo -e "${LIGHT_BLUE}  ${HEART} Created by Aye & Hue - Making GitHub PRs as smooth as Elvis! ${HEART}"
    echo -e "${PINK}  ${STAR} Special thanks to Trisha from Accounting for keeping us organized! ${STAR}"
    echo -e "${CYAN}════════════════════════════════════════════════════════════════════════════════${NC}"
    echo ""
}

# 📝 Help function with colorful descriptions
show_help() {
    echo -e "${BOLD}${YELLOW}${INFO} Available Commands:${NC}"
    echo ""
    echo -e "  ${GREEN}${HAMMER} build${NC}           - Build the project (debug mode)"
    echo -e "  ${LIGHT_GREEN}${ROCKET} build-release${NC}   - Build the project (release mode, optimized)"
    echo -e "  ${BLUE}${GEAR} run${NC}             - Run the development server"
    echo -e "  ${PURPLE}${LIGHTNING} dev${NC}             - Run with auto-reload (development mode)"
    echo -e "  ${ORANGE}${FIRE} test${NC}            - Run all tests with pretty output"
    echo -e "  ${CYAN}${WRENCH} test-watch${NC}      - Run tests in watch mode"
    echo -e "  ${PINK}${SPARKLES} clippy${NC}          - Run Clippy linter (Rust's best friend!)"
    echo -e "  ${LIGHT_BLUE}${STAR} fmt${NC}             - Format code beautifully"
    echo -e "  ${RED}${CROSS} clean${NC}           - Clean build artifacts"
    echo -e "  ${YELLOW}${COFFEE} setup${NC}           - Initial project setup"
    echo -e "  ${GREEN}${CHECK} check${NC}           - Quick compile check"
    echo -e "  ${PURPLE}${COMPUTER} docker-build${NC}    - Build Docker image"
    echo -e "  ${CYAN}${SHIP} docker-run${NC}      - Run Docker container"
    echo -e "  ${ORANGE}${WARNING} doctor${NC}          - Health check for development environment"
    echo -e "  ${LIGHT_GREEN}${TADA} deploy${NC}          - Deploy to production (when ready!)"
    echo -e "  ${PINK}${RAINBOW} status${NC}          - Show project status"
    echo -e "  ${WHITE}${INFO} help${NC}            - Show this colorful help message"
    echo ""
    echo -e "${BOLD}${CYAN}${SPARKLES} Pro Tips:${NC}"
    echo -e "  ${LIGHT_BLUE}• Always run ${BOLD}clippy${NC}${LIGHT_BLUE} before committing - it's like having a Rust guru review your code!${NC}"
    echo -e "  ${LIGHT_GREEN}• Use ${BOLD}test-watch${NC}${LIGHT_GREEN} during development for instant feedback${NC}"
    echo -e "  ${YELLOW}• Run ${BOLD}doctor${NC}${YELLOW} if something feels wrong - it'll check everything!${NC}"
    echo -e "  ${PURPLE}• The ${BOLD}dev${NC}${PURPLE} mode will auto-reload when you save files${NC}"
    echo ""
}

# 📊 Status check function
show_status() {
    echo -e "${BOLD}${LIGHT_BLUE}${COMPUTER} Project Status:${NC}"
    echo ""

    # 📁 Check project directory
    echo -e "${CYAN}📁 Project Directory:${NC} ${PROJECT_DIR}"

    # 🦀 Check Rust version
    if command -v rustc &> /dev/null; then
        RUST_VERSION=$(rustc --version)
        echo -e "${GREEN}${CHECK} Rust:${NC} ${RUST_VERSION}"
    else
        echo -e "${RED}${CROSS} Rust:${NC} Not installed"
    fi

    # 📦 Check Cargo version
    if command -v cargo &> /dev/null; then
        CARGO_VERSION=$(cargo --version)
        echo -e "${GREEN}${CHECK} Cargo:${NC} ${CARGO_VERSION}"
    else
        echo -e "${RED}${CROSS} Cargo:${NC} Not installed"
    fi

    # 🗄️ Check if PostgreSQL is running
    if command -v psql &> /dev/null; then
        echo -e "${GREEN}${CHECK} PostgreSQL:${NC} Available"
    else
        echo -e "${YELLOW}${WARNING} PostgreSQL:${NC} Not found (may need for database)"
    fi

    # 📄 Check important files
    if [ -f "$CARGO_TOML" ]; then
        echo -e "${GREEN}${CHECK} Cargo.toml:${NC} Found"
    else
        echo -e "${RED}${CROSS} Cargo.toml:${NC} Missing!"
    fi

    if [ -f "$ENV_FILE" ]; then
        echo -e "${GREEN}${CHECK} .env file:${NC} Found"
    else
        echo -e "${YELLOW}${WARNING} .env file:${NC} Not found (copy from .env.example)"
    fi

    # 📊 Project stats
    if [ -d "$PROJECT_DIR/src" ]; then
        RUST_FILES=$(find "$PROJECT_DIR/src" -name "*.rs" | wc -l)
        echo -e "${PURPLE}${STAR} Rust Files:${NC} ${RUST_FILES}"
    fi

    if [ -d "$PROJECT_DIR/target" ]; then
        TARGET_SIZE=$(du -sh "$PROJECT_DIR/target" 2>/dev/null | cut -f1)
        echo -e "${CYAN}${GEAR} Target Size:${NC} ${TARGET_SIZE}"
    fi

    echo ""
}

# 🔨 Build function with progress indication
build_project() {
    local mode=${1:-debug}
    echo -e "${BOLD}${GREEN}${HAMMER} Building Feedbacker (${mode} mode)...${NC}"
    echo ""

    cd "$PROJECT_DIR" || exit 1

    if [ "$mode" = "release" ]; then
        echo -e "${YELLOW}${ROCKET} Building optimized release version...${NC}"
        cargo build --release
    else
        echo -e "${BLUE}${GEAR} Building debug version...${NC}"
        cargo build
    fi

    if [ $? -eq 0 ]; then
        echo ""
        echo -e "${GREEN}${CHECK} Build completed successfully! ${TADA}${NC}"
        echo -e "${CYAN}${INFO} Binary location: target/${mode}/feedbacker${NC}"
    else
        echo ""
        echo -e "${RED}${CROSS} Build failed! ${WARNING}${NC}"
        echo -e "${YELLOW}${INFO} Try running '${BOLD}./manage.sh doctor${NC}${YELLOW}' to diagnose issues${NC}"
        exit 1
    fi
}

# 🚀 Run function
run_project() {
    echo -e "${BOLD}${BLUE}${ROCKET} Starting Feedbacker development server...${NC}"
    echo ""

    cd "$PROJECT_DIR" || exit 1

    # 🔍 Check if .env file exists
    if [ ! -f "$ENV_FILE" ]; then
        echo -e "${YELLOW}${WARNING} No .env file found. Creating from example...${NC}"
        if [ -f "$PROJECT_DIR/.env.example" ]; then
            cp "$PROJECT_DIR/.env.example" "$ENV_FILE"
            echo -e "${GREEN}${CHECK} Created .env file from example${NC}"
        else
            echo -e "${RED}${CROSS} No .env.example file found. Please create .env manually.${NC}"
        fi
    fi

    # 🌍 Set development environment
    export RUST_LOG=debug
    export ENVIRONMENT=development

    echo -e "${LIGHT_BLUE}${INFO} Environment: Development${NC}"
    echo -e "${LIGHT_BLUE}${INFO} Log level: Debug${NC}"
    echo ""

    cargo run
}

# ⚡ Development mode with auto-reload
run_dev() {
    echo -e "${BOLD}${PURPLE}${LIGHTNING} Starting Feedbacker in development mode with auto-reload...${NC}"
    echo ""

    if ! command -v cargo-watch &> /dev/null; then
        echo -e "${YELLOW}${WARNING} cargo-watch not found. Installing...${NC}"
        cargo install cargo-watch
    fi

    cd "$PROJECT_DIR" || exit 1

    export RUST_LOG=debug
    export ENVIRONMENT=development

    echo -e "${LIGHT_GREEN}${CHECK} Auto-reload enabled - edit files and see changes instantly!${NC}"
    echo -e "${CYAN}${INFO} Press Ctrl+C to stop${NC}"
    echo ""

    cargo watch -x run
}

# 🧪 Test function with pretty output
run_tests() {
    echo -e "${BOLD}${ORANGE}${FIRE} Running all tests...${NC}"
    echo ""

    cd "$PROJECT_DIR" || exit 1

    # 🎯 Run tests with nice output
    RUST_LOG=warn cargo test --color=always -- --nocapture

    if [ $? -eq 0 ]; then
        echo ""
        echo -e "${GREEN}${CHECK} All tests passed! ${TADA}${NC}"
    else
        echo ""
        echo -e "${RED}${CROSS} Some tests failed! ${WARNING}${NC}"
        exit 1
    fi
}

# 👀 Test watch mode
run_test_watch() {
    echo -e "${BOLD}${CYAN}${WRENCH} Running tests in watch mode...${NC}"
    echo ""

    if ! command -v cargo-watch &> /dev/null; then
        echo -e "${YELLOW}${WARNING} cargo-watch not found. Installing...${NC}"
        cargo install cargo-watch
    fi

    cd "$PROJECT_DIR" || exit 1

    echo -e "${LIGHT_GREEN}${CHECK} Test watch mode enabled - tests will run automatically when you save files!${NC}"
    echo -e "${CYAN}${INFO} Press Ctrl+C to stop${NC}"
    echo ""

    cargo watch -x test
}

# 📎 Clippy function (Rust's best friend!)
run_clippy() {
    echo -e "${BOLD}${PINK}${SPARKLES} Running Clippy - Rust's helpful assistant!${NC}"
    echo ""

    cd "$PROJECT_DIR" || exit 1

    echo -e "${LIGHT_BLUE}${INFO} Clippy will help make your code even more amazing!${NC}"
    echo ""

    cargo clippy --all-targets --all-features -- -D warnings

    if [ $? -eq 0 ]; then
        echo ""
        echo -e "${GREEN}${CHECK} Clippy is happy! Your code looks great! ${SPARKLES}${NC}"
        echo -e "${CYAN}${INFO} Remember: Clippy is like having a Rust expert looking over your shoulder${NC}"
    else
        echo ""
        echo -e "${YELLOW}${WARNING} Clippy found some suggestions for improvement${NC}"
        echo -e "${LIGHT_BLUE}${INFO} Don't worry - these will make your code even better!${NC}"
    fi
}

# 🎨 Format function
format_code() {
    echo -e "${BOLD}${LIGHT_BLUE}${STAR} Formatting code to be beautiful...${NC}"
    echo ""

    cd "$PROJECT_DIR" || exit 1

    cargo fmt

    if [ $? -eq 0 ]; then
        echo -e "${GREEN}${CHECK} Code formatted beautifully! ${RAINBOW}${NC}"
        echo -e "${PINK}${INFO} Your code now looks as beautiful as Trisha's spreadsheets!${NC}"
    else
        echo -e "${RED}${CROSS} Formatting failed${NC}"
        exit 1
    fi
}

# 🧹 Clean function
clean_project() {
    echo -e "${BOLD}${RED}${CROSS} Cleaning build artifacts...${NC}"
    echo ""

    cd "$PROJECT_DIR" || exit 1

    # 🗑️ Clean cargo artifacts
    cargo clean

    # 🗑️ Clean logs if they exist
    if [ -d "$LOG_DIR" ]; then
        echo -e "${YELLOW}${INFO} Cleaning logs...${NC}"
        rm -rf "$LOG_DIR"/*
    fi

    echo -e "${GREEN}${CHECK} Project cleaned! Fresh as morning coffee! ${COFFEE}${NC}"
}

# ✅ Quick check function
quick_check() {
    echo -e "${BOLD}${GREEN}${CHECK} Running quick compile check...${NC}"
    echo ""

    cd "$PROJECT_DIR" || exit 1

    cargo check --all-targets

    if [ $? -eq 0 ]; then
        echo ""
        echo -e "${GREEN}${CHECK} Quick check passed! ${LIGHTNING}${NC}"
    else
        echo ""
        echo -e "${RED}${CROSS} Compile check failed${NC}"
        exit 1
    fi
}

# 🏥 Doctor function - comprehensive health check
run_doctor() {
    echo -e "${BOLD}${ORANGE}${WARNING} Running development environment health check...${NC}"
    echo ""

    local issues=0

    # 🦀 Check Rust installation
    if command -v rustc &> /dev/null; then
        echo -e "${GREEN}${CHECK} Rust compiler: Available${NC}"
        RUST_VERSION=$(rustc --version)
        echo -e "    ${CYAN}${INFO} Version: ${RUST_VERSION}${NC}"
    else
        echo -e "${RED}${CROSS} Rust compiler: Not found${NC}"
        echo -e "    ${YELLOW}${INFO} Install from: https://rustup.rs/${NC}"
        ((issues++))
    fi

    # 📦 Check Cargo
    if command -v cargo &> /dev/null; then
        echo -e "${GREEN}${CHECK} Cargo: Available${NC}"
    else
        echo -e "${RED}${CROSS} Cargo: Not found${NC}"
        ((issues++))
    fi

    # 🔧 Check useful tools
    if command -v cargo-watch &> /dev/null; then
        echo -e "${GREEN}${CHECK} cargo-watch: Available${NC}"
    else
        echo -e "${YELLOW}${WARNING} cargo-watch: Not found (install with: cargo install cargo-watch)${NC}"
    fi

    # 🗄️ Check PostgreSQL
    if command -v psql &> /dev/null; then
        echo -e "${GREEN}${CHECK} PostgreSQL client: Available${NC}"
    else
        echo -e "${YELLOW}${WARNING} PostgreSQL client: Not found (needed for database operations)${NC}"
    fi

    # 🐙 Check Git
    if command -v git &> /dev/null; then
        echo -e "${GREEN}${CHECK} Git: Available${NC}"
    else
        echo -e "${RED}${CROSS} Git: Not found${NC}"
        ((issues++))
    fi

    # 📄 Check project files
    if [ -f "$CARGO_TOML" ]; then
        echo -e "${GREEN}${CHECK} Cargo.toml: Found${NC}"
    else
        echo -e "${RED}${CROSS} Cargo.toml: Missing${NC}"
        ((issues++))
    fi

    if [ -f "$ENV_FILE" ]; then
        echo -e "${GREEN}${CHECK} .env file: Found${NC}"
    else
        echo -e "${YELLOW}${WARNING} .env file: Missing (you may need to create one)${NC}"
    fi

    # 📊 Summary
    echo ""
    if [ $issues -eq 0 ]; then
        echo -e "${GREEN}${BOLD}${TADA} Perfect health! Your development environment is ready to rock! ${ROCKET}${NC}"
        echo -e "${CYAN}${INFO} You're all set to build amazing things with Feedbacker!${NC}"
    else
        echo -e "${YELLOW}${WARNING} Found ${issues} issue(s) that need attention${NC}"
        echo -e "${LIGHT_BLUE}${INFO} Fix these issues for the best development experience${NC}"
    fi
}

# 🐳 Docker functions
docker_build() {
    echo -e "${BOLD}${PURPLE}${COMPUTER} Building Docker image...${NC}"
    echo ""

    cd "$PROJECT_DIR" || exit 1

    if [ ! -f "Dockerfile" ]; then
        echo -e "${RED}${CROSS} Dockerfile not found${NC}"
        echo -e "${YELLOW}${INFO} Create a Dockerfile for containerization${NC}"
        exit 1
    fi

    docker build -t feedbacker:latest .

    if [ $? -eq 0 ]; then
        echo ""
        echo -e "${GREEN}${CHECK} Docker image built successfully! ${SHIP}${NC}"
    else
        echo ""
        echo -e "${RED}${CROSS} Docker build failed${NC}"
        exit 1
    fi
}

docker_run() {
    echo -e "${BOLD}${CYAN}${SHIP} Running Docker container...${NC}"
    echo ""

    docker run -p 3000:3000 --env-file .env feedbacker:latest
}

# 🚀 Setup function for initial project setup
setup_project() {
    echo -e "${BOLD}${YELLOW}${COFFEE} Setting up Feedbacker development environment...${NC}"
    echo ""

    cd "$PROJECT_DIR" || exit 1

    # 📁 Create necessary directories
    echo -e "${CYAN}${INFO} Creating directories...${NC}"
    mkdir -p logs
    mkdir -p temp

    # 📄 Create .env file if it doesn't exist
    if [ ! -f "$ENV_FILE" ] && [ -f "$PROJECT_DIR/.env.example" ]; then
        echo -e "${YELLOW}${INFO} Creating .env file from example...${NC}"
        cp "$PROJECT_DIR/.env.example" "$ENV_FILE"
        echo -e "${GREEN}${CHECK} .env file created${NC}"
    fi

    # 🦀 Install useful Rust tools
    echo -e "${CYAN}${INFO} Installing helpful Rust tools...${NC}"
    cargo install cargo-watch || true

    # 📦 Fetch dependencies
    echo -e "${CYAN}${INFO} Fetching dependencies...${NC}"
    cargo fetch

    echo ""
    echo -e "${GREEN}${BOLD}${TADA} Setup complete! You're ready to build amazing things! ${ROCKET}${NC}"
    echo -e "${PINK}${INFO} Try running: ${BOLD}./manage.sh run${NC}${PINK} to start the server${NC}"
}

# 🚀 Deploy function (placeholder for future)
deploy_project() {
    echo -e "${BOLD}${LIGHT_GREEN}${TADA} Deploy function - Coming soon!${NC}"
    echo ""
    echo -e "${CYAN}${INFO} Future deployment options:${NC}"
    echo -e "  ${YELLOW}• Docker containers${NC}"
    echo -e "  ${YELLOW}• Kubernetes clusters${NC}"
    echo -e "  ${YELLOW}• Cloud platforms (AWS, GCP, Azure)${NC}"
    echo -e "  ${YELLOW}• Traditional VPS deployment${NC}"
    echo ""
    echo -e "${PINK}${SPARKLES} For now, use Docker or build & deploy manually!${NC}"
}

# 🎯 Main command dispatcher
main() {
    # 🎨 Always show our beautiful banner!
    show_banner

    case "${1:-help}" in
        "build")
            build_project debug
            ;;
        "build-release")
            build_project release
            ;;
        "run")
            run_project
            ;;
        "dev")
            run_dev
            ;;
        "test")
            run_tests
            ;;
        "test-watch")
            run_test_watch
            ;;
        "clippy")
            run_clippy
            ;;
        "fmt"|"format")
            format_code
            ;;
        "clean")
            clean_project
            ;;
        "check")
            quick_check
            ;;
        "setup")
            setup_project
            ;;
        "doctor")
            run_doctor
            ;;
        "docker-build")
            docker_build
            ;;
        "docker-run")
            docker_run
            ;;
        "deploy")
            deploy_project
            ;;
        "status")
            show_status
            ;;
        "help"|"--help"|"-h")
            show_help
            ;;
        *)
            echo -e "${RED}${CROSS} Unknown command: ${1}${NC}"
            echo ""
            show_help
            exit 1
            ;;
    esac
}

# 🎉 Script entry point
# Trisha from Accounting says this is the most organized script execution she's ever seen!
main "$@"

# 🚢 End of script - May your code compile fast and your tests always pass! ⚓