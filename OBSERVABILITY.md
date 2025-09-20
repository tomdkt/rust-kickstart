# Observability Setup Guide

This project includes a comprehensive observability setup with OpenTelemetry and SigNoz for distributed tracing, metrics, and logging.

## Quick Start

### 1. Start the Observability Stack

```bash
make observability
```

This command will:
- Start SigNoz (all-in-one observability platform)
- Start ClickHouse (high-performance database for telemetry data)
- Start OpenTelemetry Collector (optional, for advanced routing)
- Set up all necessary networking and volumes

### 2. Access SigNoz UI

Open your browser and navigate to: **http://localhost:3301**

No authentication required for local development.

### 3. Start Your Application

```bash
make run
```

Your application will automatically send traces, metrics, and logs to SigNoz.

## Architecture Overview

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Application   │───▶│ OpenTelemetry    │───▶│     SigNoz      │
│  (Rust + OTLP)  │    │   Collector      │    │  (Storage + UI) │
│                 │    │   (Optional)     │    │                 │
└─────────────────┘    └──────────────────┘    └─────────────────┘
                                                         │
                                                         ▼
                                               ┌─────────────────┐
                                               │   ClickHouse    │
                                               │   (Database)    │
                                               └─────────────────┘
```

### Components

- **Application**: Your Rust service sending telemetry via OTLP
- **OpenTelemetry Collector**: Optional component for advanced data processing
- **SigNoz**: All-in-one observability platform with web UI
- **ClickHouse**: High-performance database for telemetry data (traces, metrics, logs)

## Configuration Options

The observability stack provides multiple endpoints for different use cases:

#### Direct to SigNoz (Recommended for Development)

```bash
OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4316
```
- Sends data directly to SigNoz
- Simpler setup, fewer moving parts
- Good for development and simple deployments

#### Through OpenTelemetry Collector (Recommended for Production)

```bash
OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4315
```
- Allows advanced data processing, filtering, and routing
- Better for production environments
- Supports multiple backends simultaneously

## Environment Variables

### Required Variables

```bash
# OpenTelemetry endpoint
OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4316

# Service identification
OTEL_SERVICE_NAME=rust-kickstart
OTEL_SERVICE_VERSION=0.1.0
OTEL_RESOURCE_ATTRIBUTES=service.name=rust-kickstart,service.version=0.1.0,deployment.environment=development
```

### Optional Variables

```bash
# Protocol (default: http/protobuf)
OTEL_EXPORTER_OTLP_PROTOCOL=http/protobuf

# Timeout in milliseconds (default: 10000)
OTEL_EXPORTER_OTLP_TIMEOUT=10000

# Enable specific exporters
OTEL_TRACES_EXPORTER=otlp
OTEL_METRICS_EXPORTER=otlp
OTEL_LOGS_EXPORTER=otlp
```

## Components Details

### ClickHouse (Telemetry Storage)
- **Purpose**: High-performance storage for traces, metrics, and logs
- **Ports**: 8123 (HTTP), 9000 (Native)
- **Database**: `signoz_traces`
- **Data**: All telemetry data (high volume, optimized for analytics)

## SigNoz Features

### Traces View
- **Distributed Tracing**: See complete request flows across services
- **Service Map**: Visual representation of service dependencies
- **Trace Search**: Find traces by service, operation, tags, or duration
- **Span Details**: Detailed information about each operation

### Metrics Dashboard
- **Application Metrics**: Request rate, error rate, duration (RED metrics)
- **Infrastructure Metrics**: CPU, memory, disk usage
- **Custom Metrics**: Business-specific metrics from your application
- **Alerts**: Set up alerts based on metric thresholds

### Logs Management
- **Structured Logs**: JSON logs with proper parsing
- **Log Search**: Full-text search across all logs
- **Log Correlation**: Link logs to traces for better debugging
- **Log Aggregation**: Group similar log entries

## Development Workflow

### 1. Start Observability Stack

```bash
make observability
```

### 2. Run Your Application

```bash
make run
# or
cargo run
```

### 3. Generate Some Traffic

```bash
# Health check
curl http://localhost:8080/health

# Create a user
curl -X POST http://localhost:8080/users \
  -H "Content-Type: application/json" \
  -d '{"name": "John Doe", "email": "john@example.com"}'

# Get users
curl http://localhost:8080/users
```

### 4. View Traces in SigNoz

1. Open http://localhost:3301
2. Navigate to "Traces" section
3. You should see traces from your API calls

## Production Considerations

### Security Considerations
- **Authentication** for SigNoz UI in production
- **TLS encryption** for OTLP endpoints
- **Network policies** to restrict access
- **Data retention** policies for telemetry data

### Performance Tuning
- **Sampling**: Configure trace sampling to reduce overhead
- **Batch Processing**: Use batch processors in OpenTelemetry Collector
- **Resource Limits**: Set appropriate memory and CPU limits
- **Storage**: Monitor ClickHouse disk usage and set retention policies

### Scaling
- **Horizontal Scaling**: Run multiple collector instances
- **Load Balancing**: Distribute telemetry data across collectors
- **Sharding**: Use ClickHouse sharding for large deployments

## Troubleshooting

### Common Issues

#### No traces appearing in SigNoz

1. Check if observability stack is running: `docker ps`
2. Verify OTLP endpoint: `curl http://localhost:4316/v1/traces`
3. Check SigNoz logs: `docker logs signoz-query-service`
4. Check ClickHouse connectivity: `docker logs signoz-clickhouse`
5. Verify environment variables are set correctly

#### High memory usage

1. Check ClickHouse memory usage: `docker stats signoz-clickhouse`
2. Configure data retention policies
3. Adjust batch processor settings in collector
4. Enable trace sampling

### Debugging Commands

```bash
# Check all services status
docker compose -f docker-compose.observability.yaml ps

# View SigNoz query service logs
docker logs signoz-query-service

# View ClickHouse logs (telemetry data)
docker logs signoz-clickhouse

# View OpenTelemetry Collector logs
docker logs otel-collector

# Test OTLP endpoint
curl -X POST http://localhost:4316/v1/traces \
  -H "Content-Type: application/x-protobuf" \
  --data-binary @/dev/null

# Check SigNoz API health
curl http://localhost:8080/api/v1/health
```

### Data Verification

```bash
# Check ClickHouse tables
docker exec -it signoz-clickhouse clickhouse-client --query "SHOW TABLES"

# Check trace data
docker exec -it signoz-clickhouse clickhouse-client --query "SELECT count() FROM signoz_traces.signoz_spans"
```

## Integration with CI/CD

### Docker Compose Integration

The observability stack is designed to work seamlessly with your application:

```yaml
# In your docker-compose.yml
services:
  your-app:
    environment:
      - OTEL_EXPORTER_OTLP_ENDPOINT=http://otel-collector:4316
    networks:
      - app-network
      - signoz-network
```

**Network Configuration:**
- Observability stack runs on `signoz-network`
- Main application connects to both `app-network` and `signoz-network`

### Environment-Specific Configuration

```bash
# Development
OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4316

# Docker Compose
OTEL_EXPORTER_OTLP_ENDPOINT=http://otel-collector:4316

# Production (with load balancer)
OTEL_EXPORTER_OTLP_ENDPOINT=https://otel-collector.your-domain.com
```

## Migration from Uptrace

If you're migrating from Uptrace, use the provided migration script:

```bash
./migrate-to-signoz.sh
```

This script will:
1. Stop existing Uptrace services
2. Start SigNoz services
3. Update network configurations
4. Provide new access URLs

## Additional Resources

- [SigNoz Documentation](https://signoz.io/docs/)
- [OpenTelemetry Rust SDK](https://docs.rs/opentelemetry/)
- [ClickHouse Documentation](https://clickhouse.com/docs/)