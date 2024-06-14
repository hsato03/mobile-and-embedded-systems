use axum::{extract::Extension, Json};
use chrono::Utc;
use sqlx::PgPool;

use crate::models::logs::{Log, NewLog};

pub async fn create_log(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<NewLog>,
) -> Result<Json<Log>, (axum::http::StatusCode, String)> {
    let result = sqlx::query_as!(
        Log,
        "
        INSERT INTO log (message, created)
        VALUES ($1, $2)
        RETURNING id, message, created
        ",
        payload.message,
        Utc::now().naive_utc()
    )
    .fetch_one(&pool)
    .await;

    match result {
        Ok(log) => Ok(Json(log)),
        Err(e) => Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to insert log: {}", e),
        )),
    }
}

pub async fn get_logs(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<Log>>, (axum::http::StatusCode, String)> {
    let result = sqlx::query_as!(Log, r#"SELECT id, message, created FROM log"#)
        .fetch_all(&pool)
        .await;

    match result {
        Ok(temperature) => Ok(Json(temperature)),
        Err(e) => Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to fetch temperature: {}", e),
        )),
    }
}
