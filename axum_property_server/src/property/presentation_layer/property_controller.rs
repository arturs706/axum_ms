use axum::{
    routing::{get, post},
    Router,
};

use crate::{property::application_layer::property_service};

pub fn create_router() -> Router {
    Router::new()
        .route("/api/v1/property", get(property_service::get_all_properties))
}
