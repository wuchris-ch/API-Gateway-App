# API Gateway Demo Application

A comprehensive, production-ready API Gateway demonstration built with modern technologies and best practices. This project showcases a complete microservices architecture with authentication, rate limiting, monitoring, and a beautiful web interface.

## 🏗️ Architecture Overview

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Frontend      │    │   API Gateway   │    │   Backend API   │
│   (React/TS)    │◄──►│   (Rust)        │◄──►│   (FastAPI)     │
│   Port: 3000    │    │   Port: 8080    │    │   Port: 8000    │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         │              ┌─────────────────┐              │
         │              │   Kong Gateway  │              │
         │              │   Port: 8000    │              │
         │              └─────────────────┘              │
         │                       │                       │
         └───────────────────────┼───────────────────────┘
                                 │
                    ┌─────────────────┐
                    │   PostgreSQL    │
                    │   Port: 5432    │
                    └─────────────────┘
                                 │
                    ┌─────────────────┐
                    │     Redis       │
                    │   Port: 6379    │
                    └─────────────────┘
```

## 🚀 Features

### Core Features
- **Multi-language Architecture**: Rust API Gateway, Python Backend, TypeScript Frontend
- **Authentication & Authorization**: JWT tokens and API key management
- **Rate Limiting**: Redis-backed rate limiting with configurable thresholds
- **Load Balancing**: Multiple strategies (round-robin, least connections, random)
- **Health Monitoring**: Comprehensive health checks and service monitoring
- **Request Routing**: Intelligent request routing with path-based rules
- **Circuit Breaker**: Fault tolerance with automatic recovery
- **Metrics & Analytics**: Prometheus metrics and system monitoring

### Frontend Features
- **Modern React UI**: Built with Material-UI components
- **Responsive Design**: Mobile-friendly interface
- **Real-time Dashboard**: System metrics and service status
- **API Testing Tool**: Built-in API endpoint testing
- **User Management**: User and role management interface
- **Product Catalog**: CRUD operations for products
- **Order Management**: Order tracking and management

### Backend Features
- **RESTful API**: Complete CRUD operations
- **Database Integration**: PostgreSQL with SQLAlchemy ORM
- **Caching**: Redis integration for performance
- **Structured Logging**: JSON-formatted logs with correlation IDs
- **Input Validation**: Pydantic schemas for request/response validation
- **Error Handling**: Comprehensive error handling and reporting

## 🛠️ Technology Stack

### Frontend
- **React 18** with TypeScript
- **Material-UI (MUI)** for components
- **React Router** for navigation
- **Axios** for HTTP requests
- **Nginx** for production serving

### API Gateway (Rust)
- **Axum** web framework
- **Tokio** async runtime
- **Redis** for rate limiting and caching
- **PostgreSQL** for configuration storage
- **Prometheus** metrics collection

### Backend API (Python)
- **FastAPI** web framework
- **SQLAlchemy** ORM
- **PostgreSQL** database
- **Redis** caching
- **Pydantic** data validation
- **JWT** authentication

### Infrastructure
- **Docker & Docker Compose** for containerization
- **Kong Gateway** for additional API management
- **PostgreSQL** for data persistence
- **Redis** for caching and rate limiting
- **Nginx** for frontend serving

## 📋 Prerequisites

- **Docker** and **Docker Compose**
- **Node.js 18+** (for local frontend development)
- **Python 3.11+** (for local backend development)
- **Rust 1.75+** (for local gateway development)
- **Make** (optional, for using Makefile commands)

## 🚀 Quick Start

### 1. Clone the Repository
```bash
git clone <repository-url>
cd API-Gateway-App
```

### 2. Environment Setup
```bash
# Copy environment template
cp env.example .env

# Edit environment variables (optional for demo)
nano .env
```

### 3. Start the Application
```bash
# Using Docker Compose (recommended)
docker-compose up -d

# Or using Make
make start
```

### 4. Access the Application
- **Frontend**: http://localhost:3000
- **API Gateway**: http://localhost:8080
- **Backend API**: http://localhost:8000
- **Kong Admin**: http://localhost:8001
- **Kong Manager**: http://localhost:8002

### 5. Login Credentials
```
Admin User:
Username: admin
Password: admin

Regular User:
Username: user
Password: user
```

## 🔧 Development Setup

### Frontend Development
```bash
cd frontend
npm install
npm start
```

### Backend Development
```bash
cd backend
pip install -r requirements.txt
python main.py
```

### API Gateway Development
```bash
cd api-gateway
cargo run
```

## 🐳 Docker Deployment

### Development Environment
```bash
docker-compose up -d
```

### Production Environment
```bash
docker-compose -f docker-compose.prod.yml up -d
```

## 📊 Monitoring & Metrics

### Health Checks
- **API Gateway**: http://localhost:8080/health
- **Backend API**: http://localhost:8000/health
- **Frontend**: http://localhost:3000/health

### Metrics Endpoints
- **API Gateway Metrics**: http://localhost:8080/metrics
- **Backend Metrics**: http://localhost:8000/metrics

### Dashboard
Access the monitoring dashboard at http://localhost:3000 after logging in.

## 🔐 Security Features

- **JWT Authentication**: Secure token-based authentication
- **API Key Management**: Generate and manage API keys
- **Rate Limiting**: Prevent abuse with configurable limits
- **CORS Protection**: Cross-origin request security
- **Input Validation**: Comprehensive request validation
- **Security Headers**: Standard security headers implementation

## 🧪 Testing

### API Testing
Use the built-in API tester at http://localhost:3000/api-tester or:

```bash
# Health check
curl http://localhost:8080/health

# Get products (requires authentication)
curl -H "Authorization: Bearer <token>" http://localhost:8080/api/v1/products

# Test rate limiting
for i in {1..10}; do curl http://localhost:8080/api/v1/products; done
```

### Load Testing
```bash
# Install Apache Bench
sudo apt-get install apache2-utils

# Test API Gateway
ab -n 1000 -c 10 http://localhost:8080/health
```

## 📁 Project Structure

```
API-Gateway-App/
├── frontend/                 # React TypeScript frontend
│   ├── src/
│   │   ├── components/      # Reusable UI components
│   │   ├── pages/          # Page components
│   │   ├── contexts/       # React contexts
│   │   └── App.tsx         # Main application
│   ├── public/             # Static assets
│   ├── Dockerfile          # Frontend container
│   └── nginx.conf          # Nginx configuration
├── backend/                 # FastAPI Python backend
│   ├── main.py             # Application entry point
│   ├── models.py           # Database models
│   ├── schemas.py          # Pydantic schemas
│   ├── auth.py             # Authentication logic
│   ├── config.py           # Configuration management
│   ├── requirements.txt    # Python dependencies
│   └── Dockerfile          # Backend container
├── api-gateway/            # Rust API Gateway
│   ├── src/
│   │   ├── main.rs         # Gateway entry point
│   │   ├── proxy.rs        # Request proxying
│   │   ├── auth.rs         # Authentication middleware
│   │   ├── rate_limiter.rs # Rate limiting
│   │   └── metrics.rs      # Metrics collection
│   ├── Cargo.toml          # Rust dependencies
│   └── Dockerfile          # Gateway container
├── database/               # Database initialization
│   └── init.sql           # Database schema
├── docker-compose.yml      # Development environment
├── docker-compose.prod.yml # Production environment
├── Makefile               # Build and deployment commands
└── README.md              # This file
```

## 🔧 Configuration

### Environment Variables
Key configuration options in `.env`:

```bash
# Database
POSTGRES_PASSWORD=your_secure_password
KONG_DB_PASSWORD=your_kong_password

# Security
JWT_SECRET_KEY=your_jwt_secret

# Rate Limiting
RATE_LIMIT_REQUESTS_PER_MINUTE=60

# CORS
ALLOWED_ORIGINS=http://localhost:3000
```

### API Gateway Configuration
The Rust API Gateway can be configured via environment variables or JSON configuration.

## 🚀 Deployment

### Production Deployment
1. Set production environment variables
2. Build and deploy with Docker Compose:
```bash
docker-compose -f docker-compose.prod.yml up -d
```

### Cloud Deployment
The application is ready for deployment on:
- **AWS ECS/EKS**
- **Google Cloud Run/GKE**
- **Azure Container Instances/AKS**
- **DigitalOcean App Platform**

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## 📝 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🆘 Troubleshooting

### Common Issues

**Port Conflicts**
```bash
# Check what's using the ports
lsof -i :3000 -i :8000 -i :8080

# Stop conflicting services
docker-compose down
```

**Database Connection Issues**
```bash
# Reset database
docker-compose down -v
docker-compose up -d postgres
```

**Build Failures**
```bash
# Clean and rebuild
docker-compose down
docker system prune -f
docker-compose build --no-cache
```

### Logs
```bash
# View all logs
docker-compose logs -f

# View specific service logs
docker-compose logs -f api-gateway
docker-compose logs -f backend
docker-compose logs -f frontend
```

## 📞 Support

For questions, issues, or contributions:
- Create an issue in the repository
- Check the troubleshooting section
- Review the logs for error details

---

**Built with ❤️ using modern technologies and best practices** 