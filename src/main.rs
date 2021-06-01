mod cli;

extern crate clap;

use crate::cli::app::clap_app;
use crate::cli::input::get_list_url;
use reqwest::Result;
use crate::cli::output::{write_movies_list, get_list};

#[tokio::main]
async fn main() -> Result<()> {
    let matches = clap_app().get_matches();

    if let Some(list_matches) = matches.subcommand_matches("list") {
        let list =
            if list_matches.is_present("response-mock-file") {
                let json = std::fs::read_to_string(
                    list_matches.value_of("response-mock-file").unwrap()
                ).expect("can't read mock json data");
                serde_json::from_str(json.as_str()).expect("expected a parsed response")
            } else {
                let url = get_list_url(list_matches);
                get_list(&url).expect("can't get list")
            };

        write_movies_list(&mut std::io::stdout(), &list).expect("couldn't write movies list");
    }

    Ok(())
}
