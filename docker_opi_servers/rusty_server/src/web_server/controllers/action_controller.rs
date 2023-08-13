use axum::{Router, routing::post, response::IntoResponse, extract::State, http::StatusCode};
use tokio::sync::mpsc;

use crate::{web_server::AppState, EspMessage};

pub fn routes(state: AppState) -> Router { 
    Router::new()
        .route("/open", post(open_door))
        .with_state(state)
}

async fn open_door(
    State(esp_msg_tx): State<mpsc::Sender<EspMessage>>
) -> impl IntoResponse {
    match esp_msg_tx.send(EspMessage::Open).await {
        Ok(_) => {
            StatusCode::OK.into_response()
        },
        Err(_) => {
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        },
    }
}