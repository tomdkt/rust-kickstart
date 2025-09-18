# Development commands for Rust Kickstart API

.PHONY: help dev test lint format check clean build docker-up docker-down migrate audit

# Default target
help: ## Show this help message
	@echo "Available commands:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

# Development
dev: ## Start development server with hot reload
	cargo watch -x run

install: ## Install development dependencies
	cargo install cargo-watch cargo-audit cargo-llvm-cov sqlx-cli

# Testing
test: ## Run all tests
	cargo test

test-watch: ## Run tests in watch mode
	cargo watch -x test

coverage: ## Generate test coverage report
	cargo llvm-cov --all-features --workspace --html

# Code quality
format: ## Format code using rustfmt
	cargo fmt

format-check: ## Check if code is properly formatted
	cargo fmt -- --check

lint: ## Run clippy linter
	cargo clippy --all-targets --all-features -- -D warnings

lint-fix: ## Fix clippy warnings automatically
	cargo clippy --all-targets --all-features --fix -- -D warnings

check: ## Run all checks (format, lint, test)
	@echo "🔍 Checking formatting..."
	@make format-check
	@echo "🔍 Running linter..."
	@make lint
	@echo "🔍 Running tests..."
	@make test
	@echo "✅ All checks passed!"

# Security
audit: ## Run security audit
	cargo audit

# Build
build: ## Build the project
	cargo build

build-release: ## Build optimized release version
	cargo build --release

# Database
migrate: ## Run database migrations
	sqlx migrate run

migrate-revert: ## Revert last migration
	sqlx migrate revert

# Docker
docker-up: ## Start PostgreSQL with Docker Compose
	docker-compose up -d

docker-down: ## Stop Docker services
	docker-compose down

docker-logs: ## Show Docker logs
	docker-compose logs -f

# Cleanup
clean: ## Clean build artifacts
	cargo clean

# Documentation
docs: ## Generate and open documentation
	cargo doc --open --no-deps

docs-check: ## Check documentation
	cargo doc --no-deps --document-private-items

# Pre-commit hook simulation
pre-commit: ## Run pre-commit checks
	@echo "🚀 Running pre-commit checks..."
	@make format-check
	@make lint
	@make test
	@make audit
	@echo "✅ Pre-commit checks passed!"