use crate::{
	items::Function,
	store::Value,
};

use ::std::sync::Arc;

#[derive(Clone, Debug)]
pub enum Instruction {
	// ...
	Call(crate::Pass<String>),

	Return(Arc<Value>),
}
