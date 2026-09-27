use crate::{
	abi::ABI,
	context::{
		Block,
		BlockResult,
	},
	store,
	types,
};

use ::std::sync::Arc;

pub type BlockFunction = fn(block: &mut Block) -> BlockResult<()>;

#[derive(Clone, Debug)]
pub struct Function {
	pub abi: Option<Arc<ABI>>,

	pub name: String,

	pub parameters: Vec<types::Type>,
	pub bindings: Option<Vec<Arc<store::Value>>>,
	pub block_function: BlockFunction,
	pub returns: types::Type,
}

impl Function {
	///
	/// Checks if the [`Function`] is independent.
	/// This property is useful because it allows us make optimizations.
	///
	pub fn is_independent(&self) -> bool {
		return self.bindings.is_none();
	}
}
