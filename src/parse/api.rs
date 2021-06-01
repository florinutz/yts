use serde::Deserialize;
use url::Url;
use chrono::{DateTime, Utc};
use chrono::serde::ts_nanoseconds_option;
use std::fmt::{self};

#[derive(Deserialize, Clone)]
#[allow(dead_code)]
pub struct ListResponse {
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
    pub title_long: Option<String>,
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

impl fmt::Display for Movie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{id:<6} {rating} {year} {title} {genres}\n\t{url:60}{youtube}",
            id=self.id.to_owned().unwrap_or(0),
            title=self.title.to_owned().unwrap_or("missing title".to_string()),
            rating=format!("{:<3}", match self.rating {
                Some(rating) if rating > 0.0 => rating.to_string(),
                _ => "".to_string(),
            }),
            year = match &self.year {
               Some(year) if *year > 0 => format!("{:<4}", *year),
               _ => "".to_string(),
            },
            url=self.url.to_owned().expect("missing movie url").as_str(),
            genres = match &self.genres {
               Some(genres) => format!(
                   "({})",
                   genres.
                       into_iter().
                       map(|g| g.to_lowercase()).
                       collect::<Vec<String>>().
                       join(", ")
               ),
               None => "".to_string(),
            },
            youtube = match &self.yt_trailer_code {
                Some(trailer_code) if !trailer_code.is_empty() => format!("\n\thttps://www.youtube.com/watch?v={}", trailer_code),
                _ => "".into(),
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use url::Url;

    #[test]
    fn parses_api_list() {
        let json = std::fs::read_to_string("test-data/list.json").expect("can't read test data");
        let response = serde_json::from_str(json.as_str()).expect("expected a parsed response");
        assert_eq!(response.status, Some("ok".to_string()), "response_status = '{}'", "ok");
        assert_eq!(response.status_message, Some("Query was successful".to_string()));
        let data = response.data.expect("there should be some data here");
        assert_eq!(data.movie_count, Some(31474));
        assert_eq!(data.limit, Some(2));
        assert_eq!(data.page_number, Some(1));
        let movies = data.movies.expect("there should be movies here");
        assert_eq!(movies.len(), 2, "2 movies?");
        let movie = movies.first().unwrap();
        assert_eq!(movie.url, Some(Url::parse("https://yts.mx/movies/la-via-dei-babbuini-1974").unwrap()));
        assert_eq!(movie.imdb_code, Some("tt0144665".to_string()));
        assert_eq!(movie.title, Some("La via dei babbuini".to_string()));
        assert_eq!(movie.year, Some(1974));
        assert_eq!(movie.rating, Some(6.8));
        assert_eq!(movie.genres.clone().expect("there should be genres here").first().expect("there should be a genre here"), "Comedy");
        assert!(movie.summary.clone().unwrap_or("".to_string()).starts_with("This is a probably underrated brave attempt"));
        let torrent = movie.torrents.as_ref().expect("missing torrents").first().expect("missing first torrent");
        assert_eq!(torrent.ty_pe, Some("web".to_string()));
        assert_eq!(torrent.url, Some(Url::parse("https://yts.mx/torrent/download/673B3BA1335C6D1F5035C086A98676BF6C738276").unwrap()));
        let meta = response.meta.expect("there's a @meta section in the json");
        assert_eq!(meta.server_time.unwrap().timestamp_nanos(), 1622039993)
    }
}