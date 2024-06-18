use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Log {
    pub id: i32,
    pub message: String,
    pub created: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct LogPage {
    pub logs: Vec<Log>,
    pub total_pages: i64,
}

#[derive(Serialize, Deserialize)]
pub struct NewLog {
    pub message: String,
}
