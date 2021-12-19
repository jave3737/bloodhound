use crate::config::*;
use crate::pinboard::*;
use clap::{App, Arg};
use std::env;
use std::path::Path;

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

    let (use_env_var, token_string) = use_env_var();

    let config = Config::new();
    let pinboard = Api::new(token_string);

    if let Some(s) = matches.value_of("config") {
        config.create_blank().unwrap();
    }

    if use_env_var {
        todo!("here we do stuff")
    } else {
        if config.exists() {
            todo!("attempt to read config file settings")
        } else {
            todo!("create a blank config file")
        }
    }

    if matches.is_present("test") {
        pinboard.verify();
    }
}
