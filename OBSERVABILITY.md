# Observability Setup Guide

This project includes a comprehensive observability setup with OpenTelemetry and Uptrace for distributed tracing, metrics, and logging.

## Quick Start

### 1. Start Observability Stack

```bash
make observability
```

This command will:
- Start Uptrace (all-in-one observability platform)
- Start OpenTelemetry Collector (optional, for advanced routing)
- Set up all necessary networking and volumes

### 2. Access Uptrace UI

Open your browser and navigate to: **http://localhost:14319**

Default credentials:
- Email: `uptrace@localhost`
- Password: `uptrace`

### 3. Start Your Application

```bash
make dev
```

Your application will automatically send traces, metrics, and logs to Uptrace.

## Architecture Overview

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Application   │───▶│ OpenTelemetry    │───▶│     Uptrace     │
│  (Rust + OTLP)  │    │   Collector      │    │  (Storage + UI) │
└─────────────────┘    └──────────────────┘    └─────────────────┘
                              │                         │
                              ▼                         ▼
                       ┌─────────────────┐    ┌─────────────────┐
                       │   ClickHouse    │    │   ClickHouse    │
                       │   (Database)    │    │   (Database)    │
                       └─────────────────┘    └─────────────────┘
```

## Configuration

### Environment Variables

The application uses these environment variables for observability:

```bash
# Required - OpenTelemetry endpoint
OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:14318

# Service identification
OTEL_SERVICE_NAME=rust-kickstart
OTEL_SERVICE_VERSION=0.1.0
OTEL_RESOURCE_ATTRIBUTES=service.name=rust-kickstart,service.version=0.1.0,deployment.environment=development

# Optional - Protocol and timeout settings
OTEL_EXPORTER_OTLP_PROTOCOL=http/protobuf
OTEL_EXPORTER_OTLP_TIMEOUT=10000
OTEL_TRACES_EXPORTER=otlp
OTEL_METRICS_EXPORTER=otlp
OTEL_LOGS_EXPORTER=otlp
```

### Backend-Agnostic Design

The observability setup is designed to be backend-agnostic. You can easily switch between different observability backends:

#### Uptrace (Default)
```bash
OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:14318
```

#### Jaeger
```bash
OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4318
```

#### Grafana Cloud
```bash
OTEL_EXPORTER_OTLP_ENDPOINT=https://otlp-gateway-prod-us-central-0.grafana.net/otlp
# Add authentication headers as needed
```



#### Custom OpenTelemetry Collector
```bash
OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4318
```

## Features

### Distributed Tracing
- **Automatic instrumentation** for HTTP requests
- **Custom spans** for business logic
- **Trace correlation** across service boundaries
- **Performance monitoring** with latency metrics

### Metrics Collection
- **HTTP request metrics** (duration, status codes, throughput)
- **Database query metrics** (connection pool, query duration)
- **Custom business metrics**
- **System metrics** (CPU, memory, disk)

### Structured Logging
- **Correlated logs** with trace and span IDs
- **JSON format** in production for log aggregation
- **Pretty format** in development for readability
- **Contextual information** with structured fields

### Graceful Shutdown
- **Automatic flush** of telemetry data on shutdown
- **Proper resource cleanup**
- **Signal handling** for CTRL+C

## Usage Examples

### Adding Custom Spans

```rust
use tracing::{info, instrument, Span};

#[instrument(
    name = "user_creation",
    fields(user_id, email = %request.email)
)]
async fn create_user(request: CreateUserRequest) -> Result<User, UserError> {
    // Get current span to add dynamic attributes
    let span = Span::current();
    
    info!("Starting user creation process");
    
    // Validate user data
    validate_user_data(&request).await?;
    
    // Save to database
    let user = save_user_to_db(&request).await?;
    
    // Add user ID to span after creation
    span.record("user_id", &user.id.to_string());
    
    info!(user_id = %user.id, "User created successfully");
    Ok(user)
}
```

### Custom Metrics

```rust
use opentelemetry::{global, KeyValue};

// Create a counter metric
let meter = global::meter("rust-kickstart");
let user_creation_counter = meter
    .u64_counter("user_creations_total")
    .with_description("Total number of user creations")
    .init();

// Increment the counter
user_creation_counter.add(1, &[
    KeyValue::new("status", "success"),
    KeyValue::new("method", "api"),
]);
```

### Structured Logging with Context

```rust
use tracing::{info, error, warn};

// Log with structured data
info!(
    user_id = %user.id,
    email = %user.email,
    action = "login",
    ip_address = %client_ip,
    "User login successful"
);

// Error logging with context
error!(
    error = %e,
    user_id = %user.id,
    operation = "database_save",
    "Failed to save user to database"
);
```

## Uptrace Features

### Traces View
- **Trace timeline** with span hierarchy and waterfall view
- **Service map** showing dependencies and performance metrics
- **Performance analysis** with bottleneck identification
- **Error tracking** with detailed span information and stack traces

### Metrics Dashboard
- **Real-time metrics** with customizable dashboards
- **Service performance** metrics (latency, throughput, error rate)
- **Infrastructure metrics** (CPU, memory, disk usage)
- **Custom business metrics** with alerting capabilities

### Search and Filter
- **Advanced search** with SQL-like queries
- **Service-based filtering** to focus on specific services
- **Attribute-based queries** for complex filtering
- **Time range selection** with zoom and pan capabilities

### Alerting and Monitoring
- **Smart alerts** based on anomaly detection
- **Custom alert rules** for business metrics
- **Integration** with popular notification channels
- **SLA monitoring** with uptime tracking

## Advanced Configuration

### OpenTelemetry Collector

For advanced use cases, you can use the OpenTelemetry Collector for:
- **Data processing** (filtering, sampling, enrichment)
- **Multiple backends** (send data to multiple destinations)
- **Protocol translation** (convert between different formats)
- **Load balancing** across multiple backend instances

Start with collector:
```bash
docker compose -f docker-compose.observability.yaml --profile collector up -d
```

Then configure your application to send data to the collector:
```bash
OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4318
```

### Sampling Configuration

Control trace sampling to manage data volume:

```rust
// In src/config/tracing.rs
use opentelemetry_sdk::trace::Sampler;

// Sample 10% of traces
.with_sampler(Sampler::TraceIdRatioBased(0.1))

// Always sample errors and slow requests
.with_sampler(Sampler::ParentBased(Box::new(
    Sampler::TraceIdRatioBased(0.1)
)))
```

### Resource Attributes

Add custom resource attributes for better service identification:

```bash
OTEL_RESOURCE_ATTRIBUTES="service.name=rust-kickstart,service.version=0.1.0,deployment.environment=production,k8s.cluster.name=prod-cluster,k8s.namespace.name=default"
```

## Production Deployment

### Security Considerations
- **Authentication** for Uptrace UI in production (configure in uptrace.yml)
- **TLS encryption** for OTLP endpoints
- **Network policies** to restrict access
- **Data retention** policies for compliance (configurable in ClickHouse)

### Performance Optimization
- **Sampling strategies** to control data volume
- **Batch processing** for efficient data export
- **Resource limits** for collector containers
- **Monitoring** of the observability stack itself

### High Availability
- **Multiple collector instances** for redundancy
- **Load balancing** across collectors
- **ClickHouse clustering** for persistent storage
- **Backup strategies** for ClickHouse data

## Troubleshooting

### Common Issues

#### No traces appearing in Uptrace
1. Check if observability stack is running: `docker ps`
2. Verify OTLP endpoint: `curl http://localhost:14318/v1/traces`
3. Check Uptrace logs: `docker logs uptrace`
4. Check ClickHouse connectivity: `docker logs clickhouse`
5. Verify environment variables are set correctly

#### High memory usage
1. Adjust sampling rate to reduce data volume
2. Configure batch processing limits
3. Set memory limits for collector containers
4. Monitor resource usage with `docker stats`

#### Slow application performance
1. Reduce trace sampling rate
2. Optimize span creation (avoid too many spans)
3. Use asynchronous export mode
4. Monitor collector performance

### Debug Commands

```bash
# Check observability stack status
docker compose -f docker-compose.observability.yaml ps

# View Uptrace logs
docker logs uptrace

# View ClickHouse logs
docker logs clickhouse

# View collector logs (if using collector profile)
docker logs otel-collector

# Test OTLP endpoint
curl -X POST http://localhost:14318/v1/traces \
  -H "Content-Type: application/x-protobuf" \
  --data-binary @/dev/null

# Check Uptrace UI health
curl http://localhost:14319/api/v1/health

# Check application telemetry
RUST_LOG=opentelemetry=debug,tracing_opentelemetry=debug cargo run
```

## Cleanup

### Stop observability stack
```bash
make observability/destroy
```

This will:
- Stop all observability containers
- Remove volumes and networks
- Clean up all observability data

### Temporary disable
To temporarily disable OpenTelemetry without stopping the stack:
```bash
unset OTEL_EXPORTER_OTLP_ENDPOINT
```

### Access Uptrace Projects

Uptrace supports multiple projects for organizing different applications:

- **Project 1 (Uptrace)**: Used for monitoring Uptrace itself
  - Token: `project1_secret_token`
  - URL: `http://localhost:14319/1`

- **Project 2 (My project)**: For your application
  - Token: `project2_secret_token` 
  - URL: `http://localhost:14319/2`

You can configure additional projects in `uptrace.yml` as needed.

## Integration with CI/CD

### GitHub Actions Example

```yaml
name: Test with Observability
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Start observability stack
        run: make observability
      - name: Run tests with tracing
        run: |
          export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:14318
          make test
      - name: Cleanup
        run: make observability/destroy
```

### Docker Compose Integration

For development environments, you can combine the main application with observability:

```bash
# Start everything together
docker compose -f docker-compose.yml -f docker-compose.observability.yaml up -d
```

## Further Reading

- [OpenTelemetry Documentation](https://opentelemetry.io/docs/)
- [Uptrace Documentation](https://uptrace.dev/get/)
- [Tracing in Rust](https://tracing.rs/tracing/)
- [OpenTelemetry Rust SDK](https://github.com/open-telemetry/opentelemetry-rust)