#!/bin/bash

# Quick start script for Spotify Clone

set -e

echo "🎵 Starting Spotify Clone..."

# Check if Docker is available
if command -v docker-compose &> /dev/null; then
    echo "🐳 Starting with Docker Compose..."
    docker-compose up --build
else
    echo "📋 Docker not found, starting manually..."
    
    # Check if backend is built
    if [ ! -f "backend/target/release/spotify-clone-backend" ]; then
        echo "🔨 Building backend..."
        cd backend
        cargo build --release
        cd ..
    fi
    
    # Check if frontend dependencies are installed
    if [ ! -d "frontend/node_modules" ]; then
        echo "📦 Installing frontend dependencies..."
        cd frontend
        npm install
        cd ..
    fi
    
    echo "🚀 Starting services..."
    echo "Starting backend on port 8080..."
    cd backend
    cargo run --release &
    BACKEND_PID=$!
    cd ..
    
    echo "Starting frontend on port 3000..."
    cd frontend
    npm start &
    FRONTEND_PID=$!
    cd ..
    
    echo "✅ Services started!"
    echo "Backend PID: $BACKEND_PID"
    echo "Frontend PID: $FRONTEND_PID"
    echo ""
    echo "🌐 Open http://localhost:3000 in your browser"
    echo ""
    echo "To stop the services:"
    echo "kill $BACKEND_PID $FRONTEND_PID"
    
    # Wait for user input to stop
    read -p "Press Enter to stop services..."
    kill $BACKEND_PID $FRONTEND_PID 2>/dev/null || true
fi
