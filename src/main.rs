use clap::{App, Arg};
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::env;
use std::io::Write;
use std::path::Path;
use crate::pinboard::*;
use crate::config::*;

mod pinboard;
mod config;

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
    let matches = App::new("Bloodhound")
        .version("1.0")
        .author("Alejandro Miranda <alejandro.miranda@hey.com>")
        .about("Dig through pinboard bookmarks")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .takes_value(true)
                .help("Create a config.yaml that stores your api token"),
        )
        .arg(
            Arg::with_name("test")
                .short("t")
                .long("test")
                .takes_value(false)
                .help("Test your configuration"),
        )
        .get_matches();

    let (_use_env_var, token_string) = use_env_var();

    let config = Config::new();
    if let Some(s) = matches.value_of("config") {
        config.create_config_file().unwrap();
    }

    let pinboard = Api::new(token_string);
    if matches.is_present("test") {
        pinboard.verify_api_connection();
    }
}
