use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Temperature {
    pub id: i32,
    pub degrees: f64,
}

#[derive(Serialize, Deserialize)]
pub struct NewTemperature {
    pub degrees: f64,
}
