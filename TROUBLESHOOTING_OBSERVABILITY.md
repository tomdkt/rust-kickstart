# OpenTelemetry & Uptrace Troubleshooting Guide

## Quick Start

1. **Start the observability stack:**
   ```bash
   ./start-observability.sh
   ```

2. **Run your application locally:**
   ```bash
   # Make sure OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4318 in your .env
   cargo run
   ```

3. **Or run everything in Docker:**
   ```bash
   # Start observability stack first
   docker compose -f docker-compose.observability.yaml --profile collector up -d
   
   # Then start your app
   docker compose --profile app up -d
   ```

## Common Issues & Solutions

### 1. Data Not Appearing in Uptrace

**Symptoms:** OpenTelemetry Collector logs show data received, but nothing in Uptrace UI.

**Solutions:**
- ✅ **Check the correct project:** Your data goes to Project 2 ("My project"), not Project 1
- ✅ **Look in Traces, not Logs:** SpanEvents appear inside traces, not as standalone logs
- ✅ **Verify DSN:** Collector uses `project2_secret_token` and project ID `2`

### 2. Connection Refused Errors

**Symptoms:** `connection refused` when collector tries to reach Uptrace.

**Solutions:**
- ✅ **Network connectivity:** Ensure all services are on `uptrace-network`
- ✅ **Service health:** Wait for Uptrace to be healthy before starting collector
- ✅ **Correct endpoint:** Use `uptrace:14317` (gRPC) not `http://uptrace:14317`

### 3. Application Can't Reach Collector

**Symptoms:** App fails to send telemetry data.

**Solutions:**
- ✅ **Local development:** Use `http://localhost:4318`
- ✅ **Docker environment:** Use `http://otel-collector:4318`
- ✅ **Network setup:** App container must be on `uptrace-network`

### 4. Missing Traces in UI

**Check these locations in Uptrace:**
1. **Traces tab** → Look for service name `rust-kickstart`
2. **Inside trace details** → SpanEvents appear as logs within traces
3. **Time range** → Adjust time picker to include recent data
4. **Filters** → Clear any service/operation filters

## Configuration Summary

### For Local Development
```bash
# .env file
OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4318
OTEL_SERVICE_NAME=rust-kickstart
```

### For Docker Environment
```bash
# Environment variables in docker-compose.yml
OTEL_EXPORTER_OTLP_ENDPOINT=http://otel-collector:4318
OTEL_SERVICE_NAME=rust-kickstart
```

## Verification Steps

1. **Check collector is receiving data:**
   ```bash
   docker logs otel-collector
   # Should show: "Trace received" messages
   ```

2. **Check Uptrace is healthy:**
   ```bash
   curl http://localhost:14319/
   # Should return Uptrace UI
   ```

3. **Test OTLP endpoint:**
   ```bash
   curl -v http://localhost:4318/v1/traces
   # Should return 405 Method Not Allowed (means endpoint is working)
   ```

4. **Check network connectivity:**
   ```bash
   docker exec otel-collector ping uptrace
   # Should successfully ping
   ```

## Access Points

- **Uptrace UI:** http://localhost:14319
- **Login:** uptrace@localhost / uptrace
- **OTLP HTTP:** http://localhost:4318
- **OTLP gRPC:** http://localhost:4317
- **ClickHouse:** http://localhost:8123

## Project Configuration

Your telemetry data goes to:
- **Project ID:** 2
- **Project Name:** "My project"
- **Token:** project2_secret_token

The collector configuration automatically routes data to this project.