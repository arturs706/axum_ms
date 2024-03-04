use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
    middleware,
};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tower_http::cors::CorsLayer;
use user::{
    infrastructure_layer::route_auth_repo::auth_middleware,
    presentation_layer::user_controller::create_router,
};
mod user {
    pub mod application_layer;
    pub mod domain_layer;
    pub mod infrastructure_layer;
    pub mod presentation_layer;
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub database: Database,
    pub jwt_secret: JWTToken,
    pub pass_recovery_token: PasswordRecoveryToken,
}

#[derive(Clone, Debug)]
pub struct Database {
    pub db: Pool<Postgres>,
}

#[derive(Clone, Debug)]
pub struct JWTToken {
    pub jwt_secret: String,
}

#[derive(Clone, Debug)]
pub struct PasswordRecoveryToken {
    pub pass_recovery_token: String,
}

impl AppState {
    pub async fn new() -> Self {
        let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret: String = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let reset_password_secret: String =
            std::env::var("RESET_PASSWORD_SECRET").expect("RESET_PASSWORD_SECRET must be set");

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .expect("Failed to create pool");

        AppState {
            database: Database { db: pool },
            jwt_secret: JWTToken { jwt_secret },
            pass_recovery_token: PasswordRecoveryToken {
                pass_recovery_token: reset_password_secret,
            },
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router()
        .layer(cors)
        .route_layer(middleware::from_fn(auth_middleware));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:10001")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
