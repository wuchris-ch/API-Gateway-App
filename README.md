# API Gateway Application - Kong Gateway

This is a comprehensive API Gateway application built with Kong Gateway, demonstrating enterprise-grade API management capabilities.

## Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Frontend      │    │   Kong Gateway   │    │   Backend API   │
│  (React/TS)     │────│   (Port 8080)    │────│  (FastAPI)      │
│  Port 3000      │    │                  │    │  Port 8000      │
└─────────────────┘    └──────────────────┘    └─────────────────┘
                                │
                       ┌────────┴────────┐
                       │                 │
                ┌──────────────┐  ┌──────────────┐
                │  Keycloak    │  │  PostgreSQL  │
                │  Port 8180   │  │  Port 5432   │
                └──────────────┘  └──────────────┘

```

## Features

### Kong Gateway
- **API Gateway**: Route management, load balancing, and request/response transformation
- **Authentication & Authorization**: JWT validation, API key management, OAuth2 integration
- **Rate Limiting**: Configurable rate limits per API key or user
- **Monitoring**: Access logs, metrics, and health checks
- **Admin API**: RESTful API for configuration management
- **Kong Manager**: Web-based GUI for visual configuration

### Backend Services
- **FastAPI Backend**: High-performance Python API with automatic OpenAPI documentation
- **Database**: PostgreSQL with connection pooling and migrations
- **Caching**: Redis for session storage and rate limiting
- **Authentication**: JWT token validation and API key management

### Frontend
- **React with TypeScript**: Modern, responsive web interface
- **Authentication UI**: Login, registration, and token management
- **API Testing**: Built-in tools for testing API endpoints
- **Real-time Updates**: WebSocket support for live data

### Infrastructure
- **Containerized**: Full Docker Compose setup for easy deployment
- **Health Checks**: Comprehensive health monitoring across all services
- **Security**: TLS/SSL support, secure secrets management
- **Development Tools**: Hot reload, debugging support, and testing utilities

## Quick Start

### Prerequisites
- Docker and Docker Compose
- Git

### Installation

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd API-Gateway-App
   git checkout kong-gateway
   ```

2. **Set up environment variables**
   ```bash
   cp env.example .env
   # Edit .env with your specific configuration
   ```

3. **Start the application**
   ```bash
   docker-compose up -d
   ```

4. **Initialize Kong configuration**
   ```bash
   # Wait for Kong to be healthy, then configure services and routes
   ./scripts/setup-kong.sh
   ```

### Access Points

- **Frontend Application**: http://localhost:3000
- **Kong Gateway (Proxy)**: http://localhost:8080
- **Kong Admin API**: http://localhost:8001
- **Kong Manager**: http://localhost:8002
- **Backend API**: http://localhost:8000 (direct access, bypasses Kong)
- **Keycloak**: http://localhost:8180
- **PostgreSQL**: localhost:5432

## Kong Configuration

### Services and Routes

The Kong Gateway is configured with the following routes:

- `/api/v1/*` → Backend API (with authentication)
- `/auth/*` → Authentication endpoints (public)
- `/health` → Health check (public)

### Plugins

- **JWT Authentication**: Validates JWT tokens
- **Key Authentication**: API key validation
- **Rate Limiting**: Configurable limits per consumer
- **CORS**: Cross-origin resource sharing
- **Request/Response Logging**: Comprehensive logging

### Consumer Management

```bash
# Create a consumer
curl -X POST http://localhost:8001/consumers \
  --data "username=testuser"

# Create API key for consumer
curl -X POST http://localhost:8001/consumers/testuser/key-auth \
  --data "key=my-api-key"
```

## Development

### Local Development

```bash
# Start development environment
make dev

# View logs
make logs

# Reset environment
make clean && make dev
```

### Testing

```bash
# Run all tests
make test

# Test Kong configuration
make test-kong

# Test backend API
make test-api
```

### API Documentation

- **Backend API Docs**: http://localhost:8000/docs (Swagger UI)
- **Kong Admin API**: http://localhost:8001 (Kong's admin endpoints)

## Production Deployment

### Using Docker Compose Production

```bash
# Production deployment
docker-compose -f docker-compose.prod.yml up -d
```

### Environment Variables

Key environment variables for production:

```bash
# Security
JWT_SECRET_KEY=your-production-jwt-secret
POSTGRES_PASSWORD=your-secure-postgres-password
KONG_DB_PASSWORD=your-secure-kong-db-password
KEYCLOAK_ADMIN_PASSWORD=your-secure-keycloak-password

# Kong Configuration
KONG_ADMIN_LISTEN=127.0.0.1:8001  # Restrict admin access
KONG_PROXY_ACCESS_LOG=/var/log/kong/access.log
KONG_PROXY_ERROR_LOG=/var/log/kong/error.log
```

## Monitoring and Observability

### Health Checks

All services include health checks:

```bash
# Check Kong health
curl http://localhost:8080/health

# Check backend health
curl http://localhost:8000/health

# Check all services
docker-compose ps
```

### Logging

- Kong logs: Available through Docker logs
- Backend logs: Structured JSON logging
- Database logs: PostgreSQL logs with query logging

### Metrics

Kong provides built-in metrics and can be integrated with:
- Prometheus
- Grafana
- DataDog
- New Relic

## Security

### Authentication Flow

1. **Login**: User authenticates via Keycloak or direct API
2. **JWT Token**: Backend issues JWT token
3. **API Access**: Client includes JWT in Authorization header
4. **Kong Validation**: Kong validates JWT before forwarding to backend

### API Key Authentication

```bash
# Include API key in requests
curl -H "X-API-Key: your-api-key" http://localhost:8080/api/v1/users
```

### Security Headers

Kong automatically adds security headers:
- CORS headers
- Rate limiting headers
- Security policy headers

## Troubleshooting

### Common Issues

1. **Kong not starting**: Check database connection and migrations
2. **Authentication failing**: Verify JWT secret configuration
3. **Rate limiting**: Check consumer configuration and limits

### Debug Commands

```bash
# Check Kong configuration
curl http://localhost:8001/services

# View Kong routes
curl http://localhost:8001/routes

# Check plugins
curl http://localhost:8001/plugins
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details. 