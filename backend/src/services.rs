use sqlx::PgPool;
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
        let existing_user = sqlx::query!(
            "SELECT id FROM users WHERE email = $1",
            email
        )
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

        let user = sqlx::query_as!(
            User,
            "INSERT INTO users (id, username, email, password_hash, created_at, updated_at) 
             VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
            user_id,
            username,
            email,
            password_hash,
            now,
            now
        )
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Failed to create user: {}", e))?;

        let token = create_jwt_token(user.id)
            .map_err(|e| format!("Token creation error: {}", e))?;

        Ok((token, user))
    }

    pub async fn login(
        pool: &PgPool,
        email: &str,
        password: &str,
    ) -> Result<(String, User), String> {
        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE email = $1",
            email
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or_else(|| "Invalid credentials".to_string())?;

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
        sqlx::query_as!(Song, "SELECT * FROM songs ORDER BY created_at DESC")
            .fetch_all(pool)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    pub async fn get_song_by_id(pool: &PgPool, song_id: Uuid) -> Result<Option<Song>, String> {
        sqlx::query_as!(Song, "SELECT * FROM songs WHERE id = $1", song_id)
            .fetch_optional(pool)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    pub async fn search_songs(pool: &PgPool, query: &str) -> Result<Vec<Song>, String> {
        let search_pattern = format!("%{}%", query);
        sqlx::query_as!(
            Song,
            "SELECT * FROM songs WHERE title ILIKE $1 OR artist ILIKE $1 OR album ILIKE $1",
            search_pattern
        )
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Database error: {}", e))
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

        sqlx::query_as!(
            Playlist,
            "INSERT INTO playlists (id, name, user_id, description, created_at, updated_at) 
             VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
            playlist_id,
            name,
            user_id,
            description,
            now,
            now
        )
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Database error: {}", e))
    }

    pub async fn get_user_playlists(pool: &PgPool, user_id: Uuid) -> Result<Vec<Playlist>, String> {
        sqlx::query_as!(
            Playlist,
            "SELECT * FROM playlists WHERE user_id = $1 ORDER BY created_at DESC",
            user_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Database error: {}", e))
    }

    pub async fn get_playlist_with_songs(
        pool: &PgPool,
        playlist_id: Uuid,
    ) -> Result<Option<PlaylistWithSongs>, String> {
        let playlist = sqlx::query_as!(
            Playlist,
            "SELECT * FROM playlists WHERE id = $1",
            playlist_id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;

        if let Some(playlist) = playlist {
            let songs = sqlx::query_as!(
                Song,
                "SELECT s.* FROM songs s 
                 JOIN playlist_songs ps ON s.id = ps.song_id 
                 WHERE ps.playlist_id = $1 
                 ORDER BY ps.position",
                playlist_id
            )
            .fetch_all(pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

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
        let next_position = sqlx::query!(
            "SELECT COALESCE(MAX(position), 0) + 1 as next_position FROM playlist_songs WHERE playlist_id = $1",
            playlist_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?
        .next_position
        .unwrap_or(1);

        sqlx::query!(
            "INSERT INTO playlist_songs (playlist_id, song_id, position, added_at) 
             VALUES ($1, $2, $3, $4)",
            playlist_id,
            song_id,
            next_position,
            chrono::Utc::now()
        )
        .execute(pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;

        Ok(())
    }
}
