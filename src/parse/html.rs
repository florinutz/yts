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

#[allow(dead_code)]
pub fn parse(html: &str) -> Vec<Item> {
    let doc = Document::from(html);

    doc.select(Class("browse-movie-wrap"))
        .map(|node| {
            let title = match node.select(Class("browse-movie-title")).next() {
                Some(node) => node.text(),
                None => "".to_string(),
            };
            let year = match node.select(Class("browse-movie-year")).next() {
                Some(node) => node.text().parse::<u16>().unwrap_or(0),
                None => 0,
            };
            let img = match node.select(Class("img-responsive")).next() {
                Some(node) => String::from(node.attr("src").unwrap()),
                None => "".to_string(),
            };
            let href = match node.select(Class("browse-movie-link")).next() {
                Some(node) => node.attr("href").unwrap().to_string(),
                None => "".to_string(),
            };
            let rating: f32 = match node.select(Class("rating")).next() {
                Some(node) => node
                    .text()
                    .split_whitespace()
                    .next()
                    .unwrap_or("0")
                    .trim()
                    .parse::<f32>()
                    .unwrap_or(0f32),
                None => 0f32,
            };

            Item {
                title,
                year,
                href,
                img,
                rating,
                quality: "".to_string(), // todo these
                genres: vec![],
            }
        })
        .collect::<Vec<Item>>()
}

#[cfg(test)]
mod tests {
    static LIST_HTML: &str = include_str!("test-data/list.html");

    #[test]
    fn it_works() {
        use crate::parse::html::parse;
        let items = parse(LIST_HTML);
        assert_eq!(items.len(), 2)
    }
}
