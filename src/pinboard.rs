extern crate log;
use anyhow::anyhow;
use log::Level::Info;
use log::{info, log_enabled};
use regex::Regex;
use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;
use reqwest::{Method, StatusCode, Url};
use serde_json::{json, Value};

pub const PINBOARD_URL: &str = "https://api.pinboard.in/v1/";

enum TokenFields {
    Username,
    Password,
}

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

    pub fn verify(&self) -> Result<(), anyhow::Error> {
        let url = self.create_url("user/api_token/")?;
        let response = self.client.request(Method::GET, url.as_str()).send()?;
        let response_json: Value = serde_json::from_str(&response.text()?.as_str())?;
        let password_json = &response_json["result"].to_string();
        let password_token = self.extract_from_token(TokenFields::Password)?;
        let password_stripped = Regex::new("[^A-Za-z0-9]")?.replace_all(&password_json, "");
        if password_stripped.eq(&password_token) {
            println!("No problem verifying token with Pinboard.in API");
            Ok(())
        } else {
            Err(anyhow!("Issue verifying token with Pinboard.in API"))
        }
    }

    fn extract_from_token(&self, fields: TokenFields) -> Result<String, anyhow::Error> {
        let mut result = String::new();
        let regex_pattern = Regex::new(r"(?P<username>^.+):(?P<password>.+$)")?;
        let captures = regex_pattern.captures(self.token.as_str()).unwrap();
        match fields {
            TokenFields::Username => result.push_str(&captures["username"]),
            TokenFields::Password => result.push_str(&captures["password"]),
        }
        Ok(result)
    }

    fn create_url(&self, endpoint_address: &str) -> Result<Url, anyhow::Error> {
        let base = reqwest::Url::parse(PINBOARD_URL)?;
        let mut url = base.join(endpoint_address)?;
        url.query_pairs_mut()
            .append_pair("auth_token", self.token.as_str());
        url.query_pairs_mut().append_pair("format", "json");
        Ok(url)
    }
}
