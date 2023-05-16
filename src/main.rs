use std::{error::Error, fs::File, io::Write, process};

use clap::{arg, Parser};
use reqwest::header;

use serde::Deserialize;
use serde_json::Value;
use tokio_stream::StreamExt;

use indicatif::ProgressBar;

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

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    seed: String,
    #[arg(short, long)]
    image_size: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let client = reqwest::Client::new();

    let mut headers = header::HeaderMap::new();
    headers.insert(header::AUTHORIZATION, API_KEY.parse().unwrap());

    println!("Seed: {}!", args.seed);

    let query_params = [("query", args.seed)];

    let res = client
        .get(BASE_API_URL)
        .headers(headers.clone())
        .query(&query_params)
        .send()
        .await?
        .text()
        .await?;

    let res_json: Value = serde_json::from_str(&res).expect("Should parse result");

    let json_photos: Vec<Photo> = serde_json::from_value(res_json["photos"].clone())
        .expect("Should pasrse res photo array into type");

    let photos: Vec<Photo> = json_photos.into_iter().collect();

    let first_photo = photos.first().expect("Should be a photo");

    let size = args.image_size.trim().to_lowercase();

    let photo_src = match size.as_str() {
        "large" | "l" => &first_photo.src.large,
        "medium" | "m" => &first_photo.src.medium,
        "small" | "s" => &first_photo.src.small,
        _ => &first_photo.src.medium,
    };

    let res = client.get(photo_src).headers(headers).send().await?;

    if res.status().is_success() {
        let bar = ProgressBar::new(res.content_length().unwrap());

        let mut stream = tokio_stream::iter(res.bytes().await?);

        println!("Saving File {}", first_photo.alt);
        let mut file = File::create("image.jpg")?;

        while let Some(v) = stream.next().await {
            file.write_all(&[v])?;
            bar.inc(1);
        }

        bar.finish();
    } else {
        println!("Failed to download photo");
        process::exit(0);
    }

    Ok(())
}
