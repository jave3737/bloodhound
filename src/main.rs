use std::io::Write;
use clap::{App,Arg};
use std::collections::BTreeMap;

const CONFIG_FILE: &str = "config.yaml";

fn main() {

    let matches = App::new("Bloodhound")
        .version("1.0")
        .author("Alejandro Miranda <alejandro.miranda@hey.com>")
        .about("Dig through pinboard bookmarks")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .takes_value(true)
                .help("Create a config.yaml that stores your api token")
        )
        .get_matches();

    if let Some(res) = matches.value_of("config") {
        println!("Creating/Overwriting {}", CONFIG_FILE);
        match std::fs::File::create(CONFIG_FILE)
        {
            Ok(mut o) => {
                let mut map = BTreeMap::new();
                map.insert("api".to_string(), res);
                let serial_map = serde_yaml::to_string(&map).expect("Error");
                o.write_all(serial_map.as_bytes()).expect("Error");
            }
            Err(e) => {
                eprintln!("Error creating {} due to {}",CONFIG_FILE, e);
            }
        }
        }
    if !std::path::Path::new(CONFIG_FILE).exists() {
        eprintln!("No {} found! You must create it first using the config option",CONFIG_FILE);
    }
}
