#!/bin/bash

echo "🔍 SigNoz Debug Information"
echo "=========================="

echo ""
echo "📊 Service Status:"
docker compose -f docker-compose.observability.yaml ps

echo ""
echo "🏥 Service Health Checks:"
echo "ClickHouse:"
docker exec signoz-clickhouse clickhouse-client --query "SELECT 1" 2>/dev/null && echo "  ✅ ClickHouse is responding" || echo "  ❌ ClickHouse is not responding"

echo "Query Service:"
curl -s http://localhost:8080/api/v1/health >/dev/null 2>&1 && echo "  ✅ Query Service is healthy" || echo "  ❌ Query Service is not responding"

echo "Frontend:"
curl -s http://localhost:3301 >/dev/null 2>&1 && echo "  ✅ Frontend is accessible" || echo "  ❌ Frontend is not accessible"

echo "SigNoz OTel Collector:"
nc -z localhost 4317 2>/dev/null && echo "  ✅ OTLP gRPC port 4317 is open" || echo "  ❌ OTLP gRPC port 4317 is not accessible"
nc -z localhost 4318 2>/dev/null && echo "  ✅ OTLP HTTP port 4318 is open" || echo "  ❌ OTLP HTTP port 4318 is not accessible"

echo "Main OTel Collector:"
nc -z localhost 4315 2>/dev/null && echo "  ✅ OTLP gRPC port 4315 is open" || echo "  ❌ OTLP gRPC port 4315 is not accessible"
nc -z localhost 4316 2>/dev/null && echo "  ✅ OTLP HTTP port 4316 is open" || echo "  ❌ OTLP HTTP port 4316 is not accessible"

echo ""
echo "📋 Recent Logs (last 20 lines):"
echo ""
echo "ClickHouse logs:"
docker logs --tail 20 signoz-clickhouse 2>/dev/null || echo "  ❌ Cannot access ClickHouse logs"

echo ""
echo "Query Service logs:"
docker logs --tail 20 signoz-query-service 2>/dev/null || echo "  ❌ Cannot access Query Service logs"

echo ""
echo "SigNoz OTel Collector logs:"
docker logs --tail 20 signoz-otel-collector 2>/dev/null || echo "  ❌ Cannot access SigNoz OTel Collector logs"

echo ""
echo "Main OTel Collector logs:"
docker logs --tail 20 otel-collector 2>/dev/null || echo "  ❌ Cannot access Main OTel Collector logs"

echo ""
echo "🔗 Access URLs:"
echo "  - SigNoz UI: http://localhost:3301"
echo "  - Query Service API: http://localhost:8080"
echo "  - ClickHouse HTTP: http://localhost:8123"
echo ""
echo "📡 OTLP Endpoints:"
echo "  - Direct to SigNoz gRPC: localhost:4317"
echo "  - Direct to SigNoz HTTP: localhost:4318"
echo "  - Through collector gRPC: localhost:4315"
echo "  - Through collector HTTP: localhost:4316"

echo ""
echo "💾 Docker Volumes:"
docker volume ls | grep signoz

echo ""
echo "🌐 Docker Networks:"
docker network ls | grep signoz