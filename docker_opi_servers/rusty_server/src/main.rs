use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::{info, error, Level};

mod web_server;
mod frame_receiver;
mod qr_scanner;

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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .pretty()
        .with_max_level(Level::DEBUG)
        .init();

    let (frame_tx, _) = tokio::sync::broadcast::channel(64);
    let (qr_tx, _) = tokio::sync::broadcast::channel(64);

    let tasks = vec![
        tokio::spawn(web_server::run(frame_tx.clone(), qr_tx.clone())),
        tokio::spawn(qr_scanner::run(frame_tx.subscribe(), qr_tx.clone())),
        tokio::spawn(frame_receiver::run(frame_tx.clone()))
    ];

    match futures::future::try_join_all(tasks).await {
        Ok(_) => {
            info!("All tasks executed successfully, exiting.");
        },
        Err(err) => {
            error!("Unexpected error executing task(s): {err}");
        },
    }

    Ok(())
}