use clap::SubCommand;
use clap::{App, Arg};
use std::env;

mod config;
mod pinboard;

fn use_env_var() -> (bool, String) {
    let status: bool;
    let env_api_token = env::var("PINBOARD_API").unwrap_or("none".to_string());
    if env_api_token.to_owned().eq("none") {
        status = false;
    } else {
        status = true;
    }
    return (status, env_api_token);
}

fn main() {
    env_logger::init();
    let _matches = App::new("Bloodhound")
        .version("1.0")
        .author("Alejandro Miranda <alejandro.miranda@hey.com>")
        .about("Dig through pinboard bookmarks")
        .subcommand(SubCommand::with_name("config")
            .arg(Arg::with_name("update")
                .long("update").short("u").takes_value(true).help("Update the current api token")
                )
            .help("Manage user settings/configuration")
        )
        .get_matches();

    // use environment variable
    let (use_env_var, mut token_string) = use_env_var();

    // load or create config file
    let mut config = config::Config::new();
    if config.exists() {
        config.load().unwrap();
    } else {
        config.create().unwrap();
    }

    // use from config file
    if !use_env_var {
        token_string = config.get_token();
    }
    let mut pinboard = pinboard::Api::new(token_string);

    // verify pinboard communication
    pinboard.verify().unwrap();

    // show list of recent bookmarks
    pinboard.get_recent().unwrap();

}
