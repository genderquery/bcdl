use std::collections::HashMap;

use serde::Deserialize;

pub type ClientItems = Vec<ClientItem>;

#[derive(Debug, Deserialize)]
pub struct ClientItem {
    pub page_url: String,
}

#[derive(Debug, Deserialize)]
pub struct TrAlbum {
    pub artist: String,
    pub album_release_date: Option<String>,
    pub trackinfo: Vec<TrackInfo>,
    pub item_type: String,
    pub current: Current,
}

#[derive(Debug, Deserialize)]
pub struct Current {
    pub release_date: String,
}

#[derive(Debug, Deserialize)]
pub struct TrackInfo {
    pub file: Option<HashMap<String, String>>,
    pub artist: Option<String>,
    pub title: String,
    pub track_num: Option<u32>,
    pub release_date: Option<String>,
}
