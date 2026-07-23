use crate::{
	types::Type,
	store::Value,
};

#[derive(Clone, Debug)]
pub struct Function {
	pub name: String,

	pub parameters: Vec<(String, Type)>,
	pub return_type: Type,

	pub body: Vec</* TODO: `Statement` */ ()>,
}

#[derive(Clone, Debug)]
pub struct Constant {
	pub name: String,

	pub r#type: Type,
	pub value: Value,
}

#[derive(Clone, Debug)]
pub struct LuauBlock {
	pub source: String,
}

impl LuauBlock {
	pub fn new(source: String) -> Self {
		return Self {
			source: source,
		};
	}
}
