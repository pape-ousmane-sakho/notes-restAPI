mod models;
mod handlers;

use axum::routing::get;
use axum::Router;
use sqlx::postgres::PgPoolOptions;
use handlers::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL not set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    tracing::info!("Database connected successfully");

    let app = Router::new()
        .route("/", get(|| async { "Notes API is running!" }))
        .route("/notes", get(get_notes).post(create_note))
        .route("/notes/:id", get(get_note).put(update_note).delete(delete_note))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    tracing::info!("Server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}