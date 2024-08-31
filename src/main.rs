use anyhow::anyhow;
use clap::Parser;
use scraper::{selectable::Selectable, Html, Selector};
use serde_json::Value;
use url::Url;

#[derive(Debug, Parser)]
struct Args {
    url: Url,
}

enum UrlType {
    Band,
    Album,
    Track,
}

impl UrlType {
    pub fn from_url(url: &Url) -> Option<UrlType> {
        match url.path_segments()?.next()? {
            "" | "music" => Some(UrlType::Band),
            "album" => Some(UrlType::Album),
            "track" => Some(UrlType::Track),
            _ => None,
        }
    }
}

fn base_url(mut url: Url) -> Result<Url, ()> {
    url.path_segments_mut()?.clear();
    Ok(url)
}

fn crawl_band(url: Url) -> anyhow::Result<()> {
    let base_url = base_url(url.clone()).map_err(|_| anyhow!("cannot be a base"))?;

    let body = reqwest::blocking::get(url)?.text()?;
    let html = Html::parse_document(&body);

    if !html.errors.is_empty() {
        println!("Found errors while parsing:");
        for error in html.errors.iter() {
            println!("{}", error);
        }
    }

    let selector = Selector::parse("#music-grid").unwrap();
    let music_grid = html
        .select(&selector)
        .next()
        .ok_or(anyhow!("no #music-grid"))?;
    let data_client_items = music_grid.attr("data-client-items");
    if let Some(data_client_items) = data_client_items {
        parse_data_client_items(data_client_items, base_url)?;
    } else {
        let music_grid_item_selector = Selector::parse(".music-grid-item > a").unwrap();
        let links = music_grid.select(&music_grid_item_selector);
        for link in links {
            let url = link.attr("href").ok_or(anyhow!("no href"))?;
            let url = base_url.join(url)?;
            println!("{}", url);
        }
    }

    Ok(())
}

fn parse_data_client_items(data_client_items: &str, base_url: Url) -> anyhow::Result<()> {
    let data_client_items: Value = serde_json::from_str(data_client_items)?;
    let data_client_items = data_client_items
        .as_array()
        .ok_or(anyhow!("not an array"))?;
    for item in data_client_items.iter() {
        let item = item.as_object().ok_or(anyhow!("not an object"))?;
        let url = item.get("page_url").ok_or(anyhow!("no page_url"))?;
        let url = url.as_str().ok_or(anyhow!("not a str"))?;
        let url = base_url.join(url)?;
        println!("{}", url);
    }

    Ok(())
}

fn crawl_album(_url: Url) -> anyhow::Result<()> {
    todo!()
}

fn crawl_track(_url: Url) -> anyhow::Result<()> {
    todo!()
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let url = args.url;
    let url_type = UrlType::from_url(&url).unwrap();

    match url_type {
        UrlType::Band => crawl_band(url),
        UrlType::Album => crawl_album(url),
        UrlType::Track => crawl_track(url),
    }
}
