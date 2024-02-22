use axum::{
    routing::{get, post},
    Router,
};

use crate::landlord::application_layer::landlord_service;

pub fn create_router() -> Router {
    Router::new()
        .route("/api/v1/landlord", get(landlord_service::get_all_landlords))
}
