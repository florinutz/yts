use crate::cli::input::{validate_min_rating, validate_natural_one_plus};
use clap::{
    app_from_crate, crate_authors, crate_description, crate_name, crate_version, App, AppSettings,
    Arg,
};

pub fn clap_app() -> App<'static, 'static> {
    app_from_crate!()
        .global_setting(AppSettings::NextLineHelp)
        .global_setting(AppSettings::ColoredHelp)
        .subcommand(
            App::new("list")
                .about("lists movies")
                .args(&[
                    Arg::with_name("search").takes_value(true).help("Search query")
                        .multiple(true)
                        .long_help("Search query, matching on: Movie Title/IMDb Code, Actor Name/IMDb Code, Director Name/IMDb Code"),
                    Arg::with_name("limit").short("l").long("limit").takes_value(true)
                        .default_value("50")
                        .help("The limit of results per page that has been set")
                        .long_help("The limit of results per page that has been set.\n\
                        Integer between 1 - 50 (inclusive)"),
                    Arg::with_name("page")
                        .takes_value(true).short("p").long("page")
                        .help("The page in the list of movies")
                        .long_help("The page in the list of movies\n\
                        e.g. limit=15 and page=2 will show you movies 15-30\n\
                        Integer (Unsigned)")
                        .validator(validate_natural_one_plus("should be 1 or more".to_string())),
                    Arg::with_name("quality")
                        .long("quality").short("q").takes_value(true)
                        .case_insensitive(true).possible_values(&["720p", "1080p", "2160p", "3D"])
                        .help("Filter by a given quality")
                        .long_help("Filter by a given quality\nString (720p, 1080p, 2160p, 3D)"),
                    Arg::with_name("rating")
                        .takes_value(true).short("r").long("rating")
                        .help("Filter movie by a given minimum IMDb rating")
                        .long_help("Filter movie by a given minimum IMDb rating\nInteger between 0 - 9 (inclusive)")
                        .validator(validate_min_rating("this should be an integer between 1 and 9".to_string())),
                    Arg::with_name("genre").takes_value(true).short("g").long("genre")
                        .help("Filter by a given genre")
                        .long_help("Filter by a given genre (See http://www.imdb.com/genre/ for full list)"),
                    Arg::with_name("sort").long("sort").short("s").takes_value(true)
                        .case_insensitive(true)
                        .possible_values(&["title", "year", "rating", "peers", "seeds", "download_count", "like_count", "date_added"])
                        .help("Sorts the results by a criteria")
                        .long_help("Sorts the results by a criteria\nString (title, year, rating, peers, seeds, download_count, like_count, date_added)"),
                    Arg::with_name("order").long("order").short("o").takes_value(true)
                        .case_insensitive(true)
                        .possible_values(&["desc", "asc"])
                        .help("Order the results ascending or descending")
                        .long_help("Order the results ascending or descending. String. 'asc' or 'desc'."),
                    Arg::with_name("with_rotten_tomatoes").long("rt")
                        .help("get rotten tomatoes ratings")
                        .long_help("Return the list with the Rotten Tomatoes rating included"),
                    Arg::with_name("mirror").takes_value(true).long("mirror").help("domain / mirror to use")
                        .long_help("will switch the domain to a mirror")
                        .possible_values(&["yts.mx", "yts.lt", "yts.am", "yts.ag"]).default_value("yts.mx"),
                    Arg::with_name("response-mock-file").long("response-mock").short("mock")
                        .help("does not perform any connection, but uses a local json instead")
                        .takes_value(true).hidden(true),
                ])
        )
}
