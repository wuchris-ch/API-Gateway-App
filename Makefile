.PHONY: help setup start stop clean test build deploy

# Default target
help:
	@echo "API Gateway Application - Available Commands:"
	@echo ""
	@echo "  setup     - Install dependencies and setup the project"
	@echo "  start     - Start all services"
	@echo "  stop      - Stop all services"
	@echo "  clean     - Clean up containers and volumes"
	@echo "  test      - Run all tests"
	@echo "  build     - Build all components"
	@echo "  deploy    - Deploy to production"
	@echo "  logs      - Show logs from all services"
	@echo "  status    - Show status of all services"
	@echo ""

# Setup project dependencies
setup:
	@echo "Setting up API Gateway project..."
	@echo "Installing Python dependencies..."
	cd backend && pip install -r requirements.txt
	@echo "Installing Rust dependencies..."
	cd api-gateway && cargo build
	@echo "Installing Node.js dependencies..."
	cd frontend && npm install
	@echo "Setup complete!"

# Start all services
start:
	@echo "Starting API Gateway services..."
	docker-compose up -d postgres keycloak kong redis
	@echo "Waiting for services to be ready..."
	sleep 30
	@echo "Starting backend API..."
	cd backend && python main.py &
	@echo "Starting API Gateway..."
	cd api-gateway && cargo run &
	@echo "Starting frontend..."
	cd frontend && npm start &
	@echo "All services started!"
	@echo ""
	@echo "Access points:"
	@echo "  Frontend:    http://localhost:3000"
	@echo "  API Gateway: http://localhost:8080"
	@echo "  Backend API: http://localhost:8000"
	@echo "  Kong Admin:  http://localhost:8001"
	@echo "  Keycloak:    http://localhost:8180"

# Stop all services
stop:
	@echo "Stopping API Gateway services..."
	docker-compose down
	pkill -f "python main.py" || true
	pkill -f "cargo run" || true
	pkill -f "npm start" || true
	@echo "All services stopped!"

# Clean up everything
clean:
	@echo "Cleaning up API Gateway project..."
	docker-compose down -v
	docker system prune -f
	cd backend && find . -name "__pycache__" -type d -exec rm -rf {} + || true
	cd api-gateway && cargo clean
	cd frontend && rm -rf node_modules || true
	@echo "Cleanup complete!"

# Run tests
test:
	@echo "Running tests..."
	@echo "Testing backend..."
	cd backend && python -m pytest tests/ -v
	@echo "Testing API Gateway..."
	cd api-gateway && cargo test
	@echo "Testing frontend..."
	cd frontend && npm test
	@echo "All tests completed!"

# Build all components
build:
	@echo "Building all components..."
	@echo "Building backend..."
	cd backend && python -m py_compile main.py
	@echo "Building API Gateway..."
	cd api-gateway && cargo build --release
	@echo "Building frontend..."
	cd frontend && npm run build
	@echo "Build complete!"

# Deploy to production
deploy:
	@echo "Deploying to production..."
	@echo "Building production images..."
	docker-compose -f docker-compose.prod.yml build
	@echo "Starting production services..."
	docker-compose -f docker-compose.prod.yml up -d
	@echo "Production deployment complete!"

# Show logs
logs:
	@echo "Showing logs from all services..."
	docker-compose logs -f

# Show service status
status:
	@echo "Service Status:"
	@echo "==============="
	docker-compose ps
	@echo ""
	@echo "Health Checks:"
	@echo "=============="
	@curl -s http://localhost:8080/health | jq . || echo "API Gateway: Not responding"
	@curl -s http://localhost:8000/health | jq . || echo "Backend API: Not responding"
	@curl -s http://localhost:8001 > /dev/null && echo "Kong Admin: Running" || echo "Kong Admin: Not responding"

# Development helpers
dev-backend:
	cd backend && python main.py

dev-gateway:
	cd api-gateway && cargo run

dev-frontend:
	cd frontend && npm start

# Database operations
db-migrate:
	@echo "Running database migrations..."
	cd backend && alembic upgrade head

db-reset:
	@echo "Resetting database..."
	docker-compose exec postgres psql -U postgres -d api_gateway -c "DROP SCHEMA public CASCADE; CREATE SCHEMA public;"
	docker-compose exec postgres psql -U postgres -d api_gateway -f /docker-entrypoint-initdb.d/init.sql

# Kong configuration
kong-config:
	@echo "Configuring Kong Gateway..."
	./scripts/configure-kong.sh

# Monitoring
monitor:
	@echo "Opening monitoring dashboard..."
	open http://localhost:8080/metrics

# Load testing
load-test:
	@echo "Running load tests..."
	./scripts/load-test.sh

# Security scan
security-scan:
	@echo "Running security scans..."
	cd backend && safety check
	cd api-gateway && cargo audit
	cd frontend && npm audit 