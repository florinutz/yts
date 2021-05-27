use serde_json::Result;
use serde::Deserialize;
use url::Url;
use chrono::{DateTime, Utc};
use chrono::serde::ts_nanoseconds_option;
use std::io::Read;

#[derive(Deserialize, Clone)]
#[allow(dead_code)]
pub struct Response {
    pub status: Option<String>,
    pub status_message: Option<String>,
    pub data: Option<Data>,
    #[serde(rename = "@meta")]
    pub meta: Option<Meta>,
}

#[derive(Deserialize, Clone)]
#[allow(dead_code)]
pub struct Data {
    pub movie_count: Option<u64>,
    pub limit: Option<u32>,
    pub page_number: Option<u32>,
    pub movies: Option<Vec<Movie>>,
}

#[derive(Deserialize, Clone)]
#[allow(dead_code)]
pub struct Movie {
    pub id: Option<u32>,
    pub url: Option<Url>,
    pub imdb_code: Option<String>,
    pub title: Option<String>,
    pub year: Option<u16>,
    pub rating: Option<f32>,
    pub runtime: Option<u16>,
    pub genres: Option<Vec<String>>,
    pub summary: Option<String>,
    pub description_full: Option<String>,
    pub sypnosis: Option<String>,
    pub yt_trailer_code: Option<String>,
    pub language: Option<String>,
    pub mpa_rating: Option<String>,
    pub background_image: Option<String>,
    pub background_image_original: Option<String>,
    pub small_cover_image: Option<String>,
    pub medium_cover_image: Option<String>,
    pub large_cover_image: Option<String>,
    pub state: Option<String>,
    pub torrents: Option<Vec<Torrent>>,
    pub date_uploaded: Option<String>,
    #[serde(with = "ts_nanoseconds_option")]
    pub date_uploaded_unix: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Clone)]
#[allow(dead_code)]
pub struct Torrent {
    pub url: Option<Url>,
    pub hash: Option<String>,
    pub quality: Option<String>,
    #[serde(rename = "type")]
    pub ty_pe: Option<String>,
    pub seeders: Option<u32>,
    pub peers: Option<u32>,
    pub size: Option<String>,
    pub size_bytes: Option<u64>,
    pub date_uploaded: Option<String>,
    #[serde(with = "ts_nanoseconds_option")]
    pub date_uploaded_unix: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Clone)]
#[allow(dead_code)]
pub struct Meta {
    #[serde(with = "ts_nanoseconds_option")]
    pub server_time: Option<DateTime<Utc>>,
    pub server_timezone: Option<String>,
    pub api_version: Option<u8>,
    pub execution_time: Option<String>,
}

#[allow(dead_code)]
pub fn from_str(data: &str) -> Result<Response> {
    serde_json::from_str(data)
}

#[allow(dead_code)]
pub fn from_reader<R:Read>(reader: R) -> Result<Response> {
    serde_json::from_reader(reader)
}
