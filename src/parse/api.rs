use chrono::serde::ts_nanoseconds_option;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::fmt::{self};
use url::Url;

#[derive(Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct ListResponse {
    pub status: Option<String>,
    pub status_message: Option<String>,
    pub data: Option<Data>,
    #[serde(rename = "@meta")]
    pub meta: Option<Meta>,
}

#[derive(Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct Data {
    pub movie_count: Option<u64>,
    pub limit: Option<u32>,
    pub page_number: Option<u32>,
    pub movies: Option<Vec<Movie>>,
}

#[derive(Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct Movie {
    pub id: Option<u32>,
    pub url: Option<Url>,
    pub imdb_code: Option<String>,
    pub title: Option<String>,
    pub title_long: Option<String>,
    pub year: Option<u16>,
    pub rating: Option<f32>,
    pub runtime: Option<u16>,
    pub genres: Option<Vec<String>>,
    pub summary: Option<String>,
    pub description_full: Option<String>,
    pub synopsis: Option<String>,
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

#[derive(Deserialize, Clone, Debug)]
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

#[derive(Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct Meta {
    #[serde(with = "ts_nanoseconds_option")]
    pub server_time: Option<DateTime<Utc>>,
    pub server_timezone: Option<String>,
    pub api_version: Option<u8>,
    pub execution_time: Option<String>,
}

impl Movie {
    /// Returns the string representation for the id. It can be empty.
    pub fn get_id(&self) -> String {
        match self.id {
            Some(id) if id > 0 => format!("{}", id),
            _ => "".to_string(),
        }
    }

    /// Returns the string representation for the rating. It can be empty.
    pub fn get_rating(&self) -> String {
        match self.rating {
            Some(rating) if rating > 0.0 => format!("{:.1}", rating),
            _ => "".to_string(),
        }
    }

    /// Returns the string representation for the year. It can be empty.
    pub fn get_year(&self) -> String {
        match self.year {
            Some(year) if year > 0 => format!("{:<4}", year),
            _ => "".to_string(),
        }
    }

    /// Returns the string representation for the title. It can be empty.
    pub fn get_title(&self) -> String {
        self.title.to_owned().unwrap_or_else(|| "???".to_string())
    }

    /// Returns the string representation for the long title (including year). It can be empty.
    pub fn get_title_long(&self) -> String {
        self.title_long.to_owned().unwrap_or_else(|| "".to_string())
    }

    /// Returns the string representation for the yts url. It can be empty.
    pub fn get_url(&self) -> String {
        match &self.url {
            Some(url) => url.to_string(),
            _ => "".to_string(),
        }
    }

    /// Returns the string representation for the youtube trailer. It can be empty.
    pub fn get_youtube(&self) -> String {
        match &self.yt_trailer_code {
            Some(trailer_code) if !trailer_code.is_empty() => {
                format!("https://www.youtube.com/watch?v={}", trailer_code)
            }
            _ => "".into(),
        }
    }

    /// Returns the string representation for the imdb link. It can be empty.
    pub fn get_imdb(&self) -> String {
        match &self.imdb_code {
            Some(imdb) if !imdb.is_empty() => format!("https://www.imdb.com/title/{}/", imdb),
            _ => "".into(),
        }
    }

    /// Returns the string representation for the movie genres. It can be empty.
    pub fn get_genres(&self) -> String {
        match &self.genres {
            Some(genres) => genres
                .iter()
                .map(|g| g.to_lowercase())
                .collect::<Vec<String>>()
                .join(", "),
            None => "".to_string(),
        }
    }

    /// Returns the string representation for the movie summary. It can be empty.
    pub fn get_text(&self, description_type: MovieDescription) -> String {
        use MovieDescription::*;
        match description_type {
            Summary => self.summary.clone().unwrap_or_else(|| "".to_string()),
            Description => self
                .description_full
                .clone()
                .unwrap_or_else(|| "".to_string()),
            Synopsis => self.synopsis.clone().unwrap_or_else(|| "".to_string()),
        }
    }
}

#[derive(Debug)]
pub enum MovieDescription {
    Summary,
    Description,
    Synopsis,
}

impl fmt::Display for Movie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{id:<6} {rating} {year} {title} {genres}\n\t{url:60}{youtube}",
            id = self.get_id(),
            title = self.get_title(),
            rating = self.get_rating(),
            year = self.get_year(),
            url = self.get_url(),
            genres = self.get_genres(),
            youtube = self.get_youtube(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::api::ListResponse;
    use url::Url;
    static JSON: &str = include_str!("test-data/list.json");

    #[test]
    fn parses_api_list() {
        let response: ListResponse =
            serde_json::from_str(JSON).expect("expected a parsed response");

        assert_eq!(
            response.status,
            Some("ok".to_string()),
            "response_status = '{}'",
            "ok"
        );
        assert_eq!(
            response.status_message,
            Some("Query was successful".to_string())
        );

        let data = response.data.expect("there should be some data here");
        assert_eq!(data.movie_count, Some(31474));
        assert_eq!(data.limit, Some(2));
        assert_eq!(data.page_number, Some(1));
        let movies = data.movies.expect("there should be movies here");
        assert_eq!(movies.len(), 2, "2 movies?");
        let movie = movies.first().unwrap();
        assert_eq!(
            movie.url,
            Some(Url::parse("https://yts.mx/movies/la-via-dei-babbuini-1974").unwrap())
        );
        assert_eq!(movie.imdb_code, Some("tt0144665".to_string()));
        assert_eq!(movie.title, Some("La via dei babbuini".to_string()));
        assert_eq!(movie.year, Some(1974));
        assert_eq!(movie.rating, Some(6.8));
        assert_eq!(
            movie
                .genres
                .clone()
                .expect("there should be genres here")
                .first()
                .expect("there should be a genre here"),
            "Comedy"
        );
        assert!(movie
            .summary
            .clone()
            .unwrap_or_else(|| "".to_string())
            .starts_with("This is a probably underrated brave attempt"));
        let torrent = movie
            .torrents
            .as_ref()
            .expect("missing torrents")
            .first()
            .expect("missing first torrent");
        assert_eq!(torrent.ty_pe, Some("web".to_string()));
        assert_eq!(
            torrent.url,
            Some(
                Url::parse(
                    "https://yts.mx/torrent/download/673B3BA1335C6D1F5035C086A98676BF6C738276"
                )
                .unwrap()
            )
        );
        let meta = response.meta.expect("there's a @meta section in the json");
        assert_eq!(meta.server_time.unwrap().timestamp_nanos(), 1622039993)
    }
}
