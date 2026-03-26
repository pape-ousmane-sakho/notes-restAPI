use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;
use axum::Router;
use axum::routing::get;

use crate::models::{CreateNote, Note};

pub async fn get_notes(State(pool): State<PgPool>) -> impl IntoResponse {
    let notes = sqlx::query_as!(
        Note,
        "SELECT id, title, content, created_at FROM notes ORDER BY created_at DESC"
    )
    .fetch_all(&pool)
    .await;

    match notes {
        Ok(notes) => (StatusCode::OK, Json(notes)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        ).into_response(),
    }
}


pub async fn create_note(
    State(pool): State<PgPool>,
    Json(body): Json<CreateNote>,
) -> impl IntoResponse {
    let note = sqlx::query_as!(
        Note,
        "INSERT INTO notes (title, content) VALUES ($1, $2) 
         RETURNING id, title, content, created_at",
        body.title,
        body.content
    )
    .fetch_one(&pool)
    .await;

    match note {
        Ok(note) => (StatusCode::CREATED, Json(note)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        ).into_response(),
    }
}
pub async fn get_note(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let note = sqlx::query_as!(
        Note,
        "SELECT id, title, content, created_at FROM notes WHERE id = $1",
        id as Uuid
    )
    .fetch_one(&pool)
    .await;

    match note {
        Ok(note) => (StatusCode::OK, Json(note)).into_response(),
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "Note not found" })),
        ).into_response(),
    }
}

pub async fn update_note(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(body): Json<CreateNote>, // needs a body with new title and content
) -> impl IntoResponse {
    let note = sqlx::query_as!(
        Note,
        "UPDATE notes SET title = $1, content = $2 WHERE id = $3 
         RETURNING id, title, content, created_at",
        body.title,
        body.content,
        id as Uuid
    )
    .fetch_one(&pool)
    .await;

    match note {
        Ok(note) => (StatusCode::OK, Json(note)).into_response(),
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "Note not found" })),
        ).into_response(),
    }
}

pub async fn delete_note(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let result = sqlx::query!(
        "DELETE FROM notes WHERE id = $1",
        id as Uuid
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => (
            StatusCode::OK,
            Json(serde_json::json!({ "message": "Note deleted successfully" })),
        ).into_response(),
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "Note not found" })),
        ).into_response(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use serde_json::json;
    use sqlx::postgres::PgPoolOptions;
    use tower::ServiceExt;

    async fn get_test_pool() -> PgPool {
        dotenv::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL not set");
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .expect("Failed to connect to test database")
    }

    fn get_test_router(pool: PgPool) -> Router {
        Router::new()
            .route("/notes", get(get_notes).post(create_note))
            .route("/notes/:id", get(get_note).put(update_note).delete(delete_note))
            .with_state(pool)
    }

    #[tokio::test]
    async fn test_get_notes_returns_200() {
        let pool = get_test_pool().await;
        let app = get_test_router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/notes")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_create_note_returns_201() {
        let pool = get_test_pool().await;
        let app = get_test_router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/notes")
                    .header("Content-Type", "application/json")
                    .body(Body::from(
                        json!({
                            "title": "Test note",
                            "content": "Test content"
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }
}