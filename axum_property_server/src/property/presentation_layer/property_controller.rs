use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post},
    Router,
};

use crate::property::application_layer::{crossbeam_example, property_serv_single};

pub fn create_router() -> Router {
    Router::new()
        .route(
            "/api/v1/property/upload_images",
            post(property_serv_single::upload_images).route_layer(DefaultBodyLimit::max(135476000)),
        )
        .route(
            "/api/v1/property/upload_images/crossbeam",
            post(crossbeam_example::upload_images).route_layer(DefaultBodyLimit::max(135476000)),
        )
        .layer(tower_http::trace::TraceLayer::new_for_http())
}
