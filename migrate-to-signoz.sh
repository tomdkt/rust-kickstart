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
sleep 30

# Check service health
echo "🔍 Checking service health..."
docker compose -f docker-compose.observability.yaml ps

echo "✅ Migration complete!"
echo ""
echo "📊 SigNoz UI is available at: http://localhost:3301"
echo "🔧 OpenTelemetry endpoints:"
echo "   - gRPC: localhost:4315"
echo "   - HTTP: localhost:4316"
echo ""
echo "🔗 Your application should now send telemetry data to SigNoz"