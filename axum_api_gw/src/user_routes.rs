use axum::{body::Body, extract::State, response::Response};
use axum::{http::HeaderMap, http::StatusCode};
use axum_macros::debug_handler;
use deadpool_redis::Connection;
use http_body_util::BodyExt;
use http_body_util::{Empty, Full};
use hyper::body::Bytes;
use hyper::Request;
use hyper_util::rt::TokioIo;
use redis::AsyncCommands;
use std::convert::Infallible;
use tokio::net::TcpStream;
use tower_cookies::{Cookie, Cookies};

use crate::AppState;

#[debug_handler]
pub async fn fetchusershandler(
    state: State<AppState>,
    headers: HeaderMap,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let mut conn: Connection = match state.redis_pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            return Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Full::new(Bytes::from(format!(
                    "Error getting connection from pool: {}",
                    e
                ))))
                .unwrap());
        }
    };
    let users: Option<String> = match conn.get("users").await {
        Ok(users) => users,
        Err(e) => {
            return Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Full::new(Bytes::from(format!(
                    "Error getting users from redis: {}",
                    e
                ))))
                .unwrap());
        }
    };
    match users {
        Some(users) => {
            let response = Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(Full::new(Bytes::from(users)))
                .unwrap();
            Ok(response)
        }
        None => {
            let url = "http://localhost:10001/api/v1/users"
                .parse::<hyper::Uri>()
                .unwrap();
            let host = url.host().expect("uri has no host");
            let port = url.port_u16().unwrap_or(80);
            let address = format!("{}:{}", host, port);
            let stream = TcpStream::connect(address).await.unwrap();
            let io = TokioIo::new(stream);
            let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await.unwrap();
            tokio::task::spawn(async move {
                if let Err(err) = conn.await {
                    println!("Connection failed: {:?}", err);
                }
            });

            let authority = url.authority().unwrap().clone();
            let bearer_token = headers.get("Authorization").unwrap().to_str().unwrap();
            let req = Request::builder()
                .uri(url)
                .header(hyper::header::HOST, authority.as_str())
                .header("Authorization", format!("Bearer {}", bearer_token))
                .body(Empty::<Bytes>::new())
                .unwrap();

            let mut res = sender.send_request(req).await.unwrap();
            let mut full_body = Vec::new();
            while let Some(next) = res.frame().await {
                let frame = next.unwrap();
                if let Some(chunk) = frame.data_ref() {
                    full_body.extend_from_slice(chunk);
                }
            }

            let response = Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(Full::new(Bytes::from(full_body.clone())))
                .unwrap();
            if let Some(mut connection) = state.redis_pool.get().await.ok() {
                let _: () = connection
                    .set("users", full_body)
                    .await
                    .expect("Error setting users in redis");
            }

            Ok(response)
        }
    }
}

#[debug_handler]
pub async fn login_user(
    cookies: Cookies,
    headers: HeaderMap,
    request: Request<Body>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let url = "http://localhost:10001/api/v1/users/login"
        .parse::<hyper::Uri>()
        .unwrap();
    let host = url.host().expect("uri has no host");
    let port = url.port_u16().unwrap_or(80);
    let address = format!("{}:{}", host, port);
    let stream = TcpStream::connect(address).await.unwrap();
    let io = TokioIo::new(stream);
    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await.unwrap();
    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });

    let authority = url.authority().unwrap().clone();
    let bearer_token = headers.get("Authorization").unwrap().to_str().unwrap();
    let req = Request::builder()
        .method("POST")
        .uri(url)
        .header(hyper::header::CONTENT_TYPE, "application/json")
        .header(hyper::header::HOST, authority.as_str())
        .header("Authorization", format!("Bearer {}", bearer_token))
        .body(request.into_body())
        .unwrap();
    let mut res = sender.send_request(req).await.unwrap();
    let mut full_body = Vec::new();
    while let Some(next) = res.frame().await {
        let frame = next.unwrap();
        if let Some(chunk) = frame.data_ref() {
            full_body.extend_from_slice(chunk);
        }
    }
    let headers = res.headers().clone();
    let access_token: Option<&http::HeaderValue> = headers.get("access_token");
    match access_token {
        Some(access_token) => {
            let refresh_token = &headers.get("refresh_token").unwrap().to_str().unwrap();
            cookies.add(Cookie::new("refresh_token", refresh_token.to_string()));
            let response = Response::builder()
                .status(StatusCode::OK)
                .header("access_token", access_token)
                .header("content-type", "application/json")
                .body(Full::new(Bytes::from(full_body)))
                .unwrap();
        
            Ok(response)
        }
        None => {
            let response = Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(Full::new(Bytes::from(full_body)))
                .unwrap();
            Ok(response)
        }
    }

}


#[debug_handler]
pub async fn register_user(headers: HeaderMap,request: Request<Body>) -> Result<Response<Full<Bytes>>, Infallible> {
    let url = "http://localhost:10001/api/v1/users/register"
        .parse::<hyper::Uri>()
        .unwrap();
    let host = url.host().expect("uri has no host");
    let port = url.port_u16().unwrap_or(80);
    let address = format!("{}:{}", host, port);
    let stream = TcpStream::connect(address).await.unwrap();
    let io = TokioIo::new(stream);
    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await.unwrap();
    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });
    let authority = url.authority().unwrap().clone();
    let bearer_token = headers.get("Authorization").unwrap().to_str().unwrap();
    let req = Request::builder()
        .method("POST")
        .uri(url)
        .header(hyper::header::CONTENT_TYPE, "application/json")
        .header(hyper::header::HOST, authority.as_str())
        .header("Authorization", format!("Bearer {}", bearer_token))
        .body(request.into_body())
        .unwrap();
    let mut res = sender.send_request(req).await.unwrap();
    let mut full_body = Vec::new();
    while let Some(next) = res.frame().await {
        let frame = next.unwrap();
        if let Some(chunk) = frame.data_ref() {
            full_body.extend_from_slice(chunk);
        }
    }
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Full::new(Bytes::from(full_body)))
        .unwrap();

    Ok(response)
}