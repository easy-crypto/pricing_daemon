use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Price {
    #[serde(rename = "_id")]
    pub id: chrono::DateTime<chrono::Utc>,
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
    pub vwap: f64,
    pub volume: f64,
    pub count: u32,
}
