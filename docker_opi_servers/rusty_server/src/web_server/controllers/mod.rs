use axum::Router;

use super::AppState;

mod ws_controller;
mod action_controller;
mod users_controller;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .nest("/users", users_controller::routes(state.clone()))
        .nest("/ws", ws_controller::routes(state.clone()))
        .nest("/action",  action_controller::routes(state.clone()))
}