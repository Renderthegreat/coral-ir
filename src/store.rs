// TODO: Rename this file?
use crate::architecture::{
	Register,
};

use crate::types::{
	Type,
};

use ::mlua_magic_macros;

///
/// The place that the data can be found.
///
#[derive(Clone, Default, Debug)]
#[mlua_magic_macros::enumeration]
pub enum Location {
	Register(Register),
	// Stack(u64 /* 16 *Exobytes*. */),
	/// The value is an immediate value, and is stored in the instruction itself.
	///
	/// Example:
	/// ```coral
	/// foo = 5; // `5` is the immediate value.
	/// ```
	Immediate,

	#[default]
	Unknown,
}

mlua_magic_macros::compile!(type_path = Location, variants = true);

// TODO: Documentation.
#[derive(Clone, Debug)]
#[mlua_magic_macros::structure]
pub struct Value {
	pub r#type: Type,
	pub location: Location,
}

mlua_magic_macros::compile!(type_path = Value, fields = true);
