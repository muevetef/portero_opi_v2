use std::{net::SocketAddr, sync::Arc};

use axum::Router;
use tokio::sync::broadcast::Receiver;
use tower_http::services::ServeDir;
use tracing::info;

use crate::{Frame, QR};

pub async fn run(frame_rx: Receiver<Arc<Frame>>, qr_rx: Receiver<QR>) {
    let app = Router::new()
        .nest_service("/", ServeDir::new("public"));

    let addr = SocketAddr::from(([0,0,0,0], 8080));

    let server = axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>());

    info!("Web server started, listening on http://{addr}");

    server.await.unwrap();
}