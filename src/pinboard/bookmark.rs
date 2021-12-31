//use super::general::*;
// use crate::pinboard::general::*;
use super::general::*;
use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct Bookmark {
	description: String,
	extended: String, 
	hash: String, 
	href: String, 
	tags: String, 
	time: String,
	toread: String,
}

impl Bookmark {}

impl General for Bookmark {

	fn new() -> Self {
		todo!();
	}
	
	fn get_tags(&self) -> Vec<String> {
		todo!()	
	}
	
	fn get_url(&self) -> String {
		self.href.to_owned()
	}
}
