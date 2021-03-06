use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::io::Read;
use std::{io::Write, path::Path};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigYaml {
    api: String,
}

pub struct Config {
    config_yaml: ConfigYaml,
}

const CONFIG_FILE: &str = "config.yaml";

impl Config {
    pub fn create(&self) -> Result<(), anyhow::Error> {
        if !self.exists() {
            match std::fs::File::create(CONFIG_FILE) {
                Ok(mut o) => {
                    let mut yaml = BTreeMap::new();
                    yaml.insert("api", "<Paste API Token Here>");
                    let yaml_serialize = serde_yaml::to_string(&yaml)?;
                    o.write_all(yaml_serialize.as_bytes())?;
                    Ok(())
                }
                Err(e) => Err(anyhow!("Error creating {} due to {}", CONFIG_FILE, e)),
            }
        } else {
            Err(anyhow!("{} already exists", CONFIG_FILE))
        }
    }

    pub fn exists(&self) -> bool {
        Path::new(CONFIG_FILE).exists()
    }

    pub fn load(&mut self) -> Result<(),anyhow::Error> {
        let mut file = std::fs::File::open(CONFIG_FILE)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        self.config_yaml = serde_yaml::from_str(&buffer)?;
        Ok(())
    }

    pub fn get_token(&mut self) -> String {
        self.config_yaml.api.to_owned()
    }

    pub fn new() -> Self {
        Self {
            config_yaml: ConfigYaml { api: String::new() },
        }
    }
}
