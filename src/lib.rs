pub mod parse;

use crate::parse::api::ListResponse;
use cached_path::{Cache, Options as CacheOptions};
use reqwest::blocking::ClientBuilder;
use reqwest::header::{HeaderMap, HeaderValue};
use std::time::Duration;
use url::Url;

pub fn get_list(url: &Url) -> Result<ListResponse, Box<dyn std::error::Error>> {
    let mut header_map = HeaderMap::new();
    header_map.insert("user-agent", HeaderValue::from_static("florinutz/yts"));
    let client_builder = ClientBuilder::new()
        .timeout(Duration::from_secs(5))
        .default_headers(header_map);

    let cache = Cache::builder()
        .dir(std::env::temp_dir().join("yts/"))
        .client_builder(client_builder)
        .build()?;
    let path = cache.cached_path_with_options(
        url.to_string().as_str(),
        &CacheOptions::default().subdir("list"),
    )?;
    let json = std::fs::read_to_string(path)?;
    let res: ListResponse = serde_json::from_str(json.as_str())?;

    Ok(res)
}
