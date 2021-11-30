use std::fs;
use clap::{App, Arg};
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
        println!("Creating config file");
        if let Err(err) = std::fs::File::create(CONFIG_FILE) {
            eprintln!("Error creating {} due to {}",CONFIG_FILE, err);
        }
    } 
}
