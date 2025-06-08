use actix_web::{web, HttpResponse, Responder, HttpRequest, HttpMessage};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::UserResponse;

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

async fn get_current_user(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let user_id = match get_user_id_from_request(&req) {
        Ok(id) => id,
        Err(e) => return HttpResponse::Unauthorized().json(serde_json::json!({"error": e})),
    };

    match sqlx::query_as!(
        crate::models::User,
        "SELECT * FROM users WHERE id = $1",
        user_id
    )
    .fetch_optional(pool.get_ref())
    .await
    {
        Ok(Some(user)) => {
            let user_response: UserResponse = user.into();
            HttpResponse::Ok().json(user_response)
        }
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "User not found"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database error: {}", e)
        })),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/me", web::get().to(get_current_user)),
    );
}
