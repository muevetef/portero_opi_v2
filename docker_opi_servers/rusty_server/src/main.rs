use std::time::Duration;

use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database};
use tracing::{error, info, Level, log};

mod esp_comm;
mod frame_receiver;
mod qr_scanner;
mod utils;
mod web_server;
mod entities;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .pretty()
        .with_max_level(Level::DEBUG)
        .init();

    let mut db_opt = ConnectOptions::new("postgres://postgres:root@localhost/portero");
    db_opt.min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info)
        .set_schema_search_path("public");

    let db = Database::connect(db_opt).await?;

    Migrator::up(&db, None).await?;

    let (frame_tx, _) = tokio::sync::broadcast::channel(64);
    let (qr_tx, _) = tokio::sync::broadcast::channel(64);
    let (esp_msg_tx, esp_msg_rx) = tokio::sync::mpsc::channel(64);

    let tasks = vec![
        tokio::spawn(web_server::run(
            frame_tx.clone(),
            qr_tx.clone(),
            esp_msg_tx.clone(),
            db.clone()
        )),
        tokio::spawn(qr_scanner::run(frame_tx.subscribe(), qr_tx.clone())),
        tokio::spawn(frame_receiver::run(frame_tx.clone())),
        tokio::spawn(esp_comm::run(esp_msg_rx)),
    ];

    match futures::future::try_join_all(tasks).await {
        Ok(_) => {
            info!("All tasks executed successfully, exiting.");
        }
        Err(err) => {
            error!("Unexpected error executing task(s): {err}");
        }
    }

    Ok(())
}
