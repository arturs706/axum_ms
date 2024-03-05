use crate::property::{
    domain_layer::property::Property,
    infrastructure_layer::{
        custom_error_repo::CustomErrors, jwt_repo, property_repository::PropertyRepository,
    },
};
use axum::{
    extract::{Multipart, Path},
    http::{HeaderMap, StatusCode},
    Json,
};
use axum_macros::debug_handler;
use crossbeam::channel::*;
use crossbeam_channel::{bounded, select, tick, Receiver};
use image::{imageops::FilterType, DynamicImage};
use signal_hook::consts::SIGINT;
use signal_hook::iterator::Signals;
use std::{fs::File, io::Write, path::Path as StdPath, thread, time::Instant};
use tokio::fs as tfs;
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
pub async fn get_property_by_id(
    Path(id): Path<Uuid>,
) -> Result<Json<Property>, (StatusCode, String)> {
    let property_repository = PropertyRepository::new().await;
    match property_repository.get_by_id(id).await {
        Ok(property) => Ok(Json(property)),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

#[debug_handler]
pub async fn create_property(
    headers: HeaderMap,
    Json(property): Json<Property>,
) -> Result<Json<Property>, (StatusCode, String)> {
    let property_repository = PropertyRepository::new().await;
    let auth = headers
        .get("Authorization")
        .ok_or(CustomErrors::MissingCreds)
        .unwrap();
    let token = auth
        .to_str()
        .map_err(|_| CustomErrors::MissingCreds)
        .unwrap();
    let authtoken = token.replace("Bearer ", "");
    match jwt_repo::validate_token(&authtoken).await {
        Ok(_) => match property_repository.create(property).await {
            Ok(property) => Ok(Json(property)),
            Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
        },
        Err(e) => {
            println!("Access verify error: {:?}", e);
            match e.kind() {
                jsonwebtoken::errors::ErrorKind::InvalidToken => {
                    Err((StatusCode::UNAUTHORIZED, "Invalid Token".to_string()))
                }
                _ => Err((StatusCode::UNAUTHORIZED, "Invalid Key".to_string())),
            }
        }
    }
}

#[debug_handler]
pub async fn upload_images(
    headers: HeaderMap,
    mut payload: Multipart,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let auth = headers
        .get("Authorization")
        .ok_or(CustomErrors::MissingCreds)
        .unwrap();
    let token = auth
        .to_str()
        .map_err(|_| CustomErrors::MissingCreds)
        .unwrap();
    let authtoken = token.replace("Bearer ", "");
    let base_url: &str = "http://0.0.0.0:10003/api/v1/property/uploadedimg/"; // Replace with your actual server URL
    let mut image_urls: Vec<String> = Vec::new();

    match jwt_repo::validate_token(&authtoken).await {
        Ok(_) => {
            let start_time: std::time::Instant = std::time::Instant::now();

            if let Err(error) = tfs::create_dir_all("./uploadedimg/").await {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Error creating directory: {}", error),
                ));
            }

            let content_length: usize = match headers.get("content-length") {
                Some(header_value) => header_value.to_str().unwrap_or("0").parse().unwrap(),
                None => "0".parse().unwrap(),
            };

            //================================================================================================

            //================================================================================================

            let max_file_size: usize = 100_000_000;
            let dir: &str = "./uploadedimg/";
            println!("Content Length: {}", content_length);
            if content_length > max_file_size {
                return Err((StatusCode::BAD_REQUEST, "File too large".to_string()));
            }
            while let Some(mut field) = payload
                .next_field()
                .await
                .map_err(|error| (StatusCode::BAD_REQUEST, error.to_string()))?
            {
                match field.content_type() {
                    Some("image/png") => "image/png".to_string(),
                    Some("image/jpeg") => "image/jpeg".to_string(),
                    Some("image/webp") => "image/webp".to_string(),
                    Some("image/avif") => "image/avif".to_string(),
                    _ => {
                        tracing::error!("Invalid content type");
                        return Err((StatusCode::BAD_REQUEST, "Invalid content type".to_string()));
                    }
                };

                let name = field
                    .name()
                    .map(ToString::to_string)
                    .unwrap_or("name".to_owned());
                let file_name = field
                    .file_name()
                    .map(ToString::to_string)
                    .unwrap_or("file_name".to_owned());
                let file_extension = match file_name.split('.').last() {
                    Some(ext) => ext,
                    None => {
                        tracing::error!("No file extension found");
                        return Err((
                            StatusCode::BAD_REQUEST,
                            "No file extension found".to_string(),
                        ));
                    }
                };
                let unique_file_name = format!("{}-{}.{}", name, Uuid::new_v4(), file_extension);
                let file_path = format!("{}/{}", dir, unique_file_name);
                let mut file = match File::create(&file_path) {
                    Ok(file) => file,
                    Err(error) => {
                        return Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Error opening file for writing: {}", error),
                        ));
                    }
                };

                while let Some(chunk) = field
                    .chunk()
                    .await
                    .map_err(|error| (StatusCode::BAD_REQUEST, error.to_string()))?
                {
                    file.write_all(&chunk)
                        .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))?;
                }
                let new_path = handle_image_processing(file_path).await;
                let full_url = format!("{}{}.webp", base_url, new_path);
                image_urls.push(full_url);
            }
            let elapsed_time = start_time.elapsed().as_millis();
            println!("Elapsed time: {}", elapsed_time);
            Ok((StatusCode::OK, "Images uploaded successfully".to_string()))
        }
        Err(e) => {
            println!("Access verify error: {:?}", e);
            match e.kind() {
                jsonwebtoken::errors::ErrorKind::InvalidToken => {
                    Err((StatusCode::UNAUTHORIZED, "Invalid Token".to_string()))
                }
                _ => Err((StatusCode::UNAUTHORIZED, "Invalid Key".to_string())),
            }
        }
    }
}

async fn handle_image_processing(destination: String) -> String {
    let uploaded_img: DynamicImage = image::open(&destination).unwrap();
    let _ = tfs::remove_file(&destination).await.unwrap();
    let file_name_without_extension = match StdPath::new(&destination).file_stem() {
        Some(name) => name.to_string_lossy().into_owned(),
        None => {
            tracing::error!("No file name found");
            return "No file name found".to_string();
        }
    };

    let upload_dir = "./uploadedimg/";
    let new_file_path = format!("{}{}.avif", upload_dir, file_name_without_extension);

    uploaded_img
        .resize_exact(1920, 1080, FilterType::Gaussian)
        .save(&new_file_path)
        .unwrap();

    return file_name_without_extension;
}
