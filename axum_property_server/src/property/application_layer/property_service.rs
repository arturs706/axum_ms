use crate::property::{
    domain_layer::{property::Property},
    infrastructure_layer::{
        custom_error_repo::CustomErrors, jwt_repo, property_repository::PropertyRepository,
    },
};
use axum::{
    extract::Path,
    http::{HeaderMap, HeaderValue, StatusCode},
    Json,
};
use axum_macros::debug_handler;
use uuid::Uuid;

#[debug_handler]
pub async fn get_all_properties() -> Result<Json<Vec<Property>>, (StatusCode, String)> {
    let property_repository = PropertyRepository::new().await;
    match property_repository.get_all().await {
        Ok(properties) => Ok(Json(properties)),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}
