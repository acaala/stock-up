use bytes_stream::photo::get_one;
use clap::{arg, Parser};
use std::fs;
use std::io::Write;
use std::path::Path;
use std::{
    error::Error,
    fs::{read_to_string, File},
    io,
};

use rand::Rng;

static SEEDS: &'static [&str] = &[
    "nature", "cars", "coffee", "laptops", "shops", "sky", "science", "space",
];
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "empty")]
    seed: String,
    #[arg(short, long, default_value = "m")]
    image_size: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let target_dir = read_to_string("target_dir.txt").unwrap_or_else(|_| {
        let mut file = File::create("target_dir.txt").expect("should create file");

        println!("Enter the default download location:");
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        file.write_all(input.as_bytes()).unwrap();

        if !Path::new(&input).is_dir() {
            fs::create_dir(&input).unwrap();
        }

        println!("Set default download location to: {}", input);

        input
    });

    let seed = match args.seed.as_str() {
        "empty" => {
            let number = rand::thread_rng().gen_range(0..SEEDS.len());
            SEEDS[number]
        }
        _ => &args.seed,
    };

    get_one(seed, &args.image_size, &target_dir).await?;

    Ok(())
}
