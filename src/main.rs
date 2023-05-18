use bytes_stream::photo::{get_many, get_one};
use clap::{arg, Parser};
use std::io::Write;
use std::path::Path;
use std::{
    error::Error,
    fs::{read_to_string, File},
    io,
};
use std::{fs, process};

use rand::Rng;

const TARGET_DIR: &str = &"./images";

const SEEDS: &'static [&str] = &[
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

    // let target_dir = read_to_string("target_dir.txt").unwrap_or_else(|_| {
    //     let mut file = File::create("target_dir.txt").expect("should create file");

    //     println!("Enter the default download location:");
    //     let mut input = String::new();

    //     io::stdin().read_line(&mut input).unwrap();

    //     file.write_all(input.as_bytes()).unwrap();

    //     println!("Set default download location to: {}", input);

    //     input
    // });

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
