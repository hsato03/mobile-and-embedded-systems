use serde::{Deserialize, Serialize};

#[derive(Serialize, sqlx::FromRow)]
pub struct Temperature {
    pub id: i32,
    pub degrees: f64,
}

#[derive(Deserialize)]
pub struct NewTemperature {
    pub degrees: f64,
}
