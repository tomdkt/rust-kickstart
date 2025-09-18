.PHONY: infra/raise infra/down infra/logs db db/migrate db/prepare test test/verbose help

# Start infrastructure with docker-compose
infra/raise:
	docker compose --env-file ./.env up -d

# Stop infrastructure
infra/down:
	docker compose down

# Run database migrations
db/migrate:
	@which sqlx > /dev/null || (echo "❌ SQLx CLI not found. Install it with: cargo install sqlx-cli --no-default-features --features postgres" && exit 1)
	@echo "Running migrations..."
	@sqlx migrate run
	@echo "Migrations completed successfully!"

# Prepare SQLx query cache (run after migrations or SQL changes)
db/prepare:
	@which sqlx > /dev/null || (echo "❌ SQLx CLI not found. Install it with: cargo install sqlx-cli --no-default-features --features postgres" && exit 1)
	@echo "Preparing SQLx query cache..."
	@cargo sqlx prepare
	@echo "SQLx query cache updated successfully!"

# Complete database setup - idempotent command that does everything
db:
	@echo "🚀 Setting up database (idempotent)..."
	@echo "📦 Starting database container..."
	@docker compose --env-file ./.env up -d
	@echo "⏳ Waiting for database to be ready..."
	@sleep 3
	@which sqlx > /dev/null || (echo "❌ SQLx CLI not found. Install it with: cargo install sqlx-cli --no-default-features --features postgres" && exit 1)
	@echo "🔄 Running migrations..."
	@sqlx migrate run
	@echo "📝 Preparing SQLx query cache..."
	@cargo sqlx prepare
	@echo "✅ Database setup completed successfully!"

# Run migrations and prepare SQLx cache (legacy alias)
db/setup: db

# Run integration tests (requires database to be running)
test:
	@echo "🧪 Running integration tests..."
	@$(MAKE) db
	@echo "🔬 Running tests..."
	@cargo test

# Run tests with detailed output
test/verbose:
	@echo "🧪 Running integration tests with verbose output..."
	@$(MAKE) db
	@echo "🔬 Running tests with detailed output..."
	@cargo test -- --nocapture

# Show available commands
help:
	@echo "Available commands:"
	@echo "  db             - Complete database setup (idempotent) 🚀"
	@echo "  infra/raise    - Start containers in background"
	@echo "  infra/down     - Stop and remove containers"
	@echo "  db/migrate     - Run pending migrations"
	@echo "  db/prepare     - Update SQLx query cache"
	@echo "  test           - Run integration tests"
	@echo "  test/verbose   - Run tests with detailed output"
	@echo "  help           - Show this message"