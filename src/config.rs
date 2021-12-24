use anyhow::anyhow;
use std::{path::Path, io::Write};
use serde::{Serialize,Deserialize};
use std::collections::BTreeMap;

#[derive(Debug,Serialize,Deserialize)]
pub struct ConfigYaml{
    api:String
}

pub struct Config{
    config_yaml:ConfigYaml
}

const CONFIG_FILE: &str = "config.yaml";

impl Config {
    pub fn new() -> Self {
        Self{
            config_yaml: ConfigYaml{
                api:String::new()
            } 
        }
    }

    pub fn load(&self) -> bool {
        todo!()
    }

    pub fn exists(&self) -> bool {
    	Path::new(CONFIG_FILE).exists()
    }

    pub fn create_blank(&self) -> Result<(), anyhow::Error> {
    	match std::fs::File::create(CONFIG_FILE) {
        Ok(mut o) => {
            let mut yaml = BTreeMap::new();
            yaml.insert("api", "<Paste API Token Here>");
            let yaml_serialize = serde_yaml::to_string(&yaml)?;
            o.write_all(yaml_serialize.as_bytes())?;
            Ok(())
        } ,
        Err(e) => Err(anyhow!("Error creating {} due to {}",CONFIG_FILE,e))
    	}
    }

    pub fn update_token(&self) -> Result<(), anyhow::Error> {
        todo!()
    }
}
