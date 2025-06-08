from fastapi import FastAPI, HTTPException
from fastapi.middleware.cors import CORSMiddleware
from typing import List
import uvicorn
import time

app = FastAPI(
    title="API Gateway Backend (Demo)",
    description="A simplified backend API for demonstration",
    version="1.0.0",
    docs_url="/docs",
    redoc_url="/redoc"
)

# CORS middleware
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Mock data
mock_users = [
    {"id": 1, "username": "admin", "email": "admin@example.com", "is_admin": True},
    {"id": 2, "username": "user", "email": "user@example.com", "is_admin": False},
]

mock_products = [
    {"id": 1, "name": "Laptop", "price": 999.99, "category": "Electronics", "stock_quantity": 10},
    {"id": 2, "name": "Mouse", "price": 29.99, "category": "Electronics", "stock_quantity": 50},
    {"id": 3, "name": "Keyboard", "price": 79.99, "category": "Electronics", "stock_quantity": 25},
]

mock_orders = [
    {"id": 1, "user_id": 1, "total_amount": 999.99, "status": "completed", "created_at": "2024-01-15T10:00:00Z"},
    {"id": 2, "user_id": 2, "total_amount": 109.98, "status": "pending", "created_at": "2024-01-16T14:30:00Z"},
]

# Health check endpoint
@app.get("/health")
async def health_check():
    return {"status": "healthy", "timestamp": time.time()}

# Root endpoint
@app.get("/")
async def root():
    return {"message": "API Gateway Backend is running", "version": "1.0.0"}

# Authentication endpoints
@app.post("/auth/login")
async def login(username: str, password: str):
    # Simple mock authentication
    if username == "admin" and password == "admin":
        return {"access_token": "mock-admin-token", "token_type": "bearer", "user": {"username": "admin", "is_admin": True}}
    elif username == "user" and password == "user":
        return {"access_token": "mock-user-token", "token_type": "bearer", "user": {"username": "user", "is_admin": False}}
    else:
        raise HTTPException(status_code=401, detail="Invalid credentials")

# User endpoints
@app.get("/users/")
async def get_users():
    return mock_users

@app.get("/users/{user_id}")
async def get_user(user_id: int):
    user = next((u for u in mock_users if u["id"] == user_id), None)
    if not user:
        raise HTTPException(status_code=404, detail="User not found")
    return user

# Product endpoints
@app.get("/products/")
async def get_products():
    return mock_products

@app.get("/products/{product_id}")
async def get_product(product_id: int):
    product = next((p for p in mock_products if p["id"] == product_id), None)
    if not product:
        raise HTTPException(status_code=404, detail="Product not found")
    return product

@app.post("/products/")
async def create_product(name: str, price: float, category: str, stock_quantity: int = 0):
    new_id = max([p["id"] for p in mock_products]) + 1
    new_product = {
        "id": new_id,
        "name": name,
        "price": price,
        "category": category,
        "stock_quantity": stock_quantity
    }
    mock_products.append(new_product)
    return new_product

# Order endpoints
@app.get("/orders/")
async def get_orders():
    return mock_orders

@app.get("/orders/{order_id}")
async def get_order(order_id: int):
    order = next((o for o in mock_orders if o["id"] == order_id), None)
    if not order:
        raise HTTPException(status_code=404, detail="Order not found")
    return order

# Metrics endpoint
@app.get("/metrics")
async def metrics():
    return {
        "total_users": len(mock_users),
        "total_products": len(mock_products),
        "total_orders": len(mock_orders),
        "uptime": time.time()
    }

if __name__ == "__main__":
    uvicorn.run(
        "simple_main:app",
        host="0.0.0.0",
        port=9000,
        reload=True,
        log_level="info"
    ) 