//use super::general::*;
// use crate::pinboard::general::*;
use super::general::*;
struct Bookmark {
	url : String,
	tags : Vec<String>,
	date : String,
	title : String,
}

impl Bookmark {}

impl General for Bookmark {
	// add code here
	fn new() -> Self {
		todo!();
	}
	fn list(&self) -> bool{
		todo!();
	}

	fn add(&self) -> bool {
		todo!();
	}

	fn del(&self) -> bool {
		todo!();
	}
}
