# Rust Kickstart

Minimal REST API with Axum, PostgreSQL and modular architecture.

## Features

- **Modular Architecture**: Clean layers (Domain → Repository → Service → Controller)
- **Type Safety**: Compile-time architecture enforcement
- **PostgreSQL**: SQLx with migrations and query validation
- **OpenAPI**: Auto-generated documentation
- **Testing**: Isolated test schemas

📖 **Architecture details**: [ARCHITECTURE.md](ARCHITECTURE.md)

## Quick Start

```bash
    make db    # Setup database
    make dev   # Start server with hot reload
```

Server: [`http://127.0.0.1:3000`](http://127.0.0.1:3000) | Docs: [`/swagger-ui`](http://127.0.0.1:3000/swagger-ui)

## Commands

- `make dev` - Development server
- `make db` - Database setup (idempotent)
- `make test` - Run tests
- `make check` - Format, lint, test

## Database

SQLx with compile-time checking. Always commit `.sqlx/` directory.

```bash
    sqlx migrate add new_table  # Create migration
    make db                     # Apply migrations + update cache
```

## API Endpoints

- `POST /users` - Create user
- `GET /users` - List users
- `GET /users/{id}` - Get user
- `PUT /users/{id}` - Update user
- `DELETE /users/{id}` - Delete user

## Requirements

- Rust
- Docker
- SQLx CLI: `cargo install sqlx-cli --no-default-features --features postgres`
- cargo-watch: `cargo install cargo-watch` (for development server)

## License

MIT