use actix_multipart::Multipart;
use actix_web::{web, HttpResponse, Responder};
use futures_util::TryStreamExt;
use sqlx::PgPool;
use std::io::Write;
use uuid::Uuid;

use crate::config::Config;
use crate::services::SongService;

async fn get_all_songs(pool: web::Data<PgPool>) -> impl Responder {
    match SongService::get_all_songs(&pool).await {
        Ok(songs) => HttpResponse::Ok().json(songs),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e
        })),
    }
}

async fn get_song(pool: web::Data<PgPool>, path: web::Path<Uuid>) -> impl Responder {
    let song_id = path.into_inner();
    
    match SongService::get_song_by_id(&pool, song_id).await {
        Ok(Some(song)) => HttpResponse::Ok().json(song),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Song not found"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e
        })),
    }
}

async fn search_songs(
    pool: web::Data<PgPool>,
    query: web::Query<serde_json::Value>,
) -> impl Responder {
    let search_query = query
        .get("q")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    if search_query.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Search query is required"
        }));
    }

    match SongService::search_songs(&pool, search_query).await {
        Ok(songs) => HttpResponse::Ok().json(songs),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e
        })),
    }
}

async fn upload_song(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    mut payload: Multipart,
) -> impl Responder {
    let mut title = String::new();
    let mut artist = String::new();
    let mut album = String::new();
    let mut duration = 0i32;
    let mut file_path = String::new();

    while let Some(mut field) = payload.try_next().await.unwrap_or(None) {
        let content_disposition = field.content_disposition();
        let field_name = content_disposition.get_name().unwrap_or("");

        match field_name {
            "title" => {
                let mut data = Vec::new();
                while let Some(chunk) = field.try_next().await.unwrap_or(None) {
                    data.extend_from_slice(&chunk);
                }
                title = String::from_utf8(data).unwrap_or_default();
            }
            "artist" => {
                let mut data = Vec::new();
                while let Some(chunk) = field.try_next().await.unwrap_or(None) {
                    data.extend_from_slice(&chunk);
                }
                artist = String::from_utf8(data).unwrap_or_default();
            }
            "album" => {
                let mut data = Vec::new();
                while let Some(chunk) = field.try_next().await.unwrap_or(None) {
                    data.extend_from_slice(&chunk);
                }
                album = String::from_utf8(data).unwrap_or_default();
            }
            "duration" => {
                let mut data = Vec::new();
                while let Some(chunk) = field.try_next().await.unwrap_or(None) {
                    data.extend_from_slice(&chunk);
                }
                duration = String::from_utf8(data)
                    .unwrap_or_default()
                    .parse()
                    .unwrap_or(0);
            }
            "audio" => {
                let filename = content_disposition
                    .get_filename()
                    .unwrap_or("unknown.mp3");
                
                let file_id = Uuid::new_v4();
                let file_extension = std::path::Path::new(filename)
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .unwrap_or("mp3");
                
                let new_filename = format!("{}.{}", file_id, file_extension);
                file_path = format!("songs/{}", new_filename);
                
                let full_path = format!("{}/{}", config.upload_dir, file_path);
                
                // Create songs directory if it doesn't exist
                if let Some(parent) = std::path::Path::new(&full_path).parent() {
                    std::fs::create_dir_all(parent).ok();
                }

                let mut file = std::fs::File::create(&full_path).unwrap();
                
                while let Some(chunk) = field.try_next().await.unwrap_or(None) {
                    file.write_all(&chunk).unwrap();
                }
            }
            _ => {}
        }
    }

    if title.is_empty() || artist.is_empty() || file_path.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Missing required fields: title, artist, and audio file"
        }));
    }

    let song_id = Uuid::new_v4();
    let now = chrono::Utc::now();

    match sqlx::query!(
        "INSERT INTO songs (id, title, artist, album, duration, file_path, created_at) 
         VALUES ($1, $2, $3, $4, $5, $6, $7)",
        song_id,
        title,
        artist,
        album,
        duration,
        file_path,
        now
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            let song = crate::models::Song {
                id: song_id,
                title,
                artist,
                album,
                duration,
                file_path,
                cover_art: None,
                created_at: now,
            };
            HttpResponse::Created().json(song)
        }
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Failed to save song: {}", e)
        })),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/songs")
            .route("", web::get().to(get_all_songs))
            .route("/search", web::get().to(search_songs))
            .route("/upload", web::post().to(upload_song))
            .route("/{id}", web::get().to(get_song)),
    );
}
