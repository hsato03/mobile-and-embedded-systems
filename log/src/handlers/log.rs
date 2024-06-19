use axum::{
    extract::{Extension, Query},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;

use crate::models::logs::{Log, LogPage, NewLog};

#[derive(Deserialize)]
pub struct PaginationParams {
    page: Option<u32>,
    page_size: Option<u32>,
}

pub async fn create_log(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<NewLog>,
) -> Result<Json<Option<Log>>, (axum::http::StatusCode, String)> {
    let result = sqlx::query_as(
        "
        INSERT INTO log (message, created)
        VALUES ($1, $2)
        RETURNING id, message, created
        ",
    )
    .bind(payload.message)
    .bind(Utc::now().naive_utc())
    .fetch_optional(&pool)
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
    Query(params): Query<PaginationParams>,
) -> Result<Json<LogPage>, (axum::http::StatusCode, String)> {
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);
    let offset = (page - 1) * page_size;
    let mut total_count = 1;
    let mut logs: Vec<Log> = vec![];

    let count: Result<i64, (StatusCode, String)> =
        match sqlx::query_scalar("SELECT COUNT(*) FROM log")
            .fetch_one(&pool)
            .await
        {
            Ok(log) => Ok(log),
            Err(err) => {
                let message = format!("Failed to fetch log: {}", err);
                Err((StatusCode::INTERNAL_SERVER_ERROR, message))
            }
        };

    if let Ok(page_count) = count {
        total_count = page_count;
    }

    let total_pages_calc = (total_count as f64 / page_size as f64) as i64;
    let total_pages = if total_pages_calc > 0 {
        total_pages_calc
    } else {
        1
    };

    let query = format!(
        "SELECT id, message, created FROM log ORDER BY id DESC LIMIT {} OFFSET {}",
        page_size, offset
    );

    let result = sqlx::query_as(&query).fetch_all(&pool).await;
    if let Ok(logs_page) = result {
        logs = logs_page;
    }

    let response = LogPage { logs, total_pages };

    Ok(Json(response))
}
