use std::error::Error;

use bytes_stream::photo::get_one;
use clap::{arg, Parser};
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

    get_one(&args.seed, &args.image_size).await?;

    Ok(())
}
