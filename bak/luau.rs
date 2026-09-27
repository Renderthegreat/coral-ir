pub(crate) mod hacks;
pub mod moon;
pub mod permissive;

use ::mlua;

///
/// Create a new *Lua* instance for the *Coral* IR.
///
/// This function will panic if *Lua* isn't installed, but it should be installed on the system when *Coral* is installed.
///
pub(crate) fn create() -> mlua::Result<mlua::Lua> {
	// TODO: Make this safe.
	let mut lua = unsafe {
		mlua::Lua::unsafe_new_with(mlua::StdLib::DEBUG | mlua::StdLib::ALL_SAFE, mlua::LuaOptions::default())
		//.expect("*Lua* might not be installed on this system.")
	};

	// TODO: ...

	moon::modernize(&mut lua)?;

	return Ok(lua);
}
