#!/bin/bash

echo "🚀 Starting Observability Stack..."

# Start the observability stack
echo "📊 Starting Uptrace, ClickHouse, and PostgreSQL..."
docker compose -f docker-compose.observability.yaml up -d

# Wait for services to be healthy
echo "⏳ Waiting for services to be ready..."
sleep 10

# Start the OpenTelemetry Collector
echo "🔄 Starting OpenTelemetry Collector..."
docker compose -f docker-compose.observability.yaml --profile collector up -d

echo "✅ Observability stack is ready!"
echo ""
echo "🌐 Access points:"
echo "   - Uptrace UI: http://localhost:14319"
echo "   - Login: uptrace@localhost / uptrace"
echo "   - ClickHouse: http://localhost:8123"
echo "   - OTLP gRPC: localhost:4317"
echo "   - OTLP HTTP: localhost:4318"
echo ""
echo "📝 To send data from your app:"
echo "   - Set OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4318"
echo "   - Or use the collector: http://otel-collector:4318 (in Docker)"