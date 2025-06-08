#!/bin/bash

echo "🚀 Starting API Gateway Demo in Development Mode..."

# Start infrastructure services
echo "📦 Starting infrastructure services..."
docker-compose up -d postgres redis kong-database kong-migration kong keycloak

echo "⏳ Waiting for services to be ready..."
sleep 20

# Check if Node.js is available for frontend
if command -v npm &> /dev/null; then
    echo "📦 Starting frontend development server..."
    cd frontend
    npm install
    npm start &
    FRONTEND_PID=$!
    cd ..
    echo "✅ Frontend started at http://localhost:3000"
else
    echo "⚠️  Node.js not found. Please install Node.js to run the frontend."
fi

# Check if Python is available for backend
if command -v python3 &> /dev/null; then
    echo "📦 Starting backend development server..."
    cd backend
    pip install -r requirements.txt
    python main.py &
    BACKEND_PID=$!
    cd ..
    echo "✅ Backend started at http://localhost:8000"
else
    echo "⚠️  Python not found. Please install Python to run the backend."
fi

echo "🌐 Services available at:"
echo "   - Frontend: http://localhost:3000"
echo "   - Backend API: http://localhost:8000"
echo "   - Kong Admin: http://localhost:8001"
echo "   - Kong Proxy: http://localhost:8000"
echo "   - Keycloak: http://localhost:8180"

echo "📝 Login credentials:"
echo "   - Username: admin | Password: admin"
echo "   - Username: user | Password: user"

echo "🛑 Press Ctrl+C to stop all services"

# Wait for interrupt
trap 'echo "🛑 Stopping services..."; kill $FRONTEND_PID $BACKEND_PID 2>/dev/null; docker-compose down; exit' INT
wait 