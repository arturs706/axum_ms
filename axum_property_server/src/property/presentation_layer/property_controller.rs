use axum::{
    routing::{get, post},
    Router,
    extract::DefaultBodyLimit
};

use crate::property::application_layer::property_service;

pub fn create_router() -> Router {
    Router::new()
        .route("/api/v1/property", get(property_service::get_all_properties))
        .route("/api/v1/property/upload", post(property_service::create_property))
        .route("/api/v1/property/{id}", get(property_service::get_property_by_id))
        .route("/api/v1/property/upload_images", post(property_service::upload_images)
        .route_layer(DefaultBodyLimit::max(135476000)))
        .layer(tower_http::trace::TraceLayer::new_for_http())

}
