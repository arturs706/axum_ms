mod mware;
mod user_routes;
use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
    middleware,
    routing::{get, post},
    Router,
};
use deadpool_redis::{Config, Pool, Runtime};
use dotenv::dotenv;
use std::time::Duration;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;
mod unit_tests;

type DeadpoolPool = Pool;

#[derive(Clone)]
pub struct AppState {
    pub redis_pool: DeadpoolPool,
}

#[derive(Clone)]
pub struct RedisPool {
    pub pool: DeadpoolPool,
}

// const PREFIX: &str = "with_deadpool";
// const TTL: usize = 60 * 5;
const MAX_POOL_SIZE: usize = 50;
const WAIT_TIMEOUT: Option<Duration> = Some(Duration::from_secs(10));

pub fn create_pool() -> Result<DeadpoolPool, String> {
    dotenv().ok();
    let redis_url: String = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
    let config = Config::from_url(redis_url);
    config
        .builder()
        .map(|b| {
            b.max_size(MAX_POOL_SIZE)
                .wait_timeout(WAIT_TIMEOUT) // TODO needs create_timeout/recycle timeout?
                .runtime(Runtime::Tokio1)
                .build()
                .unwrap() // TODO don't panic. flat_map can't be used???
        })
        .map_err(|e| e.to_string())
}

pub(crate) fn new_app() -> Router {
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);
    let redis_pool_data = create_pool().unwrap();
    let state = AppState {
        redis_pool: redis_pool_data.clone(),
    };
    Router::new()
        .route("/api/v1", get(user_routes::fetchusershandler))
        .route("/api/v1", post(user_routes::login_user))
        .layer(cors)
        .layer(CookieManagerLayer::new())
        .layer(middleware::from_fn(mware::add_token))
        .with_state(state)
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let app = new_app();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9999").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
