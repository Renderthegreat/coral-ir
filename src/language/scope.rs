use crate::{
	language::features::*,
	luau,
};

use ::std::collections::{
	HashMap,
};

use ::mlua;

pub enum Item {
	Function(Function),
	Constant(Constant),

	LuauBlock(LuauBlock),
}

pub struct Scope {
	pub items: HashMap<String, Item>,

	pub lua: mlua::Lua,
}

impl Default for Scope {
	fn default() -> Self {
		return Self {
			items: HashMap::new(),

			lua: luau::create().unwrap(),
		};
	}
}
