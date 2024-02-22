use crate::{user::domain_layer::user::StaffUser, AppState};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthClaims {
    iss: AuthType,
    exp: i64,
    iat: i64,
    role: Issuer,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AuthType {
    APIGW,
    Access,
    Refresh,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Issuer {
    Admin,
    User,
    Other,
}

impl AuthType {
    pub fn new(auth_type: &str) -> Self {
        match auth_type {
            "access" => Self::Access,
            "refresh" => Self::Refresh,
            _ => Self::APIGW,
        }
    }
}

impl Issuer {
    pub fn _new(issuer: &str) -> Self {
        match issuer {
            "admin" => Self::Admin,
            "user" => Self::User,
            _ => Self::Other,
        }
    }
}

impl AuthClaims {
    pub fn new(_issuer: &str, role: Issuer, auth_type: &str) -> Self {
        let iat = Utc::now().timestamp();
        let exp = match auth_type {
            "refresh" => (Utc::now() + Duration::days(30)).timestamp(),
            _ => (Utc::now() + Duration::minutes(15)).timestamp(),
        };

        AuthClaims {
            iss: AuthType::new(auth_type),
            exp,
            iat,
            role,
        }
    }
}

pub async fn validate_token(token: &str) -> Result<AuthClaims, jsonwebtoken::errors::Error> {
    let validation = Validation::new(Algorithm::HS256);
    let app_state = AppState::new().await;
    let jwt_secret: String = app_state.jwt_secret.jwt_secret.clone();
    let access_secret = jwt_secret.as_bytes();
    let access_verify = decode::<AuthClaims>(
        &token,
        &DecodingKey::from_secret(access_secret),
        &validation,
    )
    .map(|data| data.claims);
    match access_verify {
        Ok(claims) => {
            if claims.exp < Utc::now().timestamp() {
                return Err(jsonwebtoken::errors::Error::from(
                    jsonwebtoken::errors::ErrorKind::InvalidToken,
                ));
            }
            Ok(claims)
        }
        Err(e) => Err(e.into()),
    }
}

pub async fn create_token(
    user: &StaffUser,
    issuer: &str,
    auth_type: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let app_state = AppState::new().await;
    let access_token_secret: String = app_state.access_token.access_token.clone();
    let access_secret = access_token_secret.as_bytes();
    let role = if user.username == "arturs" {
        Issuer::Admin
    } else {
        Issuer::User
    };
    let claims = AuthClaims::new(issuer, role, auth_type);
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(access_secret),
    )?;
    Ok(token)
}
