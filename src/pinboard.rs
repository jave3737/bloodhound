extern crate log;
use anyhow::anyhow;
use log::Level::Info;
use log::{info, log_enabled};
use regex::Regex;
use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;
use reqwest::{Method, StatusCode, Url};
use serde_json::{Value, json};

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

    pub fn verify_api_connection(&self) -> Result<(), anyhow::Error> {
        let url = self.create_url("user/api_token/")?;
        let request = self.client.request(Method::GET, url.as_str());
        let response = request.send()?;
        let response_text = response.text()?;
        let json_parse: Value = serde_json::from_str(&response_text.as_str())?;
        let password = &json_parse["result"].to_string();
        let password_token = self.extract_from_token(TokenFields::Password)?;
        let strip_alpha_numberic = Regex::new("[^A-Za-z0-9]")?;
        let stripped_password = strip_alpha_numberic.replace_all(password, "");
         if stripped_password.eq(&password_token){
             Ok(())
         } else {
             Err(anyhow!("Issue")) 
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
