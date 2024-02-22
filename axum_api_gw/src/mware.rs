use axum::{
    body::Body,
    http::{header::HeaderValue, Request},
    middleware::Next,
    response::Response,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header as JsonWebTokenHeader};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthClaims {
    iss: String,
    exp: i64,
    iat: i64,
    role: String,
}

impl AuthClaims {
    pub fn new(issuer: String) -> Self {
        let iat = Utc::now();
        let exp = iat + Duration::hours(72);

        Self {
            iss: issuer,
            exp: exp.timestamp(),
            iat: iat.timestamp(),
            role: "Other".to_string(),
        }
    }
}

pub async fn add_token(req: Request<Body>, next: Next) -> Result<Response<Body>, Response<Body>> {
    let issuer = "APIGW".to_string();
    let jwt_secret: String = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let jwt_token = match encode(
        &JsonWebTokenHeader::new(Algorithm::HS256),
        &AuthClaims::new(issuer.clone()),
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    ) {
        Ok(token) => token,
        Err(_) => return Err(Response::new(Body::empty())),
    };
    let mut req = req;
    req.headers_mut().insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", jwt_token)).unwrap(),
    );
    let response = next.run(req).await;
    Ok(response)
}
