# Rust Kickstart

A minimal Rust API boilerplate with Axum, PostgreSQL, and Docker.

## Getting Started

```bash
    # 1. Install development dependencies
    make install 
    
    # 2. Setup database (idempotent - does everything needed)
    make db

    # 2. Start server
    cargo run
```

Server runs at [`http://127.0.0.1:3000`](http://127.0.0.1:3000)

## Quick Commands

### Start Application
```bash
    # Setup database and start application
    make db && cargo run
```

### Stop Application
```bash
    # Stop the Rust application (Ctrl+C if running in foreground)
    # Or if running in background, find and kill the process:
    pkill -f "rust-kickstart" || pkill -f "target.*rust-kickstart"
    
    # Stop database
    docker-compose down
```

### Complete Restart
```bash
    # Stop everything and restart
    make infra/down && make db && cargo run
```

### Run Tests
```bash
    # Run integration tests (automatically starts database and runs migrations)
    make test
    
    # Run tests with detailed output
    make test/verbose
```

**Test Isolation**: Integration tests use isolated PostgreSQL schemas with UUID v7 identifiers. Each test creates its own schema (`test_{uuid_v7}`), runs migrations, executes tests, and automatically cleans up. This ensures complete test isolation and allows parallel execution without data conflicts.

## Database & SQLx

This project uses SQLx with compile-time query checking. The `.sqlx/` directory contains cached query metadata that allows compilation without a live database connection.

### Working with Migrations

```bash
    # Create a new migration
    sqlx migrate add create_new_table

    # Setup everything (idempotent - safe to run multiple times)
    make db
```

### Important SQLx Workflow

**Every time you:**
- Create a new migration
- Modify existing SQL queries in your code
- Add new SQL queries

**Just run:**
```bash
    make db
```

This command is **idempotent** and does everything needed:
- Starts database container (if not running)
- Runs pending migrations
- Updates SQLx query cache

**Always commit the `.sqlx/` directory** to version control so other developers and CI/CD can build without a database connection.

### Available Database Commands

- `make db` - **Complete database setup (idempotent) - USE THIS ONE! 🚀**
- `make db/migrate` - Run pending migrations only
- `make db/prepare` - Update SQLx query cache only

## API Documentation

- **Swagger UI**: [`http://127.0.0.1:3000/swagger-ui`](http://127.0.0.1:3000/swagger-ui)
- **OpenAPI JSON**: [`http://127.0.0.1:3000/api-docs/openapi.json`](http://127.0.0.1:3000/api-docs/openapi.json)

### Available Endpoints

- `POST /users` - Create new user
- `GET /users` - Get all users
- `GET /users/{id}` - Get user by ID
- `PUT /users/{id}` - Update user (partial updates supported)
- `DELETE /users/{id}` - Delete user

## Code Quality & Best Practices

This project follows strict code quality standards with automated linting, formatting, and testing.

### Linting & Formatting

**Automatic formatting and linting on save** (VS Code configured):
- **rustfmt**: Code formatting with custom rules in `rustfmt.toml`
- **Clippy**: Advanced linting with pedantic rules in `clippy.toml`
- **Workspace lints**: Comprehensive lint rules in `Cargo.toml`

### Quick Commands

```bash
# Format code
make format

# Check formatting (CI-style)
make format-check

# Run linter
make lint

# Fix linting issues automatically
make lint-fix

# Run all checks (format + lint + test)
make check

# Security audit
make audit

# Generate test coverage
make coverage
```

### Pre-commit Hooks

Install pre-commit hooks to run checks automatically:

```bash
# Install pre-commit (requires Python)
pip install pre-commit

# Install hooks
pre-commit install

# Run manually
make pre-commit
```

### GitHub Actions CI

Automated CI pipeline runs on every push/PR:
- ✅ **Formatting check**: `cargo fmt -- --check`
- ✅ **Linting**: `cargo clippy -- -D warnings`
- ✅ **Tests**: Full test suite with PostgreSQL
- ✅ **Security audit**: `cargo audit`
- ✅ **Documentation**: `cargo doc`
- ✅ **Coverage**: `cargo llvm-cov --html`

### Development Tools

Install recommended development tools:

```bash
make install
```

This installs:
- `cargo-watch` - Hot reload during development
- `cargo-audit` - Security vulnerability scanning
- `cargo-llvm-cov` - Code coverage reporting
- `sqlx-cli` - Database migration tools

### VS Code Integration

The project includes VS Code settings (`.vscode/settings.json`) that:
- Enable format on save
- Run Clippy on save
- Configure rust-analyzer with project-specific settings
- Auto-fix issues when possible

## Logging

The application uses structured logging with `tracing`. You can control log levels with the `RUST_LOG` environment variable:

```bash
    # Debug level for the app, HTTP requests, and Axum rejections
    export RUST_LOG=rust_kickstart=debug,tower_http=debug,axum::rejection=trace
    
    # Info level (default)
    export RUST_LOG=info
    
    # Only errors
    export RUST_LOG=error
```

## Requirements

- Rust
- Docker
- SQLx CLI: `cargo install sqlx-cli --no-default-features --features postgres`

## License

MIT