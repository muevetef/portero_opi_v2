use axum::Json;
use axum::{Router, routing::get, extract::State};
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::entities::users as User;
use crate::web_server::utils::{AppState, ApiResponse};

use crate::web_server::error::{Result, Error};

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(get_users))
        .with_state(state)
}

async fn get_users(
    State(db): State<DatabaseConnection>
) -> Result<Json<ApiResponse<Vec<User::Model>>>> {
    let users: Vec<User::Model> = User::Entity::find()
        .all(&db)
        .await
        .map_err(|_| Error::DatabaseError)?;

    Ok(Json(
        ApiResponse { success: true, data: users }
    ))
}
