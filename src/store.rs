// TODO: Rename this file?
use crate::{
	architecture::{
		Register,
	},
	types::{
		Type,
	},
};

///
/// The slot that a given value can be found.
///
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Debug)]
pub enum Slot {
	/// A value stored in a register.
	Register(Register),
	/// A value stored on the stack.
	Stack(u64),
	/// A value stored on the heap.
	Heap,

	/// A value that is a immediate value, and is stored in the instruction itself.
	Immediate,

	/// A value that is stored in the static section(s).
	Static(usize),

	// The value is unused.
	#[default]
	Unused,
}

impl Slot {
	pub fn is_managed(&self) -> bool {
		return matches!(*self, Slot::Register(..) | Slot::Stack(..));
	}
}

// TODO: Documentation.
#[derive(Clone, Debug)]
pub struct Value {
	pub r#type: Type,
	pub location: Slot,
}
