extern crate log;
use anyhow::anyhow;
use log::Level::Info;
use log::{info, log_enabled};
use reqwest::blocking::Client;
use reqwest::{Method, StatusCode};

pub const PINBOARD_URL: &str = "https://api.pinboard.in/v1";

pub struct Api {
    client: Client,
    token: String,
}

impl Api {
    pub fn new(token: String) -> Self {
        Self {
            client: Client::new(),
            token,
        }
    }

    pub fn test(&self) -> bool {
    	false
    }

    pub fn verify_api_connection(&self) -> Result<(), anyhow::Error> {
        let url_string =
            format!("{}/user/api_token/?auth_token={}", PINBOARD_URL, self.token).to_string();
        let url = reqwest::Url::parse(&url_string)?;
        let req = self.client.request(Method::GET, url);
        let res = req.send()?;
        info!("Formed api URL: {}", url_string);
        if res.status().eq(&StatusCode::OK) {
            info!("Response: {:}", res.text().unwrap());
            Ok(())
        } else {
            Err(anyhow!("Issues verifying api"))
        }
    }
}
