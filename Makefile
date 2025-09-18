.PHONY: infra/raise infra/down infra/logs db db/migrate db/prepare test test/verbose help

# Start app
dev:
	@$(MAKE) infra/raise
	cargo watch -x run

# Start infrastructure with docker-compose
infra/raise:
	docker compose --env-file ./.env up -d

# Stop infrastructure
infra/down:
	docker compose down

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


# Run integration tests (requires database to be running)
test:
	@echo "🧪 Running integration tests..."
	@$(MAKE) db
	@echo "🔬 Running tests..."
	@cargo test


# Show available commands
help:
	@echo "Available commands:"
	@echo "  db             - Complete database setup (idempotent) 🚀"
	@echo "  infra/raise    - Start containers in background"
	@echo "  infra/down     - Stop and remove containers"
	@echo "  test           - Run integration tests"
	@echo "  help           - Show this message"