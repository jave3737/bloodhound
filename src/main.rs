use clap::{App, Arg};
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::env;
use std::io::Write;
use std::path::Path;
use crate::pinboard::*;

mod pinboard;

const CONFIG_FILE: &str = "config.yaml";

fn create_config_file(file: &str) -> Result<(), anyhow::Error> {
    if Path::new(file).exists() == true {
        println!("{} already exists, will not overwrite", CONFIG_FILE);
    } else {
        std::fs::File::create(file)?;
    }
    Ok(())
}

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
    if let Some(s) = matches.value_of("config") {
        if let Err(e) = create_config_file(CONFIG_FILE){
            eprintln!("Issue creating {} due to {}",CONFIG_FILE,e);
        }
        //add api token to config
    }

    let pinboard = Api::new(token_string);
    if matches.is_present("test") {
        pinboard.verify_api_connection();
    }
}
