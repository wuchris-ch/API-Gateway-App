#!/bin/bash

echo "🚀 Testing API Gateway Infrastructure..."

# Start core services
echo "📦 Starting PostgreSQL and Redis..."
docker-compose up -d postgres redis

# Wait for health checks
echo "⏳ Waiting for services to be healthy..."
sleep 10

# Check service status
echo "🔍 Checking service status..."
docker-compose ps

# Start Kong database
echo "📦 Starting Kong database..."
docker-compose up -d kong-database

# Wait for Kong database
echo "⏳ Waiting for Kong database..."
sleep 10

# Run Kong migrations
echo "🔄 Running Kong migrations..."
docker-compose up kong-migration

# Check migration status
echo "🔍 Checking migration status..."
docker-compose ps kong-migration

# Start Kong Gateway
echo "📦 Starting Kong Gateway..."
docker-compose up -d kong

# Wait for Kong
echo "⏳ Waiting for Kong Gateway..."
sleep 15

# Test Kong health
echo "🏥 Testing Kong health..."
curl -f http://localhost:8001/status || echo "Kong not ready yet"

# Start Keycloak
echo "📦 Starting Keycloak..."
docker-compose up -d keycloak

echo "✅ Infrastructure test complete!"
echo "🌐 Services should be available at:"
echo "   - Kong Admin: http://localhost:8001"
echo "   - Kong Proxy: http://localhost:8000"
echo "   - Keycloak: http://localhost:8180"
echo "   - PostgreSQL: localhost:5432"
echo "   - Redis: localhost:6379" 