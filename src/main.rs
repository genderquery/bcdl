use std::error;

use bcdl::{parse_url, url_from_artist, PageUrl};
use clap::Parser;
use url::Url;

#[derive(Parser)]
struct Args {
    url_or_artist: String,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let args = Args::parse();

    let page: Result<PageUrl, Box<dyn error::Error>> =
        Url::parse(&args.url_or_artist)
            .or_else(|_| Ok(url_from_artist(&args.url_or_artist)?))
            .and_then(|url| Ok(parse_url(&url)?));

    println!("Using {}", page?.url.as_str());

    Ok(())
}
