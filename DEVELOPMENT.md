# Development Guide

## Complete Build and Run Instructions

### Prerequisites Installation

#### 1. Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

#### 2. Install Node.js
```bash
# Using nvm (recommended)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install 18
nvm use 18

# Or download from https://nodejs.org/
```

#### 3. Install PostgreSQL
```bash
# Ubuntu/Debian
sudo apt update
sudo apt install postgresql postgresql-contrib

# macOS
brew install postgresql
brew services start postgresql

# Windows
# Download from https://www.postgresql.org/download/windows/
```

#### 4. Install SQLx CLI
```bash
cargo install sqlx-cli --no-default-features --features postgres
```

### Database Setup

1. **Start PostgreSQL service**
```bash
# Ubuntu/Debian
sudo systemctl start postgresql

# macOS
brew services start postgresql
```

2. **Create database and user**
```bash
sudo -u postgres psql
```

```sql
CREATE DATABASE spotify_clone;
CREATE USER spotify_user WITH PASSWORD 'your_password';
GRANT ALL PRIVILEGES ON DATABASE spotify_clone TO spotify_user;
\q
```

### Backend Setup

1. **Navigate to backend directory**
```bash
cd backend
```

2. **Create environment file**
```bash
cp .env.example .env
# Edit .env with your database credentials
```

3. **Run database migrations**
```bash
sqlx migrate run
```

4. **Build and run**
```bash
# Development
cargo run

# Production build
cargo build --release
cargo run --release
```

### Frontend Setup

1. **Navigate to frontend directory**
```bash
cd frontend
```

2. **Install dependencies**
```bash
npm install
```

3. **Start development server**
```bash
npm start
```

## Docker Development

### Using Docker Compose (Recommended)

1. **Start all services**
```bash
docker-compose up --build
```

2. **Stop services**
```bash
docker-compose down
```

3. **View logs**
```bash
docker-compose logs -f backend
docker-compose logs -f frontend
```

### Individual Docker Builds

#### Backend
```bash
cd backend
docker build -t spotify-backend .
docker run -p 8080:8080 --env-file .env spotify-backend
```

#### Frontend
```bash
cd frontend
docker build -t spotify-frontend .
docker run -p 3000:3000 spotify-frontend
```

## Development Workflow

### 1. Making Changes

#### Backend Changes
- Modify Rust code in `src/`
- Add new migrations in `migrations/`
- Update models in `src/models.rs`
- Add new API endpoints in `src/handlers/`

#### Frontend Changes
- Modify React components in `src/components/`
- Add new pages in `src/pages/`
- Update styles in `src/styles/`
- Add new services in `src/services/`

### 2. Testing

#### Backend Tests
```bash
cd backend
cargo test
```

#### Frontend Tests
```bash
cd frontend
npm test
```

### 3. Code Quality

#### Backend
```bash
cd backend
cargo fmt        # Format code
cargo clippy     # Lint code
cargo check      # Check compilation
```

#### Frontend
```bash
cd frontend
npm run lint     # Lint code (if configured)
npm run format   # Format code (if configured)
```

## Database Management

### Creating Migrations
```bash
cd backend
sqlx migrate add create_new_table
# Edit the generated migration file
sqlx migrate run
```

### Reverting Migrations
```bash
sqlx migrate revert
```

### Database Reset
```bash
sqlx database drop
sqlx database create
sqlx migrate run
```

## API Testing

### Using curl

#### Register User
```bash
curl -X POST http://localhost:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username":"testuser","email":"test@example.com","password":"password123"}'
```

#### Login
```bash
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"password123"}'
```

#### Get Songs (with auth)
```bash
curl -X GET http://localhost:8080/api/songs \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

### Using Postman
Import the API collection (create one based on the endpoints in README.md)

## Troubleshooting

### Common Backend Issues

1. **Database connection failed**
```bash
# Check if PostgreSQL is running
sudo systemctl status postgresql

# Check database exists
psql -U postgres -l
```

2. **Migration errors**
```bash
# Reset database
sqlx database drop
sqlx database create
sqlx migrate run
```

3. **Compilation errors**
```bash
# Clean build
cargo clean
cargo build
```

### Common Frontend Issues

1. **Dependencies issues**
```bash
# Clear cache and reinstall
rm -rf node_modules package-lock.json
npm install
```

2. **Build errors**
```bash
# Clear build cache
rm -rf build
npm run build
```

### Docker Issues

1. **Port conflicts**
```bash
# Check what's using the port
lsof -i :8080
lsof -i :3000

# Kill processes if needed
sudo kill -9 PID
```

2. **Volume issues**
```bash
# Remove volumes
docker-compose down -v
docker volume prune
```

## Performance Optimization

### Backend
- Use connection pooling (already configured)
- Add database indexes for frequently queried fields
- Implement caching for static data
- Use async/await properly

### Frontend
- Implement lazy loading for components
- Optimize images and audio files
- Use React.memo for expensive components
- Implement virtual scrolling for large lists

## Security Considerations

### Backend
- Validate all input data
- Use parameterized queries (SQLx handles this)
- Implement rate limiting
- Use HTTPS in production
- Rotate JWT secrets regularly

### Frontend
- Sanitize user input
- Implement proper error handling
- Don't store sensitive data in localStorage
- Use environment variables for configuration

## Deployment

### Production Environment Variables

#### Backend (.env)
```env
DATABASE_URL=postgres://user:pass@host:5432/db
JWT_SECRET=very_long_random_string
RUST_LOG=info
HOST=0.0.0.0
PORT=8080
UPLOAD_DIR=/app/uploads
```

#### Frontend
```env
REACT_APP_API_URL=https://your-api-domain.com
```

### Build for Production

#### Backend
```bash
cargo build --release
```

#### Frontend
```bash
npm run build
```

### Docker Production
```bash
docker-compose -f docker-compose.prod.yml up --build
```

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Make your changes
4. Add tests if applicable
5. Commit your changes: `git commit -am 'Add feature'`
6. Push to the branch: `git push origin feature-name`
7. Submit a pull request

## Additional Resources

- [Rust Documentation](https://doc.rust-lang.org/)
- [Actix-web Guide](https://actix.rs/)
- [React Documentation](https://reactjs.org/docs/)
- [PostgreSQL Documentation](https://www.postgresql.org/docs/)
- [SQLx Documentation](https://docs.rs/sqlx/)
