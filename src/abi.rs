use crate::{
	context::Block,
	store::Slot,
	types::Type,
};

///
/// # The ABI (application binary interface).
///
/// TODO: ...
#[derive(Clone, Debug)]
pub struct ABI {
	pub name: &'static str,

	/// TODO: ...
	pub determine_block_layout: fn(&Block, parameter_types: &Vec<Type>) -> Vec<Slot>,
	/// TODO: ...
	pub determine_slot: fn(&Block, &Type) -> Slot,
}
