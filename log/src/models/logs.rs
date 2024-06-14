use chrono;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Log {
    pub id: i32,
    pub message: String,
    pub created: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct NewLog {
    pub message: String,
}
