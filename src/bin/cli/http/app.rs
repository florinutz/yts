// use crate::cli::yts::input::{validate_min_rating, validate_natural_one_plus};
use clap::{
    app_from_crate, crate_authors, crate_description, crate_name, crate_version, App, AppSettings,
};

pub fn clap_app() -> App<'static, 'static> {
    app_from_crate!()
        .global_setting(AppSettings::NextLineHelp)
        .global_setting(AppSettings::ColoredHelp)
        .subcommand(App::new("serve").about("serves yts stuff"))
}
