# Enhanced Tracing Configuration Guide

This project now includes an improved tracing and logging configuration that provides better observability for development and production environments.

## Features

### Development Environment
- **Pretty formatted logs** with colors and detailed information
- **File and line numbers** for easy debugging
- **Thread IDs and names** for concurrent debugging
- **Verbose logging** for detailed troubleshooting

### Production Environment
- **JSON structured logs** for log aggregation systems
- **Optimized performance** with reduced verbosity
- **Structured fields** for better parsing and filtering
- **Request correlation** for distributed tracing

## Configuration

### Environment Variables

You can control logging levels using the `RUST_LOG` environment variable:

```bash
# Development (verbose)
export RUST_LOG="rust_kickstart=debug,tower_http=debug,axum::rejection=trace,sqlx=info"

# Production (optimized)
export RUST_LOG="rust_kickstart=info,tower_http=info,sqlx=warn"

# Custom levels
export RUST_LOG="rust_kickstart=trace,sqlx=debug"
```

### Log Levels

- `trace`: Most verbose, includes all details
- `debug`: Development debugging information
- `info`: General information about application flow
- `warn`: Warning conditions that should be noted
- `error`: Error conditions that need attention

## Usage Examples

### Basic Logging in Your Code

```rust
use tracing::{info, warn, error, debug, trace};

// Information logging
info!("User created successfully");

// Warning with context
warn!(user_id = %user.id, "User validation failed");

// Error with structured data
error!(
    error = %e,
    user_id = %user.id,
    "Failed to create user"
);

// Debug information
debug!(request_id = %req_id, "Processing request");
```

### Structured Logging

```rust
use tracing::{info, Span};

// Create spans for request tracking
let span = tracing::info_span!(
    "user_operation",
    user_id = %user.id,
    operation = "create"
);

let _enter = span.enter();
info!("Starting user creation");
// ... your code ...
info!("User creation completed");
```

### HTTP Request Tracing

The application automatically logs HTTP requests with:
- Request method and URI
- Response status codes
- Request duration
- User agent information
- Correlation IDs for tracking

Example output in development:
```
2024-01-15T10:30:45.123456Z  INFO http_request: Request started method=POST uri=/users
2024-01-15T10:30:45.125789Z  INFO http_request: Request completed successfully status=201 latency_ms=2
```

Example output in production (JSON):
```json
{
  "timestamp": "2024-01-15T10:30:45.123456Z",
  "level": "INFO",
  "fields": {
    "message": "Request completed successfully",
    "method": "POST",
    "uri": "/users",
    "status": 201,
    "latency_ms": 2
  },
  "span": {
    "name": "http_request"
  }
}
```

## Best Practices

### 1. Use Appropriate Log Levels
- Use `info!` for business logic events
- Use `warn!` for recoverable errors or unusual conditions
- Use `error!` for actual errors that need attention
- Use `debug!` for development debugging
- Use `trace!` for very detailed debugging

### 2. Include Context
Always include relevant context in your logs:

```rust
// Good
info!(user_id = %user.id, email = %user.email, "User login successful");

// Better
info!(
    user_id = %user.id,
    email = %user.email,
    ip_address = %client_ip,
    user_agent = %user_agent,
    "User login successful"
);
```

### 3. Use Spans for Request Tracking
Create spans for operations that span multiple function calls:

```rust
async fn create_user(data: CreateUser) -> Result<User, UserError> {
    let span = tracing::info_span!(
        "create_user",
        email = %data.email,
        name = %data.name
    );
    
    async move {
        info!("Validating user data");
        // validation logic...
        
        info!("Saving user to database");
        // database logic...
        
        info!("User created successfully");
        Ok(user)
    }.instrument(span).await
}
```

### 4. Error Logging
Always log errors with context:

```rust
match user_service.create_user(data).await {
    Ok(user) => {
        info!(user_id = %user.id, "User created successfully");
        Ok(user)
    }
    Err(e) => {
        error!(
            error = %e,
            email = %data.email,
            "Failed to create user"
        );
        Err(e)
    }
}
```

## Monitoring and Observability

### Log Aggregation
In production, JSON logs can be easily ingested by:
- **ELK Stack** (Elasticsearch, Logstash, Kibana)
- **Fluentd/Fluent Bit**
- **Grafana Loki**
- **AWS CloudWatch**
- **Google Cloud Logging**

### Metrics and Alerting
Set up alerts based on log patterns:
- High error rates (`level: "ERROR"`)
- Slow requests (`latency_ms > 1000`)
- Authentication failures
- Database connection issues

### Request Tracing
Use correlation IDs to trace requests across services:
- Each HTTP request gets a unique ID
- Pass this ID to downstream services
- Use it to correlate logs across the entire request flow

## Configuration Files

The tracing configuration is located in:
- `src/config/tracing.rs` - Main configuration
- `src/main.rs` - Initialization
- `src/lib.rs` - HTTP tracing layer integration

## Troubleshooting

### Common Issues

1. **Logs not appearing**: Check `RUST_LOG` environment variable
2. **Too verbose**: Lower the log level (info instead of debug)
3. **Missing context**: Add structured fields to your log statements
4. **Performance issues**: Use appropriate log levels in production

### Debug Mode
To enable maximum verbosity for debugging:

```bash
export RUST_LOG="trace"
cargo run
```

This will show all logs from all modules, which can be helpful for troubleshooting.