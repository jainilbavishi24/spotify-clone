mod config;
mod models;
mod handlers;
mod middleware;
mod services;
mod utils;

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::fs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let config = config::Config::from_env();
    let host = config.host.clone();
    let port = config.port;
    let upload_dir = config.upload_dir.clone();

    // Create upload directory if it doesn't exist
    fs::create_dir_all(&upload_dir).expect("Failed to create upload directory");

    // Database connection
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .expect("Failed to create database pool");

    // Run migrations
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    log::info!("Starting server at http://{}:{}", host, port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(config.clone()))
            .wrap(Logger::default())
            .wrap(cors)
            .service(
                web::scope("/api")
                    .configure(handlers::auth::configure)
                    .configure(handlers::songs::configure)
                    .configure(handlers::playlists::configure)
                    .configure(handlers::users::configure)
            )
            .service(Files::new("/uploads", &upload_dir))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
