use axum::{
    routing::{get, post},
    Router,
};

use crate::user::application_layer::user_service;

pub fn create_router() -> Router {
    Router::new()
        .route("/api/v1/users", get(user_service::get_all_users))
        .route("/api/v1/users/:user_id", get(user_service::get_user_by_id))
        .route("/api/v1/users/register", post(user_service::register_user))
        .route("/api/v1/users/login", post(user_service::login_user))
}
