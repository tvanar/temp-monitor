use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Stats {
    pub hostname: String,
    pub cpu_usage: f32,
    pub cpu_freq: u64,
    pub ram_usage: u64,
    pub ram_total: u64,
    pub temp: f32,
}

#[derive(Clone, Serialize)]

pub struct TempPoint {
    pub time: DateTime<Utc>,
    pub temp: f32,
}
