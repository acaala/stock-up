use dialoguer::{theme::ColorfulTheme, Select};
use indicatif::ProgressBar;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use reqwest::{header, Response};
use serde::Deserialize;
use serde_json::{json, Value};
use std::{error::Error, fs::File, io::Write, path::Path, process};
use tokio_stream::StreamExt;

const API_KEY: &str = "l00OLYhljlpXrMrbkUMNoydmez8duIPj2YpkXtpBeG3xmkw78yLUQro0";
const BASE_API_URL: &str = "https://api.pexels.com/v1/search";

const DALLE_API_KEY: &str = "Bearer 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";
const DALLE_BASE_URL: &str = "https://api.openai.com/v1/images/generations";
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

async fn get_photos_from_ai(prompt: &str) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let mut headers = header::HeaderMap::new();
    // headers.insert(header::AUTHORIZATION, DALLE_API_KEY.parse().unwrap());

    let dalle_params = json!({
        "prompt": prompt,
        "n": 1,
        "size": "1024x1024",
    });

    let res = client
        .post(DALLE_BASE_URL)
        .headers(headers.clone())
        .json(&dalle_params)
        .send()
        .await?
        .text()
        .await?;

    let url: Value = serde_json::from_str(&res).unwrap();

    let url: String = serde_json::from_value(url["data"][0]["url"].clone()).unwrap();

    Ok(url)
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
async fn download_one(
    url: &str,
    alt: &str,
    target_dir: &str,
    seed: &str,
) -> Result<(), Box<dyn Error>> {
    let res = get_photo_from_api(url).await?;

    let bar = ProgressBar::new(res.content_length().unwrap());
    let mut stream = tokio_stream::iter(res.bytes().await?);

    println!("Saving File {}", alt);

    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(5)
        .map(char::from)
        .collect();

    let path_string = format!("{}/{}-{}.{}", target_dir, seed, rand_string, "jpg");
    let path = Path::new(&path_string);

    let mut file = File::create(path)?;

    while let Some(v) = stream.next().await {
        file.write_all(&[v])?;
        bar.inc(1);
    }

    bar.finish();
    Ok(())
}

// Download X Spin up new thread per download.

pub async fn get_one_from_ai(target_dir: &str, prompt: &str) -> Result<(), Box<dyn Error>> {
    let res = get_photos_from_ai(prompt).await?;

    download_one(&res, "Generating From DALLE", target_dir, "ai").await?;

    Ok(())
}

// Get One
pub async fn get_one(seed: &str, image_size: &str, target_dir: &str) -> Result<(), Box<dyn Error>> {
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

    download_one(&photo_src, &first_photo.alt, target_dir, seed).await?;

    Ok(())
}

// Get Many
pub async fn get_many(
    seed: &str,
    image_size: &str,
    target_dir: &str,
) -> Result<(), Box<dyn Error>> {
    let res = get_photos_from_api(seed).await?;

    let res_json: Value = serde_json::from_str(&res).expect("Should parse result");

    let json_photos: Vec<Photo> = serde_json::from_value(res_json["photos"].clone())
        .expect("Should pasrse res photo array into type");

    let photos: Vec<Photo> = json_photos.into_iter().collect();

    let selections: Vec<String> = photos.iter().map(|photo| photo.alt.clone()).collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an image:")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    let selected_photo = &photos[selection];

    let size = image_size.trim().to_lowercase();

    let photo_src = match size.as_str() {
        "large" | "l" => &selected_photo.src.large,
        "medium" | "m" => &selected_photo.src.medium,
        "small" | "s" => &selected_photo.src.small,
        _ => &selected_photo.src.medium,
    };

    download_one(photo_src, &selected_photo.alt, target_dir, seed).await?;

    Ok(())
}
