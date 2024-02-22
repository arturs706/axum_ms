use crate::user::{
    domain_layer::user::StaffUser,
    infrastructure_layer::{
        custom_error_repo::CustomErrors, jwt_repo, user_repository::UserRepository,
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
pub async fn get_all_users() -> Result<Json<Vec<StaffUser>>, (StatusCode, String)> {
    let user_repository = UserRepository::new().await;
    match user_repository.get_all().await {
        Ok(users) => Ok(Json(users)),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

#[debug_handler]
pub async fn get_user_by_id(
    Path(user_id): Path<Uuid>,
) -> Result<Json<StaffUser>, (StatusCode, String)> {
    let user_repository = UserRepository::new().await;
    match user_repository.get_by_id(user_id).await {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

#[debug_handler]
pub async fn register_user(
    Json(body): Json<StaffUser>,
) -> Result<Json<StaffUser>, (StatusCode, String)> {
    let user_repository = UserRepository::new().await;
    user_repository
        .save(StaffUser {
            user_id: Some(Uuid::new_v4()),
            name: body.name,
            username: body.username,
            mob_phone: body.mob_phone,
            passwd: body.passwd,
            acc_level: body.acc_level,
            status: body.status,
            a_created: Some(chrono::Local::now().naive_local()),
        })
        .await
        .map(|user| Json(user))
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))
}

#[debug_handler]
pub async fn login_user(
    Json(body): Json<StaffUser>,
) -> Result<(StatusCode, HeaderMap, Json<StaffUser>), CustomErrors> {
    let user_repository = UserRepository::new().await;
    let mut header = HeaderMap::new();
    if body.passwd.is_empty() {
        return Err(CustomErrors::MissingCreds);
    }
    match user_repository.login(body.username, body.passwd).await {
        Ok(user) => {
            let access_token = jwt_repo::create_token(&user, "user", "access").await;
            let refresh_token = jwt_repo::create_token(&user, "user", "refresh").await;
            match access_token {
                Ok(provided_token) => {
                    header.append(
                        "access_token",
                        HeaderValue::from_str(&provided_token).unwrap(),
                    );
                    header.append(
                        "refresh_token",
                        HeaderValue::from_str(&refresh_token.unwrap()).unwrap(),
                    );
                    Ok((StatusCode::OK, header, Json(user)))
                }
                Err(_) => Err(CustomErrors::InternalServerError),
            }
        }
        Err(_) => Err(CustomErrors::MissingCreds),
    }
}
