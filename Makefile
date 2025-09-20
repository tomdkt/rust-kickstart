.PHONY: dev dev/optimized dev/shutdown infra/raise infra/down db db/clean test test/unit test/integration check observability observability/destroy help

# Start app
dev:
	@$(MAKE) infra/raise
	@RUST_LOG=info cargo watch -x 'run' --why --clear

dev/shutdown:
	@pkill -f rust-kickstart 2>/dev/null || echo "No rust-kickstart processes found"

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

# Start observability stack (SigNoz + OpenTelemetry)
observability:
	@echo "🚀 Starting Observability Stack..."
	@echo "📊 Starting all services (SigNoz, ClickHouse, OpenTelemetry Collector)..."
	@docker compose --env-file ./.env -f docker-compose.observability.yaml up -d
	@echo "⏳ Waiting for services to be ready..."
	@echo "✅ Observability stack is ready!"
	@echo ""
	@echo "🌐 Access points:"
	@echo "   - SigNoz UI: http://localhost:3301"
	@echo "   - ClickHouse: http://localhost:8123"
	@echo "   - OTLP gRPC: localhost:4315"
	@echo "   - OTLP HTTP: localhost:4316"
	@echo ""
	@echo "📝 To send data from your app:"
	@echo "   - Set OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4316"
	@echo "   - Or use the collector: http://otel-collector:4316 (in Docker)"

# Stop and clean observability stack
observability/destroy:
	@echo "🧹 Stopping observability stack..."
	@docker compose --env-file ./.env -f docker-compose.observability.yaml down --volumes
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
	@echo "  observability  - Start observability stack (SigNoz + OpenTelemetry) 🔍
	@echo "  observability/destroy - Stop and clean observability stack"
	@echo "  infra/raise    - Start containers in background"
	@echo "  infra/down     - Stop and remove containers"
	@echo "  help           - Show this message"