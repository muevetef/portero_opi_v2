use std::{net::SocketAddr, sync::Arc};

use axum::Router;
use sea_orm::DatabaseConnection;
use tokio::sync::{broadcast, mpsc};

use tower_http::services::ServeDir;
use tracing::info;

use crate::{utils::{Frame, QR, EspMessage}, web_server::utils::AppState};

mod controllers;
mod utils;
mod error;

pub async fn run(
    frame_tx: broadcast::Sender<Arc<Frame>>, 
    qr_tx: broadcast::Sender<QR>, 
    esp_msg_tx: mpsc::Sender<EspMessage>,
    db: DatabaseConnection
) {
    let state = AppState {
        frame_tx: Arc::new(frame_tx),
        qr_tx: Arc::new(qr_tx),
        esp_msg_tx,
        db
    };
    
    let app = Router::new()
        .nest("/api/", controllers::routes(state))
        .fallback_service(ServeDir::new("public").append_index_html_on_directories(true));
    
    let addr = SocketAddr::from(([0,0,0,0], 8080));

    let server = axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>());

    info!("Web server started, listening on http://{addr}");

    server.await.unwrap();
}