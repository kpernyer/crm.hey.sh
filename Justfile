# CRM.hey.sh - Root Justfile
# Orchestrates all subdirectories

set dotenv-load := true

# Show available commands
default:
    @just --list

# === CORE COMMANDS ===

# Install all dependencies
install:
    just backend/install
    just frontend/install
    just mobile/install

# Build everything
build:
    just backend/build
    just frontend/build
    just mobile/build

# Run all tests
test:
    just backend/test
    just frontend/test
    just mobile/test

# Run linting and type checks
check:
    just backend/check
    just frontend/check
    just mobile/check

# Run full CI pipeline
ci: install check test build
    @echo "CI complete"

# === RUN ===

# Run backend server
run:
    just backend/run

# Run frontend dev server
run-frontend:
    just frontend/run

# Run mobile (iOS simulator)
run-mobile:
    just mobile/run

# Run all services locally
run-all:
    just docker-up
    just backend/run &
    just frontend/run

# === DOCKER ===

# Start docker services (SurrealDB)
docker-up:
    docker-compose up -d

# Stop docker services
docker-down:
    docker-compose down

# View docker logs
docker-logs:
    docker-compose logs -f

# === CLEAN ===

# Clean all build artifacts
clean:
    just backend/clean
    just frontend/clean
    just mobile/clean

# === LLM INTEGRATION ===

# Run MCP server (for Claude Desktop/Code)
run-mcp:
    cd backend/mcp-server && cargo run

# Build MCP server
build-mcp:
    cd backend/mcp-server && cargo build --release

# Install Python LLM tools dependencies
install-llm-tools:
    pip install -r backend/src/llm_tools/requirements.txt

# Run LangChain agent example
run-langchain-agent:
    python backend/src/llm_tools/examples/langchain_agent.py

# Run Claude tool use example
run-claude-tools:
    python backend/src/llm_tools/examples/claude_tools.py

# Run OpenAI function calling example
run-openai-functions:
    python backend/src/llm_tools/examples/openai_functions.py

# === DEPLOY ===

# Deploy to dev environment
deploy-dev:
    just infra/deploy dev

# Deploy to prod environment
deploy-prod:
    just infra/deploy prod
