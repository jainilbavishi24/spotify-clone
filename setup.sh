#!/bin/bash

# Spotify Clone Setup Script

set -e

echo "🎵 Setting up Spotify Clone..."

# Check if required tools are installed
check_command() {
    if ! command -v $1 &> /dev/null; then
        echo "❌ $1 is not installed. Please install it first."
        exit 1
    fi
}

echo "📋 Checking prerequisites..."
check_command "cargo"
check_command "node"
check_command "npm"
check_command "psql"

# Setup database
echo "🗄️ Setting up database..."
read -p "Enter PostgreSQL username (default: postgres): " PG_USER
PG_USER=${PG_USER:-postgres}

read -s -p "Enter PostgreSQL password: " PG_PASSWORD
echo

read -p "Enter database name (default: spotify_clone): " DB_NAME
DB_NAME=${DB_NAME:-spotify_clone}

# Create database if it doesn't exist
echo "Creating database if it doesn't exist..."
PGPASSWORD=$PG_PASSWORD psql -h localhost -U $PG_USER -tc "SELECT 1 FROM pg_database WHERE datname = '$DB_NAME'" | grep -q 1 || PGPASSWORD=$PG_PASSWORD psql -h localhost -U $PG_USER -c "CREATE DATABASE $DB_NAME"

# Setup backend
echo "🦀 Setting up Rust backend..."
cd backend

# Create .env file
cat > .env << EOF
DATABASE_URL=postgres://$PG_USER:$PG_PASSWORD@localhost:5432/$DB_NAME
JWT_SECRET=$(openssl rand -base64 32)
RUST_LOG=debug
HOST=127.0.0.1
PORT=8080
UPLOAD_DIR=./uploads
EOF

echo "✅ Created backend .env file"

# Install sqlx-cli if not present
if ! command -v sqlx &> /dev/null; then
    echo "📦 Installing sqlx-cli..."
    cargo install sqlx-cli --no-default-features --features postgres
fi

# Run migrations
echo "🔄 Running database migrations..."
sqlx migrate run

# Build backend
echo "🔨 Building backend..."
cargo build --release

cd ..

# Setup frontend
echo "⚛️ Setting up React frontend..."
cd frontend

# Install dependencies
echo "📦 Installing frontend dependencies..."
npm install

cd ..

echo "✅ Setup complete!"
echo ""
echo "🚀 To start the application:"
echo "1. Start the backend:"
echo "   cd backend && cargo run"
echo ""
echo "2. In another terminal, start the frontend:"
echo "   cd frontend && npm start"
echo ""
echo "3. Open http://localhost:3000 in your browser"
echo ""
echo "🐳 Or use Docker:"
echo "   docker-compose up --build"
