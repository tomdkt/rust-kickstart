# Migration from Uptrace to SigNoz

This document outlines the migration from Uptrace to SigNoz for observability in this project.

## What Changed

### Services Replaced
- **Uptrace** → **SigNoz** (observability platform)
- **PostgreSQL** (Uptrace metadata) → **Removed** (SigNoz uses ClickHouse only)
- **ClickHouse** → **Kept** (still used by SigNoz for telemetry storage)

### New Services Added
- **SigNoz Query Service**: Backend API for data queries
- **SigNoz Frontend**: Web UI for visualization
- **SigNoz OTel Collector**: Specialized collector for SigNoz

### Port Changes
| Service | Old Port | New Port | Purpose |
|---------|----------|----------|---------|
| UI Access | 14319 | 3301 | Web interface |
| OTLP gRPC | 4317 | 4315 | OpenTelemetry gRPC |
| OTLP HTTP | 4318 | 4316 | OpenTelemetry HTTP |

### Configuration Changes

#### Environment Variables
```bash
# Old (Uptrace)
OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:14318

# New (SigNoz)
OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4316
```

#### Docker Networks
```yaml
# Old
networks:
  uptrace-network:
    name: uptrace-network

# New
networks:
  signoz-network:
    name: signoz-network
```

## Migration Steps

### Automatic Migration
Use the provided migration script:
```bash
./migrate-to-signoz.sh
```

### Manual Migration
1. **Stop Uptrace services:**
   ```bash
   docker compose -f docker-compose.observability.yaml down
   ```

2. **Update environment variables:**
   ```bash
   # Update .env file
   OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4316
   ```

3. **Start SigNoz services:**
   ```bash
   make observability
   ```

4. **Access new UI:**
   - Old: http://localhost:14319
   - New: http://localhost:3301

## Benefits of SigNoz

### Advantages over Uptrace
- **Open Source**: Fully open source with active community
- **Better Performance**: Optimized for high-volume telemetry data
- **Modern UI**: More intuitive and responsive interface
- **Active Development**: Regular updates and new features
- **Cost Effective**: No licensing costs for production use

### Features
- **Distributed Tracing**: Complete request flow visualization
- **Metrics Dashboard**: Application and infrastructure metrics
- **Logs Management**: Structured log search and correlation
- **Alerting**: Built-in alerting system
- **Service Map**: Visual service dependency mapping

## Verification

After migration, verify everything works:

1. **Check services are running:**
   ```bash
   docker ps | grep signoz
   ```

2. **Access SigNoz UI:**
   ```bash
   open http://localhost:3301
   ```

3. **Test telemetry data:**
   ```bash
   # Start your application
   make run
   
   # Generate some traffic
   curl http://localhost:8080/health
   curl http://localhost:8080/users
   ```

4. **Verify traces in SigNoz:**
   - Go to http://localhost:3301
   - Navigate to "Traces" section
   - You should see traces from your API calls

## Rollback Plan

If you need to rollback to Uptrace:

1. **Stop SigNoz:**
   ```bash
   docker-compose -f docker-compose.observability.yaml down
   ```

2. **Restore Uptrace configuration:**
   ```bash
   git checkout HEAD~1 -- docker-compose.observability.yaml otelcol-config.yaml
   ```

3. **Update environment variables:**
   ```bash
   OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:14318
   ```

4. **Start Uptrace:**
   ```bash
   make observability
   ```

## Data Migration

**Note**: Telemetry data (traces, metrics, logs) cannot be migrated between Uptrace and SigNoz as they use different storage schemas. You will start with a clean slate in SigNoz.

If you need to preserve historical data:
1. Export data from Uptrace (if supported)
2. Keep Uptrace running in parallel for historical analysis
3. Use SigNoz for new data going forward

## Support

For issues with the migration:
1. Check the troubleshooting section in OBSERVABILITY.md
2. Review SigNoz documentation: https://signoz.io/docs/
3. Check container logs: `docker logs <container-name>`