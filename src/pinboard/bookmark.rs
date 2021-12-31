//use super::general::*;
// use crate::pinboard::general::*;
use super::general::*;
use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct Bookmark {
	description: String,
	href: String, 
	additional_info: AdditionalInfo
}

#[derive(Debug,Serialize,Deserialize)]
pub struct AdditionalInfo {
	extended: Option<String>,
	hash: Option<String>,
	tags: Option<String>,
	time: Option<String>,
	toread: Option<String>
}

impl Default for AdditionalInfo {
	fn default() -> Self {
	    AdditionalInfo{
	    	extended: Default::default(),
	    	hash: Default::default(),
	    	tags: Default::default(),
	    	time: Default::default(),
	    	toread: Default::default()
	    }
	}
}
impl Bookmark {}

impl General for Bookmark {

	fn new(href:String, description:String) -> Self {
		Bookmark{
			href,
			description,
			additional_info: Default::default()
		}	
	}
	
	fn get_tags(&self) -> Vec<String> {
		todo!()	
	}
	
	fn get_url(&self) -> String {
		self.href.to_owned()
	}
}
