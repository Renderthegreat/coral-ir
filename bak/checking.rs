pub trait Checkable {
	fn check(&self) -> Result<(), Box<dyn ::core::error::Error>>;
}
