use bytes_stream::photo::{get_many, get_one};
use clap::{arg, Parser};
use std::fs;

use std::error::Error;
use std::path::Path;

use rand::Rng;

const TARGET_DIR: &str = "./images";

const SEEDS: &[&str] = &[
    "nature", "cars", "coffee", "laptops", "shops", "sky", "science", "space",
];
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "empty")]
    seed: String,

    #[arg(short, long, default_value = "m")]
    image_size: String,

    #[arg(short, long, default_value_t = false)]
    list: bool,

    #[arg(short = 'O', long, default_value_t = false)]
    open_image_directory: bool,

    #[arg(short = 'o', long, default_value_t = false)]
    open_after_download: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    if !Path::new(TARGET_DIR).is_dir() {
        fs::create_dir(TARGET_DIR).unwrap();
    }

    if args.open_image_directory {
        opener::open(TARGET_DIR).expect("should open file explorer");
        return Ok(());
    }

    let seed = match args.seed.as_str() {
        "empty" => {
            let number = rand::thread_rng().gen_range(0..SEEDS.len());
            SEEDS[number]
        }
        _ => &args.seed,
    };

    if args.list {
        get_many(seed, &args.image_size, TARGET_DIR).await?;
    } else {
        get_one(seed, &args.image_size, TARGET_DIR).await?;
    }

    if args.open_after_download {
        opener::open(TARGET_DIR).expect("should open file explorer");
    }

    Ok(())
}
