extern crate log;
use clap::{App, Arg};
use log::Level::Info;
use log::{info, log_enabled};
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::env;
use std::io::Write;


const CONFIG_FILE: &str = "config.yaml";

fn parse_option(map: &mut HashMap<String, String>, matches: clap::ArgMatches) {
    if let Some(s) = matches.value_of("config") {
        let temp = s.to_owned();
        map.insert("config".to_string(), temp);
    }

    if log_enabled!(Info) {
        for (key, val) in map.iter() {
            info!("Option: {} Value: {}", key, val);
        }
    }
}

fn create_config_file(map: HashMap<String, String>) -> Result<bool, anyhow::Error> {
    let mut bt_map = BTreeMap::new();
     bt_map.insert(ToString::to_string(&"api"), map.contains_key("config"));
     let mut file = std::fs::File::create(CONFIG_FILE)?;
     let serial_map = serde_yaml::to_string(&bt_map)?;
     file.write_all(serial_map.as_bytes())?;
    Ok(true)
}

fn use_env_var() -> (bool, String) {
    let mut status = false;
    let env_api_token = env::var("PINBOARD_API").unwrap_or("none".to_string());

    if env_api_token.to_owned().eq("none") {
        status = false;
    } else {
        status = true;
    }

    if log_enabled!(Info) && status == true {
        info!("Found api token via environment variable: {}", env_api_token);
        info!("Will NOT be using api token specified in {}", CONFIG_FILE);
    }

    return (status, env_api_token);
}

fn main() {
    env_logger::init();

    let  matches = App::new("Bloodhound")
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
        .get_matches();

    let mut map: HashMap<String, String> = HashMap::new();
    parse_option(&mut map, matches);

    //check if we are going to automatically use the environment variable
    let (_env_status, _env_string) = use_env_var();

    create_config_file(map).unwrap();

}
