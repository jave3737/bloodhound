extern crate log;
use anyhow::{Error,anyhow};
use clap::{App, Arg};
use log::Level::Info;
use log::{info, log_enabled};
use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;
use reqwest::{Request, StatusCode};
use reqwest::{Method, Url};
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::{env, result};
use std::io::Write;

const CONFIG_FILE: &str = "config.yaml";
const PINBOARD_URL: &str = "https://api.pinboard.in/v1";

fn verify_api_connection(client:Client, token_string:&str) -> Result<(),anyhow::Error> {
    let mut url_string = String::new();
    url_string = format!("{}/user/api_token/?auth_token={}",PINBOARD_URL,token_string);
    let url = reqwest::Url::parse(&url_string)?;
    let req = client.request(Method::GET, url);
    let res = req.send()?;
    info!("Formed api URL: {}",url_string);
    if res.status().eq(&StatusCode::OK) {
        info!("Response: {:}",res.text().unwrap());
        Ok(())
    } else {
        Err(anyhow!("Issues verifying api")) 
    }
}

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
    let status: bool;
    let env_api_token = env::var("PINBOARD_API").unwrap_or("none".to_string());

    if env_api_token.to_owned().eq("none") {
        status = false;
    } else {
        status = true;
    }

    if log_enabled!(Info) && status == true {
        info!(
            "Found api token via environment variable: {}",
            env_api_token
        );
        info!("Will NOT be using api token specified in {}", CONFIG_FILE);
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
        .get_matches();

    let mut map: HashMap<String, String> = HashMap::new();
    parse_option(&mut map, matches);

    //check if we are going to automatically use the environment variable
    let (use_env_var, token_string) = use_env_var();

    //create the file if env_statu s= false, config doenst exists, and the create config option was set
    //create_config_file(map).unwrap();

    let client = Client::new();
    verify_api_connection(client, token_string.as_str()).unwrap();
    // match verify_api_connection(client, "token_string") {
    //     Ok(_) => todo!(),
    //     Err(_) => todo!(),
    // }
}

