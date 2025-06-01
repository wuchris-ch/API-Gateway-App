from pydantic_settings import BaseSettings
from typing import Optional
import os

class Settings(BaseSettings):
    # Database settings
    database_url: str = "postgresql://postgres:postgres@localhost:5432/api_gateway"
    
    # JWT settings
    secret_key: str = "your-secret-key-change-in-production"
    algorithm: str = "HS256"
    access_token_expire_minutes: int = 30
    
    # Redis settings
    redis_url: str = "redis://localhost:6379"
    
    # API settings
    api_title: str = "API Gateway Backend"
    api_version: str = "1.0.0"
    debug: bool = True
    
    # CORS settings
    allowed_origins: list = ["*"]
    
    # Rate limiting
    rate_limit_requests: int = 100
    rate_limit_window: int = 60  # seconds
    
    class Config:
        env_file = ".env"
        case_sensitive = False

settings = Settings() 