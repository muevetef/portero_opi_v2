use std::{net::SocketAddr, sync::Arc};

use axum::{extract::{WebSocketUpgrade, ConnectInfo, ws::{WebSocket, Message}, State}, TypedHeader, headers, response::IntoResponse, Router, routing::get};
use tokio::sync::broadcast::{Receiver, Sender};
use tracing::{info, warn, debug};

use crate::{web_server::AppState, Frame, QR};

pub fn routes(state: AppState) -> Router { 
    Router::new()
        .route("/cam", get(cam_ws_upgrade))
        .route("/qr", get(qr_ws_upgrade))
        .with_state(state)
}

async fn cam_ws_upgrade(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(frame_tx): State<Arc<Sender<Arc<Frame>>>>
) -> impl IntoResponse {
    let user_agent = match user_agent {
        Some(user_agent) => user_agent.0.to_string(),
        None => "Unknown Browser".to_owned(),
    };

    info!("`{user_agent}`@{addr} connecting to camera feed...");

    ws.on_upgrade(move |socket| cam_ws_handle(socket, addr, frame_tx.subscribe()))
}

async fn cam_ws_handle(mut socket: WebSocket, addr: SocketAddr, mut frame_rx: Receiver<Arc<Frame>>) {
    if socket.send(Message::Ping(vec![1,2,3])).await.is_ok() {
        debug!("Pinged {addr}...")
    } else {
        debug!("Could not send ping {addr}!");
        return;
    }
    
    loop {
        let frame = match frame_rx.recv().await {
            Ok(frame) => frame,
            Err(_) => {
                warn!("Could not get frame for client {addr}");
                return;
            },
        };

        match socket.send(Message::Binary(frame.data.clone())).await {
            Ok(_) => (),
            Err(_) => {
                info!("{addr} disconnected from camera feed!");
                return;
            },
        }
    }
}

async fn qr_ws_upgrade(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(qr_tx): State<Arc<Sender<QR>>>
) -> impl IntoResponse {
    let user_agent = match user_agent {
        Some(user_agent) => user_agent.0.to_string(),
        None => "Unknown Browser".to_owned(),
    };

    info!("`{user_agent}`@{addr} connecting to qr feed...");

    ws.on_upgrade(move |socket| qr_ws_handle(socket, addr, qr_tx.subscribe()))
}

async fn qr_ws_handle(mut socket: WebSocket, addr: SocketAddr, mut qr_rx: Receiver<QR>) {
    if socket.send(Message::Ping(vec![1,2,3])).await.is_ok() {
        debug!("Pinged {addr}...")
    } else {
        debug!("Could not send ping {addr}!");
        return;
    }
    
    loop {
        let qr = match qr_rx.recv().await {
            Ok(qr) => qr,
            Err(_) => {
                warn!("Could not get QR for client {addr}");
                return;
            },
        };

        let json = match serde_json::to_string(&qr) {
            Ok(qr) => {qr},
            Err(_) => { return },
        };

        match socket.send(Message::Text(json)).await {
            Ok(_) => (),
            Err(_) => {
                info!("{addr} disconnected from QR feed!");
                return;
            },
        }
    }
}