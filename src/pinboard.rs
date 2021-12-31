extern crate log;

use std::arch;
use std::env;

use crate::pinboard::bookmark::Bookmark;
use anyhow::anyhow;
use regex::Regex;
use reqwest::blocking::Client;
use reqwest::{Method, Url};
use serde_json::Value;

pub const PINBOARD_URL: &str = "https://api.pinboard.in/v1/";
pub const PINBOARD_API: &str = "PINBOARD_API";

mod bookmark;
mod general;

enum TokenFields {
    Username,
    Password,
}

pub fn use_env_var() -> (bool, String) {
    let status: bool;
    let env_api_token = env::var(PINBOARD_API).unwrap_or("none".to_string());
    if env_api_token.to_owned().eq("none") {
        status = false;
    } else {
        status = true;
    }
    return (status, env_api_token);
}

pub struct Api {
    client: Client,
    token: String,
    update_time: String,
}

impl Api {
    pub fn new(token: String) -> Self {
        Self {
            client: Client::new(),
            update_time: String::new(),
            token,
        }
    }

    pub fn verify(&self) -> Result<(), anyhow::Error> {
        let response_json = self.create_request(Vec::new(), "user/api_token/")?;
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

    fn create_request(
        &self,
        argument_pairs: Vec<[&str; 2]>,
        endpoint_address: &str,
    ) -> Result<Value, anyhow::Error> {
        let base = reqwest::Url::parse(PINBOARD_URL)?;
        let mut url = base.join(endpoint_address)?;
        url.query_pairs_mut()
            .append_pair("auth_token", self.token.as_str());
        url.query_pairs_mut().append_pair("format", "json");
        if !argument_pairs.is_empty() {
            for n in argument_pairs {
                url.query_pairs_mut().append_pair(n[0], n[1]);
            }
        }
        let response = self.client.request(Method::GET, url.as_str()).send()?;
        let response_json: Value = serde_json::from_str(&response.text()?.as_str())?;
        Ok(response_json)
    }

    fn check_latest_update(&mut self) -> Result<(), anyhow::Error> {
        let json = self.create_request(Vec::new(), "posts/update/")?;
        self.update_time = json["update_time"].to_string();
        Ok(())
    }

    pub fn get_recent(
        &mut self,
        number_of_entries: i32,
    ) -> Result<Vec<bookmark::Bookmark>, anyhow::Error> {
        let json = self.create_request(
            Vec::from([["count", number_of_entries.to_string().as_str()]]),
            "posts/recent",
        )?;
        let json_array = json["posts"].as_array().unwrap().to_owned();
        let mut bookmarks: Vec<Bookmark> = Vec::new();
        for json_object in json_array {
            if json_object.is_object() {
                bookmarks.push(serde_json::from_value(json_object)?)
            }
        }
        Ok(bookmarks)
    }
}

#[cfg(test)]
mod test {
    use super::{use_env_var, Api};
    use rand::Rng;
    #[test]
    fn verify() {
        let (_, token_string) = use_env_var();
        let pinboard = Api::new(token_string);
        if let Ok(o) = pinboard.verify() {
            assert_eq!(o, ())
        }
    }

    #[test]
    fn recent() {
        let (_, token_string) = use_env_var();
        let mut pinboard = Api::new(token_string);
        let number_of_entries: i32 = rand::thread_rng().gen_range(0..100);
        let number_of_entries_usize = usize::try_from(number_of_entries).unwrap();
        if let Ok(o) = pinboard.get_recent(number_of_entries) {
            assert_eq!(number_of_entries_usize, o.len())
        }
    }
}
