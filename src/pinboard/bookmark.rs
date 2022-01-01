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
struct BookmarkBuilder {
    url: String,
    description: String,
    bookmark_builder_optional: BookmarkBuilderOptional
}

impl BookmarkBuilder {
	fn new(url: String, description: String) -> Self {
		Self{
			url,
			description,
			bookmark_builder_optional: Default::default()
		}
	}
}

#[derive(Debug)]
struct BookmarkBuilderOptional {
    extended: String,
    tags: Vec<String>,
    dt: String,
    replace: bool,
    shared: bool,
    toread: bool,
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
