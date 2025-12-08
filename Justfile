# CRM.hey.sh Monorepo Justfile
# Orchestrates builds across all components

set dotenv-load := true

# List available recipes
default:
    @just --list

# === ALL ===

# Build everything
build: build-backend build-frontend build-mobile
    @echo "All components built successfully"

# Test everything
test: test-backend test-frontend test-mobile
    @echo "All tests passed"

# Check/lint everything
check: check-backend check-frontend check-mobile
    @echo "All checks passed"

# Clean everything
clean: clean-backend clean-frontend clean-mobile
    @echo "All components cleaned"

# === BACKEND ===

# Build backend
build-backend:
    just backend/build

# Test backend
test-backend:
    just backend/test

# Check backend
check-backend:
    just backend/check

# Clean backend
clean-backend:
    just backend/clean

# Run backend locally
run-backend:
    just backend/run

# === FRONTEND ===

# Build frontend
build-frontend:
    just frontend/build

# Test frontend
test-frontend:
    just frontend/test

# Check frontend
check-frontend:
    just frontend/check

# Clean frontend
clean-frontend:
    just frontend/clean

# Run frontend dev server
dev-frontend:
    just frontend/dev

# === MOBILE ===

# Build mobile
build-mobile:
    just mobile/build

# Test mobile
test-mobile:
    just mobile/test

# Check mobile
check-mobile:
    just mobile/check

# Clean mobile
clean-mobile:
    just mobile/clean

# === INFRASTRUCTURE ===

# Deploy to development
deploy-dev:
    just infra/deploy-dev

# Deploy to production
deploy-prod:
    just infra/deploy-prod

# Start local docker environment
docker-up:
    docker-compose up -d

# Stop local docker environment
docker-down:
    docker-compose down

# View docker logs
docker-logs:
    docker-compose logs -f

# === CI/CD ===

# Run full CI pipeline (what CI runs)
ci: check test build
    @echo "CI pipeline complete"

# Install all dependencies
install: install-backend install-frontend install-mobile
    @echo "All dependencies installed"

install-backend:
    just backend/install

install-frontend:
    just frontend/install

install-mobile:
    just mobile/install

# === UTILITIES ===

# Format all code
fmt: fmt-backend fmt-frontend
    @echo "All code formatted"

fmt-backend:
    just backend/fmt

fmt-frontend:
    just frontend/fmt

# Generate protobuf code
proto:
    just backend/proto
