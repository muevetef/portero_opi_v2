use axum::Router;

use super::AppState;

mod ws_controller;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .nest("/ws", ws_controller::routes(state.clone()))
}