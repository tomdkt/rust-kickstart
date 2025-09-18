# Rust Kickstart

A minimal Rust API boilerplate with Axum, PostgreSQL, and Docker.

## Getting Started

```bash
    # 1. Setup database (idempotent - does everything needed)
    make db

    # 2. Start development server with hot reload
    make dev
```

Server runs at [`http://127.0.0.1:3000`](http://127.0.0.1:3000)

## Available Commands

```bash
    # Start development server with hot reload
    make dev

    # Setup database (idempotent - safe to run multiple times)
    make db

    # Run integration tests
    make test

    # Run all code quality checks (format, lint, test)
    make check

    # Start infrastructure containers
    make infra/raise

    # Stop infrastructure containers
    make infra/down

    # Show all available commands
    make help
```

**Test Isolation**: Integration tests use isolated PostgreSQL schemas with UUID v7 identifiers. Each test creates its
own schema (`test_{uuid_v7}`), runs migrations, executes tests, and automatically cleans up. This ensures complete test
isolation and allows parallel execution without data conflicts.

## Database & SQLx

This project uses SQLx with compile-time query checking. The `.sqlx/` directory contains cached query metadata that
allows compilation without a live database connection.

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

**Always commit the `.sqlx/` directory** to version control so other developers and CI/CD can build without a database
connection.

### Database Commands

- `make db` - **Complete database setup (idempotent) - USE THIS ONE! 🚀**

## API Documentation

- **Swagger UI**: [`http://127.0.0.1:3000/swagger-ui`](http://127.0.0.1:3000/swagger-ui)
- **OpenAPI JSON**: [`http://127.0.0.1:3000/api-docs/openapi.json`](http://127.0.0.1:3000/api-docs/openapi.json)

### Available Endpoints

- `POST /users` - Create new user
- `GET /users` - Get all users
- `GET /users/{id}` - Get user by ID
- `PUT /users/{id}` - Update user (partial updates supported)
- `DELETE /users/{id}` - Delete user

## Code Quality

Run all code quality checks with:

```bash
  make check
```

This command runs:

- **Code formatting** with `cargo fmt`
- **Linting** with `cargo clippy`
- **Tests** with full database setup

## Logging

The application uses structured logging with `tracing`. You can control log levels with the `RUST_LOG` environment
variable:

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
- cargo-watch: `cargo install cargo-watch` (for development server)

## License

MIT