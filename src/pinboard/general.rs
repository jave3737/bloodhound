pub trait General {
	fn new() -> Self;

	fn get_url(&self) -> String;

	fn get_tags(&self) -> Vec<String>;
}
