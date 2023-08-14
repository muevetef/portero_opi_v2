use tracing::{info, error, Level};

mod web_server;
mod frame_receiver;
mod qr_scanner;
mod esp_comm;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .pretty()
        .with_max_level(Level::DEBUG)
        .init();

    let (frame_tx, _) = tokio::sync::broadcast::channel(64);
    let (qr_tx, _) = tokio::sync::broadcast::channel(64);
    let (esp_msg_tx, esp_msg_rx) = tokio::sync::mpsc::channel(64);

    let tasks = vec![
        tokio::spawn(web_server::run(frame_tx.clone(), qr_tx.clone(), esp_msg_tx.clone())),
        tokio::spawn(qr_scanner::run(frame_tx.subscribe(), qr_tx.clone())),
        tokio::spawn(frame_receiver::run(frame_tx.clone())),
        tokio::spawn(esp_comm::run(esp_msg_rx))
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