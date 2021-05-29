mod cli;

extern crate clap;

use crate::cli::app::clap_app;
use crate::cli::input::get_list_url;

fn main() {
    let app = clap_app();
    let matches = app.get_matches();
    let url = get_list_url(matches.subcommand_matches("list").unwrap());
    print!("{}", url)
}

