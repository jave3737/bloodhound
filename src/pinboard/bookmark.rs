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
	// url : String,
	// tags : Vec<String>,
	// date : String,
	// title : String,
}

impl Bookmark {}

impl General for Bookmark {
	// add code here
	fn new() -> Self {
		todo!();
	}
	fn list(&self) -> Vec<String>{
		todo!();
	}

	fn add(&self) -> bool {
		todo!();
	}

	fn del(&self) -> bool {
		todo!();
	}
}
