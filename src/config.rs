use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use serde_json::Serializer;
use std::collections::BTreeMap;
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
    pub fn create_blank(&self) -> Result<(), anyhow::Error> {
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

    pub fn load(&self) -> bool {
        todo!()
    }

    pub fn new() -> Self {
        Self {
            config_yaml: ConfigYaml { api: String::new() },
        }
    }

    pub fn update(&self, token: &str) -> Result<(), anyhow::Error> {
        let file = std::fs::File::open(CONFIG_FILE).unwrap();
        let d: String = serde_yaml::from_reader(file).unwrap();
        println!("{}",d);
        todo!()
    }
}
