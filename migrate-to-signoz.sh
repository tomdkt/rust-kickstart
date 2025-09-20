#!/bin/bash

# Migration script from Uptrace to SigNoz
echo "🔄 Migrating from Uptrace to SigNoz..."

# Stop existing Uptrace services
echo "⏹️  Stopping Uptrace services..."
docker compose -f docker-compose.observability.yaml down

# Remove Uptrace volumes (optional - uncomment if you want to clean up)
# echo "🗑️  Removing Uptrace volumes..."
# docker volume rm $(docker volume ls -q | grep uptrace) 2>/dev/null || true

# Start SigNoz services
echo "🚀 Starting SigNoz services..."
docker compose -f docker-compose.observability.yaml up -d

# Wait for services to be ready
echo "⏳ Waiting for services to be ready..."
echo "   This may take a few minutes for first startup..."

# Wait for ClickHouse
echo "   Waiting for ClickHouse..."
timeout 60 bash -c 'until docker exec signoz-clickhouse clickhouse-client --query "SELECT 1" 2>/dev/null; do sleep 2; done'

# Wait for Query Service
echo "   Waiting for Query Service..."
timeout 60 bash -c 'until curl -s http://localhost:8080/api/v1/health >/dev/null 2>&1; do sleep 2; done'

# Wait for Frontend
echo "   Waiting for Frontend..."
timeout 30 bash -c 'until curl -s http://localhost:3301 >/dev/null 2>&1; do sleep 2; done'

# Check service health
echo "🔍 Checking service health..."
docker compose -f docker-compose.observability.yaml ps

echo ""
echo "✅ Migration complete!"
echo ""
echo "📊 SigNoz UI is available at: http://localhost:3301"
echo "🔧 OpenTelemetry endpoints:"
echo "   - Direct to SigNoz gRPC: localhost:4317"
echo "   - Direct to SigNoz HTTP: localhost:4318"
echo "   - Through collector gRPC: localhost:4315"
echo "   - Through collector HTTP: localhost:4316"
echo ""
echo "🔗 Your application should now send telemetry data to SigNoz"
echo "💡 Update your OTEL_EXPORTER_OTLP_ENDPOINT to http://localhost:4316"