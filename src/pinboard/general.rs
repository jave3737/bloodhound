pub trait General {
	fn new(href: String, description: String) -> Self;

	fn get_url(&self) -> String;

	fn get_tags(&self) -> Vec<String>;
}
