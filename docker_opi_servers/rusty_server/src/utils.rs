use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub struct Frame {
    pub data: Vec<u8>,
    pub timestamp: DateTime<Utc>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QR {
    pub code: String,
    pub timestamp: DateTime<Utc>,
    pub points: Vec<Point<i32>>,
    pub frame_size: Point<i32>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Point<T> {
    pub x: T,
    pub y: T
}

pub enum EspMessage {
    Open
}