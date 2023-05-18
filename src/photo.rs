use indicatif::ProgressBar;
use reqwest::{header, Response};
use serde::Deserialize;
use serde_json::Value;
use std::{error::Error, fs::File, io::Write, process};
use tokio_stream::StreamExt;

const API_KEY: &str = "l00OLYhljlpXrMrbkUMNoydmez8duIPj2YpkXtpBeG3xmkw78yLUQro0";
const BASE_API_URL: &str = "https://api.pexels.com/v1/search";

#[derive(Deserialize, Debug)]
struct Photo {
    alt: String,
    src: PhotoSrc,
}

#[derive(Deserialize, Debug)]
struct PhotoSrc {
    large: String,
    medium: String,
    small: String,
}

async fn get_photos_from_api(seed: &str) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let mut headers = header::HeaderMap::new();
    headers.insert(header::AUTHORIZATION, API_KEY.parse().unwrap());

    let query_params = [("query", seed)];

    let res = client
        .get(BASE_API_URL)
        .headers(headers.clone())
        .query(&query_params)
        .send()
        .await?
        .text()
        .await?;

    Ok(res)
}

async fn get_photo_from_api(url: &str) -> Result<Response, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let mut headers = header::HeaderMap::new();
    headers.insert(header::AUTHORIZATION, API_KEY.parse().unwrap());

    let res = client.get(url).headers(headers).send().await?;

    if res.status().is_success() {
        Ok(res)
    } else {
        println!("Failed to get photo");
        process::exit(0)
    }
}

// Download One
async fn download_one(url: &str, alt: &str) -> Result<(), Box<dyn Error>> {
    let res = get_photo_from_api(url).await?;

    let bar = ProgressBar::new(res.content_length().unwrap());
    let mut stream = tokio_stream::iter(res.bytes().await?);

    println!("Saving File {}", alt);
    let mut file = File::create("image.jpg")?;

    while let Some(v) = stream.next().await {
        file.write_all(&[v])?;
        bar.inc(1);
    }

    bar.finish();
    Ok(())
}

// Download X Spin up new thread per download.

// Get One
pub async fn get_one(seed: &str, image_size: &str) -> Result<(), Box<dyn Error>> {
    let res = get_photos_from_api(seed).await?;

    let res_json: Value = serde_json::from_str(&res).expect("Should parse result");

    let json_photos: Vec<Photo> = serde_json::from_value(res_json["photos"].clone())
        .expect("Should pasrse res photo array into type");

    let photos: Vec<Photo> = json_photos.into_iter().collect();

    let first_photo = photos.first().expect("Should be a photo");

    let size = image_size.trim().to_lowercase();

    let photo_src = match size.as_str() {
        "large" | "l" => &first_photo.src.large,
        "medium" | "m" => &first_photo.src.medium,
        "small" | "s" => &first_photo.src.small,
        _ => &first_photo.src.medium,
    };

    download_one(photo_src, &first_photo.alt).await?;

    Ok(())
}

// Get Many
