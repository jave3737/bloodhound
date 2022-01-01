//use super::general::*;
// use crate::pinboard::general::*;
use super::general::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Bookmark {
    description: String,
    href: String,
    extended: String,
    hash: String,
    tags: String,
    time: String,
    toread: String,
}

#[derive(Debug)]
pub struct BookmarkBuilder {
    url: String,
    description: String,
    bookmark_builder_optional: BookmarkBuilderOptional
}

#[derive(Debug)]
struct BookmarkBuilderOptional {
    extended: String,
    tags: String,
    dt: String,
    replace: bool,
    shared: bool,
    toread: bool,
}


impl BookmarkBuilder {
	pub fn new(url: String, description: String) -> Self {
		Self{
			url,
			description,
			bookmark_builder_optional: Default::default()
		}
	}
	pub fn set_toread(&mut self,toread: bool) {
		self.bookmark_builder_optional.toread = toread
	}

	pub fn set_shared(&mut self, shared: bool) {
		self.bookmark_builder_optional.shared = shared
	}

	pub fn set_replace(&mut self, replace:bool) {
		self.bookmark_builder_optional.replace = replace
	}

	pub fn set_extended(&mut self, extended:String) {
		self.bookmark_builder_optional.extended = extended
	}

	pub fn set_dt(&mut self, dt: String){
		self.bookmark_builder_optional.dt = dt
	}

	fn set_tags(&mut self, tags:Vec<String>){
		let mut buffer = String::new();
		 for n in tags{
		 	buffer.push_str(n.as_str())
		 }
		 self.bookmark_builder_optional.tags = buffer
	}
}

impl Default for BookmarkBuilderOptional {
    fn default() -> BookmarkBuilderOptional {
        BookmarkBuilderOptional {
            extended: Default::default(),
            tags: Default::default(),
            dt: Default::default(),
            replace: Default::default(),
            shared: Default::default(),
            toread: Default::default(),
        }
    }
}

impl Bookmark {}

impl General for Bookmark {
    fn get_tags(&self) -> Vec<String> {
        todo!()
    }

    fn get_url(&self) -> String {
        self.href.to_owned()
    }
}
