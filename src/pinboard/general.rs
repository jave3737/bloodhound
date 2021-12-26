pub trait General {
	fn new() -> Self;

	fn list(&self) -> bool;

	fn add(&self) -> bool;

	fn del(&self) -> bool;
}
