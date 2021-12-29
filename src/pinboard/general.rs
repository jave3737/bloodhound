pub trait General {
	fn new() -> Self;

	fn list(&self) -> Vec<String>;

	fn add(&self) -> bool;

	fn del(&self) -> bool;
}
