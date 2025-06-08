# Spotify Clone

A full-stack music streaming application built with Rust (backend) and React (frontend).

## Features

- 🎵 Music streaming and playback
- 👤 User authentication (register/login)
- 🎧 Create and manage playlists
- 🔍 Search songs by title, artist, or album
- 📱 Responsive design
- 🎨 Spotify-inspired UI
- 📤 Upload your own music files

## Tech Stack

### Backend (Rust)
- **Actix-web** - Web framework
- **SQLx** - Database toolkit
- **PostgreSQL** - Database
- **JWT** - Authentication
- **Bcrypt** - Password hashing

### Frontend (React)
- **React 18** - UI framework
- **React Router** - Client-side routing
- **Axios** - HTTP client
- **React Icons** - Icon library
- **CSS3** - Styling

## Prerequisites

- **Rust** (1.70+)
- **Node.js** (18+)
- **PostgreSQL** (13+)
- **Docker & Docker Compose** (optional)

## Quick Start with Docker

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd spotify-clone
   ```

2. **Start with Docker Compose**
   ```bash
   docker-compose up --build
   ```

3. **Access the application**
   - Frontend: http://localhost:3000
   - Backend API: http://localhost:8080

## Manual Setup

### Database Setup

1. **Install PostgreSQL** and create a database:
   ```sql
   CREATE DATABASE spotify_clone;
   ```

2. **Set environment variables** (create `.env` in backend folder):
   ```env
   DATABASE_URL=postgres://postgres:password@localhost:5432/spotify_clone
   JWT_SECRET=your_super_secret_jwt_key_here
   RUST_LOG=debug
   HOST=127.0.0.1
   PORT=8080
   UPLOAD_DIR=./uploads
   ```

### Backend Setup

1. **Navigate to backend directory**
   ```bash
   cd backend
   ```

2. **Install SQLx CLI** (for migrations)
   ```bash
   cargo install sqlx-cli --no-default-features --features postgres
   ```

3. **Run database migrations**
   ```bash
   sqlx migrate run
   ```

4. **Build and run the backend**
   ```bash
   cargo build --release
   cargo run
   ```

   The backend will start on http://localhost:8080

### Frontend Setup

1. **Navigate to frontend directory**
   ```bash
   cd frontend
   ```

2. **Install dependencies**
   ```bash
   npm install
   ```

3. **Start the development server**
   ```bash
   npm start
   ```

   The frontend will start on http://localhost:3000

## Build Commands

### Backend
```bash
cd backend

# Development
cargo run

# Release build
cargo build --release

# Run tests
cargo test

# Check code
cargo check
```

### Frontend
```bash
cd frontend

# Development server
npm start

# Production build
npm run build

# Run tests
npm test

# Lint code
npm run lint
```

## API Endpoints

### Authentication
- `POST /api/auth/register` - Register new user
- `POST /api/auth/login` - Login user

### Songs
- `GET /api/songs` - Get all songs
- `GET /api/songs/{id}` - Get song by ID
- `GET /api/songs/search?q={query}` - Search songs
- `POST /api/songs/upload` - Upload new song (multipart/form-data)

### Playlists
- `GET /api/playlists` - Get user playlists
- `POST /api/playlists` - Create new playlist
- `GET /api/playlists/{id}` - Get playlist with songs
- `POST /api/playlists/{id}/songs` - Add song to playlist

### Users
- `GET /api/users/me` - Get current user info

## Project Structure

```
spotify-clone/
├── backend/                 # Rust backend
│   ├── src/
│   │   ├── handlers/       # API route handlers
│   │   ├── models.rs       # Data models
│   │   ├── services.rs     # Business logic
│   │   ├── config.rs       # Configuration
│   │   ├── middleware.rs   # JWT middleware
│   │   ├── utils.rs        # Utility functions
│   │   └── main.rs         # Application entry point
│   ├── migrations/         # Database migrations
│   ├── Cargo.toml         # Rust dependencies
│   └── Dockerfile         # Backend Docker config
├── frontend/               # React frontend
│   ├── src/
│   │   ├── components/    # React components
│   │   ├── pages/         # Page components
│   │   ├── services/      # API services
│   │   ├── context/       # React context
│   │   ├── styles/        # CSS files
│   │   └── App.js         # Main App component
│   ├── package.json       # Node.js dependencies
│   └── Dockerfile         # Frontend Docker config
└── docker-compose.yml     # Docker Compose config
```

## Usage

1. **Register/Login** - Create an account or login
2. **Upload Music** - Go to Library and upload your music files
3. **Create Playlists** - Organize your music into playlists
4. **Search** - Find songs by title, artist, or album
5. **Play Music** - Click on any song to start playing

## Development

### Adding New Features

1. **Backend**: Add new handlers in `src/handlers/`, update models in `src/models.rs`
2. **Frontend**: Create new components in `src/components/`, add pages in `src/pages/`
3. **Database**: Create new migrations with `sqlx migrate add <name>`

### Environment Variables

Backend `.env` file:
```env
DATABASE_URL=postgres://username:password@localhost:5432/spotify_clone
JWT_SECRET=your-secret-key
RUST_LOG=debug
HOST=127.0.0.1
PORT=8080
UPLOAD_DIR=./uploads
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is for educational purposes. Please respect music copyrights when using this application.

## Troubleshooting

### Common Issues

1. **Database connection failed**
   - Ensure PostgreSQL is running
   - Check DATABASE_URL in .env file
   - Verify database exists

2. **Frontend can't connect to backend**
   - Ensure backend is running on port 8080
   - Check CORS configuration
   - Verify API endpoints

3. **File upload fails**
   - Check UPLOAD_DIR permissions
   - Ensure directory exists
   - Verify file size limits

### Getting Help

- Check the logs for error messages
- Ensure all dependencies are installed
- Verify environment variables are set correctly
