extern crate log;
use anyhow::anyhow;
use log::Level::Info;
use log::{info, log_enabled};
use reqwest::blocking::Client;
use reqwest::{Method, StatusCode, Url};
use serde_json::Value;

pub const PINBOARD_URL: &str = "https://api.pinboard.in/v1/";

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

    pub fn verify_api_connection(&self) -> Result<(), anyhow::Error> {
        let url = self.create_url("user/api_token/")?;
        let request = self.client.request(Method::GET, url.as_str());
        let response = request.send()?;
        let response_text = response.text()?;
        let json_parse: Value = serde_json::from_str(&response_text.as_str())?;
        println!("{:?}", json_parse["result"]);
        todo!()
    }

    pub fn create_url(&self, endpoint_address: &str) -> Result<Url, anyhow::Error> {
        let base = reqwest::Url::parse(PINBOARD_URL)?;
        let mut url = base.join(endpoint_address)?;
        url.query_pairs_mut().append_pair("auth_token", self.token.as_str());
        url.query_pairs_mut().append_pair("format", "json");
        Ok(url)
    }
}
