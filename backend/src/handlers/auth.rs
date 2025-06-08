use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::models::{AuthResponse, LoginRequest, RegisterRequest};
use crate::services::AuthService;

async fn register(
    pool: web::Data<PgPool>,
    register_data: web::Json<RegisterRequest>,
) -> impl Responder {
    match AuthService::register(
        &pool,
        &register_data.username,
        &register_data.email,
        &register_data.password,
    )
    .await
    {
        Ok((token, user)) => HttpResponse::Created().json(AuthResponse {
            token,
            user: user.into(),
        }),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": e
        })),
    }
}

async fn login(
    pool: web::Data<PgPool>,
    login_data: web::Json<LoginRequest>,
) -> impl Responder {
    match AuthService::login(&pool, &login_data.email, &login_data.password).await {
        Ok((token, user)) => HttpResponse::Ok().json(AuthResponse {
            token,
            user: user.into(),
        }),
        Err(e) => HttpResponse::Unauthorized().json(serde_json::json!({
            "error": e
        })),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login)),
    );
}
