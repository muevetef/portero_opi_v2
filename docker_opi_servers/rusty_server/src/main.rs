use chrono::{DateTime, Utc};

mod web_server;
mod frame_receiver;
mod qr_scanner;

pub struct Frame {
    pub data: Vec<u8>,
    pub timestamp: DateTime<Utc>
}

pub struct QR {
    pub code: String,
    pub timestamp: DateTime<Utc>
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .pretty()
        .init();

    let (frame_sx, frame_rx) = crossbeam_channel::unbounded();
    let (qr_sx, qr_rx) = crossbeam_channel::unbounded();

    let tasks = vec![
        tokio::spawn(web_server::run(frame_rx.clone(), qr_rx.clone())),
        tokio::spawn(qr_scanner::run(frame_rx.clone(), qr_sx.clone())),
        tokio::spawn(frame_receiver::run(frame_sx.clone()))
    ];

    for task in tasks {
        task.await?;
    }

    Ok(())
}