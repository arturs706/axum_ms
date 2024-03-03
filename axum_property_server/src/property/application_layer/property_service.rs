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

#[debug_handler]
pub async fn get_property_by_id(Path(id): Path<Uuid>) -> Result<Json<Property>, (StatusCode, String)> {
    let property_repository = PropertyRepository::new().await;
    match property_repository.get_by_id(id).await {
        Ok(property) => Ok(Json(property)),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

#[debug_handler]
pub async fn create_property(
    headers: HeaderMap,
    Json(property): Json<Property>
) -> Result<Json<Property>, (StatusCode, String)> {
    let property_repository = PropertyRepository::new().await;
    let auth = headers
        .get("Authorization")
        .ok_or(CustomErrors::MissingCreds).unwrap();
    let token = auth.to_str().map_err(|_| CustomErrors::MissingCreds).unwrap();
    let authtoken = token.replace("Bearer ", "");
    match jwt_repo::validate_token(&authtoken).await {
        Ok(_) => match property_repository.create(property).await {
            Ok(property) => Ok(Json(property)),
            Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
        },
        Err(e) => {
            println!("Access verify error: {:?}", e);
            match e.kind() {
                jsonwebtoken::errors::ErrorKind::InvalidToken => Err((StatusCode::UNAUTHORIZED, "Invalid Token".to_string())),
                _ => Err((StatusCode::UNAUTHORIZED, "Invalid Key".to_string())),
            }
        }
    }
}
