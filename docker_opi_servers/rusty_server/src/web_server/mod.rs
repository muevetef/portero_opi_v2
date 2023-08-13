use std::{net::SocketAddr, sync::Arc};

use axum::{Router, extract::FromRef};
use tokio::sync::broadcast::Sender;
use tower_http::services::ServeDir;
use tracing::info;

use crate::{Frame, QR};

mod controllers;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub frame_tx: Arc<Sender<Arc<Frame>>>,
    pub qr_tx: Arc<Sender<QR>>
}

pub async fn run(frame_tx: Sender<Arc<Frame>>, qr_tx: Sender<QR>) {
    let state = AppState {
        frame_tx: Arc::new(frame_tx),
        qr_tx: Arc::new(qr_tx)
    };
    
    let app = Router::new()
        .merge(controllers::routes(state))
        .fallback_service(ServeDir::new("public").append_index_html_on_directories(true));
    
    let addr = SocketAddr::from(([0,0,0,0], 8080));

    let server = axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>());

    info!("Web server started, listening on http://{addr}");

    server.await.unwrap();
}