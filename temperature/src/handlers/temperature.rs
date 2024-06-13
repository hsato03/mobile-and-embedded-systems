use axum::{extract::Extension, Json};
use sqlx::PgPool;

use crate::models::temperatures::{NewTemperature, Temperature};

pub async fn create_temperature(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<NewTemperature>,
) -> Result<Json<Temperature>, (axum::http::StatusCode, String)> {
    let result = sqlx::query_as!(
        Temperature,
        "
        INSERT INTO temperature (degrees)
        VALUES ($1)
        RETURNING id, degrees
        ",
        payload.degrees
    )
    .fetch_one(&pool)
    .await;

    match result {
        Ok(temperature) => Ok(Json(temperature)),
        Err(e) => Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to insert temperature: {}", e),
        )),
    }
}

pub async fn get_temperature(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Option<Temperature>>, (axum::http::StatusCode, String)> {
    let result = sqlx::query_as!(Temperature, r#"SELECT id, degrees FROM temperature;"#)
        .fetch_optional(&pool)
        .await;

    match result {
        Ok(temperature) => Ok(Json(temperature)),
        Err(e) => Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to fetch temperature: {}", e),
        )),
    }
}

pub async fn update_temperature(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<NewTemperature>,
) -> Result<Json<Temperature>, (axum::http::StatusCode, String)> {
    let result = sqlx::query_as!(
        Temperature,
        "UPDATE temperature SET degrees = $1 RETURNING id, degrees",
        payload.degrees
    )
    .fetch_one(&pool)
    .await;

    match result {
        Ok(temperature) => Ok(Json(temperature)),
        Err(e) => Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to fetch temperature {}", e),
        )),
    }
}
