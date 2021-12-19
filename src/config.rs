use anyhow::anyhow;
use std::path::Path;

pub struct Config;

const CONFIG_FILE: &str = "config.yaml";

impl Config {
    pub fn new() -> Self {
        //todo!()
        Self{}
    }

    pub fn config_file_exists(&self) -> bool {
    	Path::new(CONFIG_FILE).exists()
    }

    pub fn create_config_file(&self) -> Result<(), anyhow::Error> {
    	match std::fs::File::create(CONFIG_FILE) {
        Ok(o) => Ok(()),
        Err(e) => Err(anyhow!("Error creating {} due to {}",CONFIG_FILE,e))
    	}
    }
}
