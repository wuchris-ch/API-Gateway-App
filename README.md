# API Gateway Application

This repository demonstrates two different approaches to building a production-ready API Gateway application. Each approach is implemented in a separate branch to avoid confusion and allow you to explore each independently.

## 🌟 Choose Your Gateway Approach

### 🦀 Rust Gateway (Custom Implementation)
**Branch: `rust-gateway`**

A high-performance, custom-built API Gateway written in Rust with Tokio for async processing.

```bash
git checkout rust-gateway
```

**Features:**
- **Ultra High Performance**: 50,000+ RPS with sub-millisecond latency
- **Custom Logic**: Complete control over gateway behavior
- **Rust Ecosystem**: Built with Axum, Tokio, and Redis
- **Memory Efficient**: ~10MB memory footprint
- **Circuit Breaker**: Built-in fault tolerance
- **Custom Rate Limiting**: Redis-backed with burst handling
- **Prometheus Metrics**: Built-in observability

**Best For:**
- High-throughput applications
- Custom gateway logic requirements  
- Learning Rust and async programming
- Maximum performance optimization
- Microservices with specific routing needs

---

### 🐒 Kong Gateway (Enterprise Solution)
**Branch: `kong-gateway`**

Production-ready API Gateway using Kong, an enterprise-grade solution used by thousands of companies.

```bash
git checkout kong-gateway
```

**Features:**
- **Enterprise Ready**: Battle-tested in production environments
- **Plugin Ecosystem**: 100+ plugins available
- **Admin Interface**: Web-based management UI
- **Declarative Config**: Configuration as code
- **Multi-Protocol**: HTTP/HTTPS, gRPC, WebSocket support
- **Enterprise Support**: Commercial support available
- **Industry Standard**: Well-known by DevOps teams

**Best For:**
- Enterprise environments
- Teams familiar with Kong
- Quick deployment needs
- Extensive plugin requirements
- Production environments requiring support

---

## 🏗️ Common Architecture

Both approaches share the same backend infrastructure:

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Frontend      │    │   API Gateway    │    │   Backend API   │
│  (React/TS)     │────│  (Kong/Rust)     │────│  (FastAPI)      │
│  Port 3000      │    │   Port 8080      │    │  Port 8000      │
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

### Shared Components

**Backend Services:**
- **FastAPI Backend**: Python API with automatic OpenAPI docs
- **PostgreSQL Database**: Persistent data storage
- **Redis Cache**: Session storage and rate limiting
- **Keycloak Auth**: OAuth/OIDC authentication server

**Frontend:**
- **React + TypeScript**: Modern web interface
- **Material-UI**: Professional component library
- **Authentication UI**: Login/register flows
- **API Testing Tools**: Built-in endpoint testing

**Infrastructure:**
- **Docker Compose**: Container orchestration
- **Health Checks**: Service monitoring
- **Environment Configuration**: Flexible deployment options

## 🚀 Quick Start

### 1. Choose Your Approach

**For High Performance & Custom Logic:**
```bash
git checkout rust-gateway
```

**For Enterprise & Quick Setup:**
```bash
git checkout kong-gateway
```

### 2. Set Up Environment
```bash
cp env.example .env
# Edit .env with your configuration
```

### 3. Start the Application
```bash
docker-compose up -d
```

### 4. Access Points

| Service | Rust Gateway | Kong Gateway |
|---------|-------------|-------------|
| Frontend | http://localhost:3000 | http://localhost:3000 |
| Gateway | http://localhost:8080 | http://localhost:8080 |
| Backend | http://localhost:8000 | http://localhost:8000 |
| Auth | http://localhost:8180 | http://localhost:8180 |
| Admin UI | N/A | http://localhost:8002 |

## 📊 Comparison

| Feature | Rust Gateway | Kong Gateway |
|---------|-------------|-------------|
| **Performance** | ⭐⭐⭐⭐⭐ Ultra High | ⭐⭐⭐⭐ High |
| **Memory Usage** | ⭐⭐⭐⭐⭐ Very Low | ⭐⭐⭐ Medium |
| **Setup Time** | ⭐⭐⭐ Medium | ⭐⭐⭐⭐⭐ Very Fast |
| **Customization** | ⭐⭐⭐⭐⭐ Full Control | ⭐⭐⭐ Plugin-based |
| **Enterprise Features** | ⭐⭐ Basic | ⭐⭐⭐⭐⭐ Extensive |
| **Learning Curve** | ⭐⭐ Steep | ⭐⭐⭐⭐ Gentle |
| **Community** | ⭐⭐⭐ Growing | ⭐⭐⭐⭐⭐ Large |
| **Production Ready** | ⭐⭐⭐ Good | ⭐⭐⭐⭐⭐ Excellent |

## 🛠️ Development

### Common Commands

```bash
# Start development environment
make dev

# View logs
make logs

# Run tests
make test

# Clean and restart
make clean && make dev
```

### Branch-Specific Development

**Rust Gateway Development:**
```bash
git checkout rust-gateway
cd api-gateway
cargo run
cargo test
cargo bench
```

**Kong Gateway Configuration:**
```bash
git checkout kong-gateway
# Configure Kong via Admin API
curl -X POST http://localhost:8001/services \
  --data "name=backend" \
  --data "url=http://backend:8000"
```

## 📚 Documentation

Each branch contains comprehensive documentation:

- **Rust Gateway**: Detailed performance tuning, custom middleware, and Rust-specific guides
- **Kong Gateway**: Enterprise deployment, plugin configuration, and Kong administration

## 🤝 Contributing

1. Fork the repository
2. Choose your preferred branch (`rust-gateway` or `kong-gateway`)
3. Create a feature branch from your chosen approach
4. Make your changes
5. Add tests
6. Submit a pull request

## 📋 Use Cases

### Choose Rust Gateway If:
- You need maximum performance (50k+ RPS)
- You want to learn Rust and async programming
- You need custom gateway logic
- Memory usage is a concern
- You're building high-frequency trading systems
- You enjoy low-level optimization

### Choose Kong Gateway If:
- You need enterprise features out-of-the-box
- Your team is familiar with Kong
- You want extensive plugin ecosystem
- You need commercial support
- You're building enterprise applications
- You prefer configuration over coding

## 🔗 Related Projects

- **Kong Gateway**: https://github.com/Kong/kong
- **Axum Framework**: https://github.com/tokio-rs/axum
- **FastAPI**: https://github.com/tiangolo/fastapi
- **Keycloak**: https://github.com/keycloak/keycloak

## 📝 License

This project is licensed under the MIT License - see the LICENSE file for details.

---

**Start exploring:** Choose your gateway approach and dive into either the `rust-gateway` or `kong-gateway` branch! 