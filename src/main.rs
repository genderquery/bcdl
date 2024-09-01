use anyhow::Context;
use bcdl::json::{ClientItems, TrAlbum};
use clap::Parser;
use scraper::{selectable::Selectable, Html, Selector};
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

fn base_url(mut url: Url) -> Url {
    url.path_segments_mut().expect("URL can be a base").clear();
    url
}

fn crawl_band(url: Url) -> anyhow::Result<Vec<Url>> {
    let base_url = base_url(url.clone());

    let body = reqwest::blocking::get(url)?.text()?;
    let html = Html::parse_document(&body);

    let selector = Selector::parse("#music-grid").unwrap();
    let music_grid = html.select(&selector).next().context("no #music-grid")?;
    let client_items = music_grid.attr("data-client-items");

    if let Some(data_client_items) = client_items {
        let client_items: ClientItems = serde_json::from_str(data_client_items).unwrap();

        client_items
            .iter()
            .map(|item| Ok(base_url.join(item.page_url.as_str())?))
            .collect()
    } else {
        let music_grid_item_selector = Selector::parse(".music-grid-item > a").unwrap();
        let links = music_grid.select(&music_grid_item_selector);

        links
            .map(|link| {
                link.attr("href")
                    .context("no href")
                    .and_then(|url| Ok(base_url.join(url)?))
            })
            .collect()
    }
}

fn scrape_album_or_track(url: Url) -> anyhow::Result<TrAlbum> {
    let body = reqwest::blocking::get(url)?.text()?;
    let html = Html::parse_document(&body);

    let selector = Selector::parse("script[data-tralbum]").unwrap();
    let tralbum = html
        .select(&selector)
        .next()
        .context("no script[data-tralbum]")?
        .attr("data-tralbum")
        .context("no attr data-tralbum")?;

    Ok(serde_json::from_str(tralbum).unwrap())
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let url = args.url;
    let url_type = UrlType::from_url(&url).unwrap();

    let tracks: anyhow::Result<Vec<TrAlbum>> = match url_type {
        UrlType::Band => {
            let urls = crawl_band(url)?;
            urls.into_iter().map(scrape_album_or_track).collect()
        }
        UrlType::Album => todo!(),
        UrlType::Track => todo!(),
    };

    dbg!(tracks?);

    Ok(())
}
