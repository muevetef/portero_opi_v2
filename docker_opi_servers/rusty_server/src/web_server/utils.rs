use axum::extract::FromRef;
use sea_orm::DatabaseConnection;
use serde::Serialize;
use tokio::sync::{mpsc, broadcast};
use std::sync::Arc;
use crate::utils::{Frame, QR, EspMessage};

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T
}

#[derive(Clone, FromRef)]
pub struct AppState {
    pub frame_tx: Arc<broadcast::Sender<Arc<Frame>>>,
    pub qr_tx: Arc<broadcast::Sender<QR>>,
    pub esp_msg_tx: mpsc::Sender<EspMessage>,
    pub db: DatabaseConnection
}
