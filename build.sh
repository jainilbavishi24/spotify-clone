#!/bin/bash

# Spotify Clone Build Script

set -e

echo "🎵 Building Spotify Clone..."

# Build backend
echo "🦀 Building Rust backend..."
cd backend
cargo build --release
echo "✅ Backend built successfully"
cd ..

# Build frontend
echo "⚛️ Building React frontend..."
cd frontend
npm run build
echo "✅ Frontend built successfully"
cd ..

echo "🎉 Build complete!"
echo ""
echo "🚀 To run the application:"
echo "Backend: cd backend && cargo run --release"
echo "Frontend: cd frontend && npm start"
echo ""
echo "🐳 Or use Docker: docker-compose up --build"
