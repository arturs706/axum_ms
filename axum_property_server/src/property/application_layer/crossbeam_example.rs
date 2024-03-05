use axum::{
    extract::Multipart,
    http::{header, HeaderMap, StatusCode},
};
use crossbeam_channel::{bounded, unbounded, Sender};
use crossbeam_utils::thread as xbutthread;
use image::{imageops::FilterType, DynamicImage};
use std::future::Future;
use std::{fs::File, io::Write, path::Path as StdPath, thread, time::Instant};
use tokio::fs as tfs;
use uuid::Uuid;

use crate::property::infrastructure_layer::{
    custom_error_repo::CustomErrors, property_repository::PropertyRepository,
};

pub async fn upload_images(
    headers: HeaderMap,
    mut payload: Multipart,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let property_repository = PropertyRepository::new().await;
    if let Err(error) = tfs::create_dir_all("./uploadedimg/").await {
        tracing::error!("Error creating directory: {}", error);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error creating directory: {}", error),
        ));
    }
    let image_qty: usize = match headers.get("x-image-quantity") {
        Some(header_value) => header_value.to_str().unwrap_or("0").parse().unwrap(),
        None => "0".parse().unwrap(),
    };
    let base_url: &str = "http://0.0.0.0:10003/api/v1/property/uploadedimg/"; // Replace with your actual server URL

    let mut futures: Vec<String> = Vec::with_capacity(image_qty as usize);
    let start_time: std::time::Instant = std::time::Instant::now();

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
        let dir: &str = "./uploadedimg/";
        let unique_file_name = format!("{}-{}.{}", name, Uuid::new_v4(), file_extension);

        futures.push(unique_file_name.clone());
        let file_path = format!("{}/{}", dir, unique_file_name.clone());

        let mut file = match File::create(&file_path) {
            Ok(file) => file,
            Err(error) => {
                tracing::error!("Error opening file for writing: {}", error);
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
    }

    let (middle_sender, middle_receiver) = bounded(image_qty);
    let (final_sender, final_receiver) = unbounded();
    xbutthread::scope(|scope| {
        scope.spawn(move |_| {
            for _ in 0..image_qty {
                if let Ok(name) = middle_receiver.recv() {
                    final_sender
                        .send(name)
                        .expect("Failed to send the final name.");
                }
            }
        });

        for image in futures.clone() {
            println!("Processing: {:?}", image);
            let middle_sender_clone: Sender<String> = middle_sender.clone();
            scope.spawn(move |_| {
                let middle_sender_clone = middle_sender_clone;
                let dir_string = String::from("./uploadedimg/");
                let fut = async move {
                    let modified_name = handle_image_processing(dir_string+&image.to_string()).await;
                    middle_sender_clone
                        .send(modified_name)
                        .expect("Failed to send the modified name to the next step.");
                };
                tokio::runtime::Runtime::new().unwrap().block_on(fut)
            });
        }
    })
    .unwrap();

    for _ in 0..futures.clone().len() {
        if let Ok(final_name) = final_receiver.recv() {
            println!("Processed: {:?}", final_name);
        }
    }
    let elapsed_time = start_time.elapsed().as_millis();
    println!("Elapsed time: {}", elapsed_time);
    Ok((StatusCode::OK, "Images are being processed".to_string()))
}

async fn handle_image_processing(destination: String) -> String {
    println!("Processing: {:?}", destination);
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
