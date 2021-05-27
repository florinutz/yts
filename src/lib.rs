mod api;

use select::document::Document;
use select::predicate::Class;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Item {
    pub title: String,
    pub year: u16,
    pub href: String,
    pub img: String,
    pub quality: String,
    pub rating: f32,
    pub genres: Vec<String>,
}

pub fn parse(html: String) -> Vec<Item> {
    let doc = Document::from(html.as_str());

    doc.select(Class("browse-movie-wrap")).map(|node| {
        let title = match node.select(Class("browse-movie-title")).next() {
            Some(node) => node.text(),
            None => "".to_string()
        };
        let year = match node.select(Class("browse-movie-year")).next() {
            Some(node) => node.text().parse::<u16>().unwrap_or(0),
            None => 0
        };
        let img = match node.select(Class("img-responsive")).next() {
            Some(node) => String::from(node.attr("src").unwrap()),
            None => "".to_string()
        };
        let href = match node.select(Class("browse-movie-link")).next() {
            Some(node) => node.attr("href").unwrap().to_string(),
            None => "".to_string()
        };
        let rating: f32 = match node.select(Class("rating")).next() {
            Some(node) => node.text().split_whitespace().next().unwrap_or("0").trim()
                .parse::<f32>().unwrap_or(0f32),
            None => 0f32
        };

        Item {
            title,
            year,
            href,
            img,
            rating,
            quality: "".to_string(),
            genres: vec![],
        }
    }).collect::<Vec<Item>>()
}

#[cfg(test)]
mod tests {
    use url::Url;

    #[test]
    fn parses_api_list() {
        use crate::api;
        let html = std::fs::read_to_string("test-data/list.json").expect("can't read test data");
        let response = api::from_str(html.as_str()).expect("expected a parsed response");
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

    #[test]
    fn it_works() {
        use crate::parse;
        let html = std::fs::read_to_string("test-data/list.html").expect("can't read test data");
        let items = parse(html);
        // todo move this next block to a command
        // todo add more tests for parsing
        // items.iter().for_each(|item| {
        //     print!("{title} {year} {rating}{href}{img}\n\n",
        //            title = item.title,
        //            year = if item.year > 0 { format!("({})", item.year) } else { "".to_string() },
        //            rating = if item.rating > 0f32 { format!("({:.1} imdb)", item.rating) } else { "".to_string() },
        //            href = if !item.href.is_empty() { format!("\n\t{}", item.href) } else { "".to_string() },
        //            img = if !item.img.is_empty() { format!("\n\t{}", item.img) } else { "".to_string() },
        //     );
        // });
        assert_eq!(items.len(), 14)
    }
}
