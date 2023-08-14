use tokio::sync::mpsc;
use tracing::info;

use crate::utils::EspMessage;

pub async fn run(mut esp_msg_rx: mpsc::Receiver<EspMessage>) {
    loop {
        let msg = esp_msg_rx.recv().await;

        if let Some(msg) = msg {
            match msg {
                EspMessage::Open => info!("EspMessage::Open"),
            }
        }
    }
}