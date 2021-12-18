extern crate log;
use anyhow::anyhow;
use log::Level::Info;
use log::{info, log_enabled};
use reqwest::blocking::Client;
use reqwest::{Method, StatusCode};

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
    	let base = reqwest::Url::parse(PINBOARD_URL)?;
		let mut url = base.join("user/api_token/")?;
    	url.query_pairs_mut().append_pair("auth_token", self.token.as_str());
    	url.query_pairs_mut().append_pair("format", "json");
        let req = self.client.request(Method::GET, url.as_str());
        let res = req.send()?;
        println!("{:?}", res.text()?);
        // let value = serde_json::from_str(res.text()?.as_slice())?;
        // println!("{:?}", v["result"]);
        todo!()
        // info!("Formed api URL: {}", url_string);
        // if res.status().eq(&StatusCode::OK) {
        //     info!("Response: {:}", res.text().unwrap());
        //     Ok(())
        // } else {
        //     Err(anyhow!("Issues verifying api"))
        // }
    }
}
