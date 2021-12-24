use crate::config::*;
use crate::pinboard::*;
use clap::SubCommand;
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
        .subcommand(SubCommand::with_name("config")
            .arg(Arg::with_name("exists")
                .long("exists").short("e").takes_value(false).help("Checks if a config file exists"))
            .help("Manage user settings/configuration")
        )
        .subcommand(SubCommand::with_name("test")
            .arg(Arg::with_name("verify")
                .long("verify").short("v").takes_value(false).help("Verify that the currently configured api token can communicate with Pinboard.in"))
            .help("Test/debug settings")
        )
        .get_matches();

    let (_use_env_var, token_string) = use_env_var();

    let config = Config::new();
    let pinboard = Api::new(token_string);

    if let Some(s) = matches.subcommand_matches("config") {
        if s.is_present("exist") {
            if config.exists() {
                println!("config file exists")
            } else {
                println!("config does not exist")
            }
        }
    }

    if let Some(s) = matches.subcommand_matches("test") {
        if s.is_present("verify") {
            pinboard.verify().unwrap();
        }
    }

    // if use_env_var {
    //     todo!("here we do stuff")
    // } else {
    //     if config.exists() {
    //         todo!("attempt to read config file settings")
    //     } else {
    //         todo!("create a blank config file")
    //     }
    // }
}
