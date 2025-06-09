use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::models::*;
use crate::utils::{hash_password, verify_password, create_jwt_token};

pub struct AuthService;

impl AuthService {
    pub async fn register(
        pool: &PgPool,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<(String, User), String> {
        // Check if user already exists
        let existing_user = sqlx::query("SELECT id FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        if existing_user.is_some() {
            return Err("User with this email already exists".to_string());
        }

        let password_hash = hash_password(password)
            .map_err(|e| format!("Password hashing error: {}", e))?;

        let user_id = Uuid::new_v4();
        let now = chrono::Utc::now();

        let row = sqlx::query(
            "INSERT INTO users (id, username, email, password_hash, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6) RETURNING id, username, email, password_hash, created_at, updated_at"
        )
        .bind(user_id)
        .bind(username)
        .bind(email)
        .bind(password_hash)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Failed to create user: {}", e))?;

        let user = User {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            password_hash: row.get("password_hash"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        };

        let token = create_jwt_token(user.id)
            .map_err(|e| format!("Token creation error: {}", e))?;

        Ok((token, user))
    }

    pub async fn login(
        pool: &PgPool,
        email: &str,
        password: &str,
    ) -> Result<(String, User), String> {
        let row = sqlx::query("SELECT id, username, email, password_hash, created_at, updated_at FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?
            .ok_or_else(|| "Invalid credentials".to_string())?;

        let user = User {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            password_hash: row.get("password_hash"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        };

        if !verify_password(password, &user.password_hash)
            .map_err(|e| format!("Password verification error: {}", e))? {
            return Err("Invalid credentials".to_string());
        }

        let token = create_jwt_token(user.id)
            .map_err(|e| format!("Token creation error: {}", e))?;

        Ok((token, user))
    }
}

pub struct SongService;

impl SongService {
    pub async fn get_all_songs(pool: &PgPool) -> Result<Vec<Song>, String> {
        let rows = sqlx::query("SELECT id, title, artist, album, duration, file_path, cover_art, created_at FROM songs ORDER BY created_at DESC")
            .fetch_all(pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        let songs = rows.into_iter().map(|row| Song {
            id: row.get("id"),
            title: row.get("title"),
            artist: row.get("artist"),
            album: row.get("album"),
            duration: row.get("duration"),
            file_path: row.get("file_path"),
            cover_art: row.get("cover_art"),
            created_at: row.get("created_at"),
        }).collect();

        Ok(songs)
    }

    pub async fn get_song_by_id(pool: &PgPool, song_id: Uuid) -> Result<Option<Song>, String> {
        let row = sqlx::query("SELECT id, title, artist, album, duration, file_path, cover_art, created_at FROM songs WHERE id = $1")
            .bind(song_id)
            .fetch_optional(pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        Ok(row.map(|r| Song {
            id: r.get("id"),
            title: r.get("title"),
            artist: r.get("artist"),
            album: r.get("album"),
            duration: r.get("duration"),
            file_path: r.get("file_path"),
            cover_art: r.get("cover_art"),
            created_at: r.get("created_at"),
        }))
    }

    pub async fn search_songs(pool: &PgPool, query: &str) -> Result<Vec<Song>, String> {
        let search_pattern = format!("%{}%", query);
        let rows = sqlx::query("SELECT id, title, artist, album, duration, file_path, cover_art, created_at FROM songs WHERE title ILIKE $1 OR artist ILIKE $1 OR album ILIKE $1")
            .bind(&search_pattern)
            .fetch_all(pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        let songs = rows.into_iter().map(|row| Song {
            id: row.get("id"),
            title: row.get("title"),
            artist: row.get("artist"),
            album: row.get("album"),
            duration: row.get("duration"),
            file_path: row.get("file_path"),
            cover_art: row.get("cover_art"),
            created_at: row.get("created_at"),
        }).collect();

        Ok(songs)
    }
}

pub struct PlaylistService;

impl PlaylistService {
    pub async fn create_playlist(
        pool: &PgPool,
        user_id: Uuid,
        name: &str,
        description: Option<&str>,
    ) -> Result<Playlist, String> {
        let playlist_id = Uuid::new_v4();
        let now = chrono::Utc::now();

        let row = sqlx::query(
            "INSERT INTO playlists (id, name, user_id, description, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6) RETURNING id, name, user_id, description, cover_image, created_at, updated_at"
        )
        .bind(playlist_id)
        .bind(name)
        .bind(user_id)
        .bind(description)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;

        Ok(Playlist {
            id: row.get("id"),
            name: row.get("name"),
            user_id: row.get("user_id"),
            description: row.get("description"),
            cover_image: row.get("cover_image"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    pub async fn get_user_playlists(pool: &PgPool, user_id: Uuid) -> Result<Vec<Playlist>, String> {
        let rows = sqlx::query("SELECT id, name, user_id, description, cover_image, created_at, updated_at FROM playlists WHERE user_id = $1 ORDER BY created_at DESC")
            .bind(user_id)
            .fetch_all(pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        let playlists = rows.into_iter().map(|row| Playlist {
            id: row.get("id"),
            name: row.get("name"),
            user_id: row.get("user_id"),
            description: row.get("description"),
            cover_image: row.get("cover_image"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }).collect();

        Ok(playlists)
    }

    pub async fn get_playlist_with_songs(
        pool: &PgPool,
        playlist_id: Uuid,
    ) -> Result<Option<PlaylistWithSongs>, String> {
        let playlist_row = sqlx::query("SELECT id, name, user_id, description, cover_image, created_at, updated_at FROM playlists WHERE id = $1")
            .bind(playlist_id)
            .fetch_optional(pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        if let Some(row) = playlist_row {
            let playlist = Playlist {
                id: row.get("id"),
                name: row.get("name"),
                user_id: row.get("user_id"),
                description: row.get("description"),
                cover_image: row.get("cover_image"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };

            let song_rows = sqlx::query("SELECT s.id, s.title, s.artist, s.album, s.duration, s.file_path, s.cover_art, s.created_at FROM songs s JOIN playlist_songs ps ON s.id = ps.song_id WHERE ps.playlist_id = $1 ORDER BY ps.position")
                .bind(playlist_id)
                .fetch_all(pool)
                .await
                .map_err(|e| format!("Database error: {}", e))?;

            let songs = song_rows.into_iter().map(|row| Song {
                id: row.get("id"),
                title: row.get("title"),
                artist: row.get("artist"),
                album: row.get("album"),
                duration: row.get("duration"),
                file_path: row.get("file_path"),
                cover_art: row.get("cover_art"),
                created_at: row.get("created_at"),
            }).collect();

            Ok(Some(PlaylistWithSongs {
                id: playlist.id,
                name: playlist.name,
                user_id: playlist.user_id,
                description: playlist.description,
                cover_image: playlist.cover_image,
                created_at: playlist.created_at,
                updated_at: playlist.updated_at,
                songs,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn add_song_to_playlist(
        pool: &PgPool,
        playlist_id: Uuid,
        song_id: Uuid,
    ) -> Result<(), String> {
        // Get the next position
        let row = sqlx::query("SELECT COALESCE(MAX(position), 0) + 1 as next_position FROM playlist_songs WHERE playlist_id = $1")
            .bind(playlist_id)
            .fetch_one(pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        let next_position: i32 = row.get("next_position");

        sqlx::query("INSERT INTO playlist_songs (playlist_id, song_id, position, added_at) VALUES ($1, $2, $3, $4)")
            .bind(playlist_id)
            .bind(song_id)
            .bind(next_position)
            .bind(chrono::Utc::now())
            .execute(pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        Ok(())
    }
}
