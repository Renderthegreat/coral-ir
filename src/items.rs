pub(self) mod function;
pub(self) mod literal;

pub use self::{
	function::*,
	literal::*,
};

use crate::{
	store,
};

// TODO: Documentation.

#[derive(Clone, Debug)]
pub struct Constant {
	pub value: store::Value,
}

#[derive(Clone, Debug)]
pub struct Static {
	pub value: store::Value,
}
