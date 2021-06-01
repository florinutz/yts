use std::{
    io::{self, Write},
    time::Duration,
};
use reqwest::blocking::ClientBuilder;
use yts::parse::api::ListResponse;
use cached_path::{Cache, Options};
use url::Url;
use reqwest::header::{HeaderMap, HeaderValue};

pub fn write_movies_list(out: &mut dyn Write, lr: &ListResponse) -> io::Result<()> {
    for movie in lr.data.as_ref().expect("missing data").movies.as_ref().expect("missing movies") {
        write!(out, "{}\n\n", movie)?;
    }
    Ok(())
}

pub fn get_list(url: &Url) -> Result<ListResponse, Box<dyn std::error::Error>> {
    let mut header_map = HeaderMap::new();
    header_map.insert("user-agent", HeaderValue::from_static("florinutz/yts"));
    let client_builder = ClientBuilder::new().timeout(Duration::from_secs(5)).default_headers(header_map);

    let cache = Cache::builder()
        .dir(std::env::temp_dir().join("yts/"))
        .client_builder(client_builder)
        .build().expect("can't build cache");

    let path = cache.cached_path_with_options(
        url.to_string().as_str(),
        &Options::default().subdir("list"),
    )?;

    let json = std::fs::read_to_string(path).expect("can't read file contents");
    let res: ListResponse = serde_json::from_str(json.as_str()).expect("can't parse json");

    Ok(res)
}
