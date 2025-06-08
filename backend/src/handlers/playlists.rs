use actix_web::{web, HttpResponse, Responder, HttpRequest, HttpMessage};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{CreatePlaylistRequest, AddSongToPlaylistRequest};
use crate::services::PlaylistService;

// Extract user_id from Authorization header
fn get_user_id_from_request(req: &HttpRequest) -> Result<Uuid, String> {
    let auth_header = req.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| "Missing Authorization header".to_string())?;

    if !auth_header.starts_with("Bearer ") {
        return Err("Invalid Authorization header format".to_string());
    }

    let token = &auth_header[7..]; // Remove "Bearer " prefix
    crate::middleware::validate_jwt(token)
}

async fn create_playlist(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    playlist_data: web::Json<CreatePlaylistRequest>,
) -> impl Responder {
    let user_id = match get_user_id_from_request(&req) {
        Ok(id) => id,
        Err(e) => return HttpResponse::Unauthorized().json(serde_json::json!({"error": e})),
    };

    match PlaylistService::create_playlist(
        &pool,
        user_id,
        &playlist_data.name,
        playlist_data.description.as_deref(),
    )
    .await
    {
        Ok(playlist) => HttpResponse::Created().json(playlist),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e
        })),
    }
}

async fn get_user_playlists(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let user_id = match get_user_id_from_request(&req) {
        Ok(id) => id,
        Err(e) => return HttpResponse::Unauthorized().json(serde_json::json!({"error": e})),
    };

    match PlaylistService::get_user_playlists(&pool, user_id).await {
        Ok(playlists) => HttpResponse::Ok().json(playlists),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e
        })),
    }
}

async fn get_playlist(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let playlist_id = path.into_inner();

    match PlaylistService::get_playlist_with_songs(&pool, playlist_id).await {
        Ok(Some(playlist)) => HttpResponse::Ok().json(playlist),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Playlist not found"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e
        })),
    }
}

async fn add_song_to_playlist(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    song_data: web::Json<AddSongToPlaylistRequest>,
) -> impl Responder {
    let playlist_id = path.into_inner();

    match PlaylistService::add_song_to_playlist(&pool, playlist_id, song_data.song_id).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Song added to playlist successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e
        })),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/playlists")
            .route("", web::post().to(create_playlist))
            .route("", web::get().to(get_user_playlists))
            .route("/{id}", web::get().to(get_playlist))
            .route("/{id}/songs", web::post().to(add_song_to_playlist)),
    );
}
