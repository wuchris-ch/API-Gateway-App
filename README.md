# API Gateway Application - Rust Gateway

This is a comprehensive API Gateway application built with a custom Rust-based gateway, demonstrating high-performance API management with modern Rust technologies.

## Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Frontend      │    │   Rust Gateway   │    │   Backend API   │
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
                       │
                ┌──────────────┐
                │    Redis     │
                │  Port 6379   │
                └──────────────┘

```

## Features

### Rust API Gateway
- **High Performance**: Built with Rust and Tokio for maximum throughput and minimal latency
- **Load Balancing**: Multiple strategies (round-robin, least connections, random)
- **Rate Limiting**: Redis-backed rate limiting with configurable thresholds
- **Circuit Breaker**: Fault tolerance with automatic recovery
- **Health Monitoring**: Comprehensive health checks and service monitoring
- **Request Routing**: Intelligent request routing with path-based rules
- **Authentication**: JWT validation and API key management
- **Metrics**: Prometheus-compatible metrics collection
- **Async Processing**: Non-blocking I/O for maximum concurrency

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
- **Performance Dashboard**: Real-time metrics and gateway statistics

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
   git checkout rust-gateway
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

### Access Points

- **Frontend Application**: http://localhost:3000
- **Rust Gateway (Primary)**: http://localhost:8080
- **Backend API**: http://localhost:8000 (direct access, bypasses gateway)
- **Keycloak**: http://localhost:8180
- **PostgreSQL**: localhost:5432
- **Redis**: localhost:6379

## Rust Gateway Configuration

### Routes

The Rust Gateway is configured with the following routes:

- `/api/v1/*` → Backend API (with authentication)
- `/auth/*` → Authentication endpoints (public)
- `/health` → Health check (public)

### Features

- **Load Balancing**: Round-robin distribution across backend servers
- **Rate Limiting**: Configurable per-route and global rate limits
- **Circuit Breaker**: Automatic failure detection and recovery
- **Authentication**: JWT token validation and API key support
- **Health Checks**: Automatic backend health monitoring
- **Metrics**: Prometheus metrics at `/metrics`

### Configuration Example

```json
{
  "server": {
    "host": "0.0.0.0",
    "port": 8080
  },
  "routes": [
    {
      "path": "/api/v1/*",
      "backend": "backend_api",
      "rate_limit": 100,
      "auth_required": true,
      "timeout_ms": 30000
    }
  ],
  "backends": {
    "backend_api": {
      "servers": ["http://backend:8000"],
      "health_check": {
        "enabled": true,
        "path": "/health",
        "interval_seconds": 30
      }
    }
  }
}
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

### Rust Gateway Development

```bash
# Run the gateway locally (requires Redis and PostgreSQL)
cd api-gateway
cargo run

# Run tests
cargo test

# Check performance
cargo bench
```

### Testing

```bash
# Run all tests
make test

# Test gateway functionality
make test-gateway

# Load testing
make load-test
```

### API Documentation

- **Backend API Docs**: http://localhost:8000/docs (Swagger UI)
- **Gateway Metrics**: http://localhost:8080/metrics (Prometheus format)
- **Gateway Health**: http://localhost:8080/health

## Performance

### Benchmarks

The Rust Gateway is designed for high performance:

- **Throughput**: 50,000+ requests per second
- **Latency**: Sub-millisecond overhead
- **Memory**: Low memory footprint (~10MB base)
- **CPU**: Efficient CPU usage with async processing

### Load Testing

```bash
# Install wrk for load testing
brew install wrk  # macOS
sudo apt install wrk  # Ubuntu

# Test gateway performance
wrk -t12 -c400 -d30s http://localhost:8080/health

# Test with authentication
wrk -t12 -c400 -d30s -H "Authorization: Bearer <token>" http://localhost:8080/api/v1/users
```

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
KEYCLOAK_ADMIN_PASSWORD=your-secure-keycloak-password

# Gateway Configuration
RUST_LOG=info
GATEWAY_HOST=0.0.0.0
GATEWAY_PORT=8080
REDIS_URL=redis://redis:6379
DATABASE_URL=postgresql://postgres:password@postgres:5432/api_gateway
```

### Kubernetes Deployment

```yaml
# Example Kubernetes deployment
apiVersion: apps/v1
kind: Deployment
metadata:
  name: rust-gateway
spec:
  replicas: 3
  selector:
    matchLabels:
      app: rust-gateway
  template:
    metadata:
      labels:
        app: rust-gateway
    spec:
      containers:
      - name: rust-gateway
        image: api-gateway:latest
        ports:
        - containerPort: 8080
        env:
        - name: RUST_LOG
          value: "info"
```

## Monitoring and Observability

### Health Checks

```bash
# Check gateway health
curl http://localhost:8080/health

# Check backend health through gateway
curl http://localhost:8080/api/v1/health

# Check all services
docker-compose ps
```

### Metrics

The Rust Gateway exposes Prometheus metrics:

```bash
# View metrics
curl http://localhost:8080/metrics

# Key metrics:
# - http_requests_total
# - http_request_duration_seconds
# - active_connections
# - backend_health_status
```

### Logging

- Structured JSON logging with correlation IDs
- Configurable log levels via `RUST_LOG` environment variable
- Request/response logging with performance metrics

## Security

### Authentication Flow

1. **Login**: User authenticates via Keycloak or direct API
2. **JWT Token**: Backend issues JWT token
3. **API Access**: Client includes JWT in Authorization header
4. **Gateway Validation**: Rust gateway validates JWT before forwarding

### API Key Authentication

```bash
# Include API key in requests
curl -H "X-API-Key: your-api-key" http://localhost:8080/api/v1/users
```

### Rate Limiting

- **Per-route limits**: Configurable per endpoint
- **Global limits**: Overall request rate limiting
- **Burst handling**: Short-term burst allowances
- **Redis storage**: Distributed rate limiting across instances

## Advanced Features

### Circuit Breaker

Automatic failure detection and recovery:

```json
{
  "circuit_breaker": {
    "enabled": true,
    "failure_threshold": 5,
    "recovery_timeout_seconds": 60
  }
}
```

### Load Balancing Strategies

- **Round Robin**: Default, equal distribution
- **Least Connections**: Route to least busy server
- **Random**: Random server selection
- **Weighted**: Custom weights per server

### Request Transformation

- **Header manipulation**: Add, remove, or modify headers
- **Path rewriting**: Transform request paths
- **Query parameter handling**: Add or modify query parameters

## Troubleshooting

### Common Issues

1. **Gateway not starting**: Check Redis and PostgreSQL connections
2. **High latency**: Monitor backend health and circuit breaker status
3. **Rate limiting**: Check Redis connectivity and rate limit configuration

### Debug Commands

```bash
# Check gateway configuration
curl http://localhost:8080/config

# View backend health
curl http://localhost:8080/backends/health

# Check rate limit status
curl http://localhost:8080/rate-limits/status
```

### Performance Tuning

```bash
# Increase connection pool sizes
GATEWAY_DB_POOL_SIZE=20
GATEWAY_REDIS_POOL_SIZE=20

# Adjust rate limiting
GATEWAY_DEFAULT_RATE_LIMIT=1000

# Enable request compression
GATEWAY_COMPRESSION=true
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests (especially for the Rust gateway)
5. Submit a pull request

### Development Setup

```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install development dependencies
cd api-gateway
cargo install cargo-watch
cargo install cargo-audit

# Run in development mode with hot reload
cargo watch -x run
```

## License

This project is licensed under the MIT License - see the LICENSE file for details. 