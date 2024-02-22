use crate::landlord::{
    domain_layer::landlord::Landlord,
    infrastructure_layer::{
        custom_error_repo::CustomErrors, jwt_repository, landlord_repository::LandlordRepository,
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
pub async fn get_all_landlords() -> Result<Json<Vec<Landlord>>, (StatusCode, String)> {
    let landlord_repository = LandlordRepository::new().await;
    match landlord_repository.get_all().await {
        Ok(landlords) => Ok(Json(landlords)),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}
