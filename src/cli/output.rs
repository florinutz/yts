use cached_path::{Cache, Options as CacheOptions};
use colored::Colorize;
use hyphenation::{Language, Load, Standard};
use prettytable::{format, Cell, Row, Table};
use reqwest::blocking::ClientBuilder;
use reqwest::header::{HeaderMap, HeaderValue};
use std::{
    io::{self},
    time::Duration,
};
use textwrap::{fill, Options as TextWrapOptions};
use url::Url;
use yts::parse::api::{ListResponse, MovieDescription};

pub fn write_list(out: &mut dyn io::Write, list: &ListResponse) -> io::Result<()> {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER);

    for movie in list
        .data
        .as_ref()
        .expect("missing data")
        .movies
        .as_ref()
        .expect("missing movies")
    {
        let left = format!(
            "{rating}\n\n{year}\n{genres}\n\n{id}",
            rating = movie.get_rating().as_str().green(),
            year = movie.get_year().as_str().green(),
            genres = fill(movie.get_genres().as_str(), 12),
            id = movie.get_id(),
        );
        let right = format!(
            "{title}\n{url}\n{yt}\n{imdb}\n\n{summary}",
            title = movie.get_title().as_str().bright_green(),
            url = movie.get_url(),
            yt = movie.get_youtube(),
            imdb = movie.get_imdb(),
            summary = {
                let text = movie.get_text(MovieDescription::Summary);
                let dictionary = Standard::from_embedded(Language::EnglishUS).unwrap();
                let options = TextWrapOptions::new(90).splitter(dictionary);
                fill(text.as_str(), &options)
            },
        );
        let cells = vec![Cell::new(right.as_str()), Cell::new(left.as_str())];
        table.add_row(Row::new(cells));
    }

    table.print(out)?;

    Ok(())
}

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
