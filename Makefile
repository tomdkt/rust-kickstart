.PHONY: dev dev/optimized infra/raise infra/down db db/clean test test/unit test/integration check observability observability/destroy help

# Start app
dev:
	@$(MAKE) infra/raise
	cargo watch -x run

dev/optimized:
	@$(MAKE) infra/raise
	RUST_LOG=info cargo run --release

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

db/clean:
	@echo "🧹 Cleaning up database container and volumes..."
	@docker compose --env-file ./.env down --volumes
	@echo "✅ Database cleanup completed successfully!"

# Run unit tests (fast, no database required)
test/unit:
	@echo "🧪 Running unit tests..."
	@cargo test --lib -- --nocapture

# Run integration tests (requires database to be running)
test/integration:
	@echo "🧪 Running integration tests..."
	@echo "📦 Ensuring database is running..."
	@$(MAKE) infra/raise
	@echo "🔬 Running integration tests with logging..."
	@RUST_LOG=info cargo test --test integration_user --test integration_bank_user -- --nocapture

# Run all tests (unit + integration)
test:
	@echo "🧪 Running all tests..."
	@$(MAKE) test/unit
	@$(MAKE) test/integration


# Run all code quality checks (format, lint, test)
check:
	@echo "🔍 Running code quality checks..."
	@echo "📝 Formatting code..."
	@cargo fmt
	@echo "🔧 Running linter..."
	@cargo clippy --all-targets --all-features -- -D warnings
	@echo "🧪 Running tests..."
	@$(MAKE) test
	@echo "✅ All checks passed!"

# Start observability stack (Jaeger + OpenTelemetry)
observability:
	@echo "🔍 Starting observability stack..."
	@echo "📊 Starting Jaeger and OpenTelemetry services..."
	@docker compose -f docker-compose.observability.yaml up -d
	@echo "⏳ Waiting for services to be ready..."
	@echo "✅ Observability stack started successfully!"
	@echo "🌐 Jaeger UI: http://localhost:16686"
	@echo "📡 OTLP HTTP endpoint: http://localhost:4318"
	@echo "📡 OTLP gRPC endpoint: http://localhost:4317"

# Stop and clean observability stack
observability/destroy:
	@echo "🧹 Stopping observability stack..."
	@docker compose -f docker-compose.observability.yaml down --volumes
	@echo "✅ Observability stack stopped and cleaned!"

# Show available commands
help:
	@echo "Available commands:"
	@echo "  dev            - Start development server with hot reload"
	@echo "  dev/optimized  - Start server with release flag"
	@echo "  db             - Complete database setup (idempotent) 🚀"
	@echo "  db/clean       - Complete database cleanup"
	@echo "  test           - Run all tests (unit + integration)"
	@echo "  test/unit      - Run unit tests only (fast, no database)"
	@echo "  test/integration - Run integration tests (requires database)"
	@echo "  check          - Run all code quality checks (format, lint, test)"
	@echo "  observability  - Start observability stack (Jaeger + OpenTelemetry) 🔍"
	@echo "  observability/destroy - Stop and clean observability stack"
	@echo "  infra/raise    - Start containers in background"
	@echo "  infra/down     - Stop and remove containers"
	@echo "  help           - Show this message"