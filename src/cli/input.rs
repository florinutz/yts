use strum::{EnumString, EnumVariantNames, Display, EnumIter};
use clap::ArgMatches;
use url::Url;

#[derive(Display, Debug, PartialEq, EnumString, EnumVariantNames, EnumIter)]
pub enum Qualities {
    #[strum(serialize = "720p")]
    Q720p,
    #[strum(serialize = "1080p")]
    Q1080p,
    #[strum(serialize = "2160p")]
    Q2160p,
    #[strum(serialize = "3D")]
    Q3D,
}

pub fn validate_natural_one_plus(msg: String) -> impl Fn(String) -> Result<(), String> {
    move |val| match val.trim().parse::<u16>() {
        Ok(x) if x > 0 => Ok(()),
        _ => Err(msg.clone()),
    }
}

pub fn validate_min_rating(msg: String) -> impl Fn(String) -> Result<(), String> {
    move |val| match val.trim().parse::<u8>() {
        Ok(x) if (1..=9).contains(&x) => Ok(()),
        _ => Err(msg.clone()),
    }
}

/// Builds an url for the call described here https://yts.mx/api#list_movies
pub fn get_list_url(matches: &ArgMatches) -> Url {
    let mut url = Url::parse("https://yts.mx/api/v2/list_movies.json").unwrap();

    if let Some(limit) = matches.value_of("limit") {
        url.query_pairs_mut().append_pair("limit", limit);
    }

    if let Some(val) = matches.value_of("page") {
        url.query_pairs_mut().append_pair("page", val);
    }

    if let Some(val) = matches.value_of("quality") {
        url.query_pairs_mut().append_pair("quality", val);
    }

    if let Some(val) = matches.value_of("rating") {
        url.query_pairs_mut().append_pair("minimum_rating", val);
    }

    if let Some(val) = matches.value_of("query") {
        url.query_pairs_mut().append_pair("query_term", val);
    }

    if let Some(val) = matches.value_of("genre") {
        url.query_pairs_mut().append_pair("genre", val);
    }

    if let Some(val) = matches.value_of("sort") {
        url.query_pairs_mut().append_pair("sort_by", val);
    }

    if let Some(val) = matches.value_of("order") {
        url.query_pairs_mut().append_pair("order_by", val);
    }

    if matches.is_present("with_rotten_tomatoes") {
        url.query_pairs_mut().append_key_only("with_rt_ratings");
    }

    url
}

#[cfg(test)]
mod tests {
    use crate::get_list_url;
    use crate::cli::app::clap_app;

    fn url_from_cli_input(vec: Vec<&str>) -> String {
        let vec = vec!["yts", "list"].into_iter().chain(vec.into_iter()).collect::<Vec<&str>>();
        get_list_url(
            clap_app().get_matches_from(vec)
                .subcommand_matches("list").unwrap()
        ).to_string()
    }

    #[test]
    fn limit() {
        assert_eq!(url_from_cli_input(vec!["-l", "14", "-p", "13"]), "https://yts.mx/api/v2/list_movies.json?limit=14&page=13");
    }

    #[test]
    fn rotten() {
        assert_eq!(url_from_cli_input(vec!["--rt"]), "https://yts.mx/api/v2/list_movies.json?limit=50&with_rt_ratings");
    }

    #[test]
    fn sort() {
        assert_eq!(url_from_cli_input(vec!["-s", "title"]), "https://yts.mx/api/v2/list_movies.json?limit=50&sort_by=title");
    }
}
