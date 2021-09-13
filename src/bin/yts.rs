mod cli;

use crate::cli::yts::{app::clap_app, input::list_req_from_clap};
use log::error;
use std::process::exit;
use yts::parse::api::ListResponse;

fn main() {
    env_logger::init();

    let matches = clap_app().get_matches();

    // the list subcommand
    if let Some(list_matches) = matches.subcommand_matches("list") {
        let list: ListResponse = if list_matches.is_present("response-mock-file") {
            let json =
                std::fs::read_to_string(list_matches.value_of("response-mock-file").unwrap())
                    .expect("can't read mock json data");
            serde_json::from_str(json.as_str()).expect("expected a parsed response")
        } else {
            let request = list_req_from_clap(list_matches).unwrap_or_else(|e| {
                eprintln!("encountered a problem while retrieving the list");
                error!("can't retrieve list: {}", e);
                exit(1);
            });
            request.execute().unwrap_or_else(|e| {
                eprintln!("encountered a problem while retrieving the list");
                error!("can't retrieve list: {}", e);
                exit(1);
            })
        };

        println!("{}", list);
    }
}
