use clap::ArgMatches;
use std::error::Error;
use yts::request::ListRequest;

pub fn list_req_from_clap<'a>(matches: &'a ArgMatches) -> Result<ListRequest<'a>, Box<dyn Error>> {
    let mut r = ListRequest::new();

    if let Some(val) = matches.value_of("limit") {
        r.limit(val.parse()?);
    }
    if let Some(val) = matches.value_of("page") {
        r.page(val.parse()?);
    }
    if let Some(val) = matches.value_of("quality") {
        r.quality(val.parse()?);
    }
    if let Some(val) = matches.value_of("rating") {
        r.rating(val.parse()?);
    }
    if let Some(vals) = matches.values_of("search") {
        let query = vals
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        r.query_term(query);
    }
    if let Some(val) = matches.value_of("genre") {
        r.genre(val);
    }
    if let Some(val) = matches.value_of("sort") {
        r.sort_by(val.parse()?);
    }
    if let Some(val) = matches.value_of("order") {
        r.order_by(val.parse()?);
    }
    if let Some(val) = matches.value_of("mirror") {
        r.mirror(val)?;
    }
    r.wirth_rt_ratings(matches.is_present("with_rotten_tomatoes"));

    Ok(r)
}

#[cfg(test)]
mod tests {
    use crate::cli::yts::app::clap_app;
    use crate::cli::yts::input::list_req_from_clap;

    fn test_url(vec: Vec<&str>) -> String {
        let actual_vec = &mut vec!["yts", "list"];
        actual_vec.extend(vec);
        let matches = clap_app().get_matches_from(actual_vec.to_vec());
        let request = list_req_from_clap(matches.subcommand_matches("list").unwrap())
            .expect("expected a request");
        request.url().to_string()
    }

    #[test]
    fn limit() {
        assert_eq!(
            test_url(vec!["-l", "14", "-p", "13"]),
            "https://yts.mx/api/v2/list_movies.json?limit=14&page=13"
        );
    }

    #[test]
    fn search() {
        assert_eq!(
            test_url(vec!["mama", "are", "mere"]),
            "https://yts.mx/api/v2/list_movies.json?limit=50&query_term=mama+are+mere"
        );
    }

    #[test]
    fn rotten() {
        assert_eq!(
            test_url(vec!["--rt"]),
            "https://yts.mx/api/v2/list_movies.json?limit=50&with_rt_ratings"
        );
    }

    #[test]
    fn sort() {
        assert_eq!(
            test_url(vec!["--sort", "title"]),
            "https://yts.mx/api/v2/list_movies.json?limit=50&sort_by=title"
        );
    }

    #[test]
    fn mirror() {
        assert_eq!(
            test_url(vec!["--mirror", "yts.ag"]),
            "https://yts.ag/api/v2/list_movies.json?limit=50"
        );
    }

    #[test]
    fn quality() {
        assert_eq!(
            test_url(vec!["--quality", "720p"]),
            "https://yts.mx/api/v2/list_movies.json?limit=50&quality=720p"
        );
    }
}
