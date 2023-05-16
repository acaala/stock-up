use std::{error::Error, process};

use clap::{arg, Parser};
use reqwest::header;

use serde::Deserialize;
use serde_json::Value;
use tokio::io::{self, AsyncReadExt};

const API_KEY: &str = "l00OLYhljlpXrMrbkUMNoydmez8duIPj2YpkXtpBeG3xmkw78yLUQro0";
const BASE_API_URL: &str = "https://api.pexels.com/v1/search";

#[derive(Deserialize, Debug)]
struct Photo {
    url: String,
    id: i32,
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
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let client = reqwest::Client::new();

    let mut headers = header::HeaderMap::new();
    headers.insert(header::AUTHORIZATION, API_KEY.parse().unwrap());

    let query_params = [("query", "nature")];

    println!("Hello {}!", args.name);

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

    let res = client
        .get(&first_photo.src.small)
        .headers(headers)
        .send()
        .await?;

    if res.status().is_success() {
        let body = res.bytes().await?;
        let buffer = body.to_vec();

        // Process each byte of the downloaded image
        for byte in buffer {
            // Do something with each byte
            println!("Byte: {}", byte);
        }
    } else {
        println!("Failed to download photo");
        process::exit(0);
    }

    // let res = first_photo.get(BASE_API_URL).headers(headers)
    // let mut photos: Vec<Photo> = Vec::new();

    // for photo in json_photos.iter() {
    //     println!("{:#?}", photo);
    //     photos.push(photo);
    // }

    Ok(())
}
