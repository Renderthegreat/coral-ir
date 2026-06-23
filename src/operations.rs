use crate::store::{
	Value,
	Location,
};
use crate::types::{
	Type,
};

use crate::architecture::{
	Register,
};

use crate::checking::{
	Checkable,
};

///
/// Any operation.
///
pub trait Operation: ::core::fmt::Debug + Checkable {
	fn get_operand(&self, n: u8) -> Option<Value>;
	fn get_result(&self) -> Option<Value>;
}

#[derive(Clone, Debug)]
pub struct Add(pub Value, pub Value);

impl Checkable for Add {
	fn check(&self) -> Result<(), Box<dyn ::core::error::Error>> {
		return Ok(());
	}
}

impl Operation for Add {
	fn get_operand(&self, n: u8) -> Option<Value> {
		return match n {
			1 => Some(self.0.clone()),
			2 => Some(self.1.clone()),
			_ => None,
		};
	}

	fn get_result(&self) -> Option<Value> {
		// TODO: Make this configurable!
		return Some(Value {
			r#type: Type::Integer(64, true),
			location: Location::Register(Register::General(0)),
		});
	}
}
