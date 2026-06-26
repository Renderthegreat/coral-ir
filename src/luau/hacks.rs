use ::core::{
	mem::transmute,
};

use ::mlua::{
	self,
	ObjectLike as _,
};

///
/// This is perhaps the most devious thing I've ever written.
/// This uses multiple dirty tricks to get the `Lua` instance from any `String`.
///
pub(crate) fn extract_lua_from_string(string: mlua::String) -> mlua::Result<mlua::Lua> {
	let value_ref = if false {
		// To get the compiler to automatically set the type to `ValueRef`, since it is `pub(crate)`..
		// This block never actually runs.
		match mlua::Value::String(string.clone()) {
			// We steal it here.
			mlua::Value::Other(vref) => vref,
			_ => unsafe { std::hint::unreachable_unchecked() },
		}
	} else {
		unsafe { transmute(string) }
	};

	// This is extremely dangerous...
	// DO NOT TREAT THIS AS A NORMAL TABLE.
	let table: mlua::Table = unsafe { transmute([value_ref]) };

	// This function call is ok since it is compiled to a static function instead of being a property.
	let weak_lua = table.weak_lua();

	return weak_lua.try_upgrade().ok_or(mlua::Error::external("Could not upgrade `WeakLua`."));
}
