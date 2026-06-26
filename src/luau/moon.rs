//!
//! # Moon standards
//!
//! A bunch of utils that makes *Lua* much more favorable.
//! I hope this gets adopted to *real* *Lua* implementations.
//! For now this is just a niche implementation by yours truely.
//! :D
//!

use crate::luau::hacks::{
	extract_lua_from_string,
};

use ::std::collections::HashMap;

use ::mlua::{
	self,
	ObjectLike as _,
};
use ::mlua_magic_macros;

use ::rand;
///
/// Modernize the *Lua* instance with *Moon*.
///
pub(crate) fn modernize(lua: &mut mlua::Lua) -> mlua::Result<()> {
	let moon = lua.create_table()?;

	let globals = lua.globals();

	let meta = lua.create_table()?;

	let index_hook = lua.create_function(move |lua, (context_table, value): (mlua::Value, mlua::Value)| {
		println!("{}", value.to_string()?);

		let key: String = lua.globals().get_path::<mlua::Table>("_G.__MOON__.meta_symbol_registry")?.get(value)?;

		// https://github.com/mlua-rs/mlua/blob/8338b1daac2e71f9a9c24768bd9d2af4270001bf/src/userdata.rs

		return match key.as_str() {
			// TODO: ...

			// Old names:
			"tostring" => lua.globals().get("tostring"),
			_ => Ok(mlua::Nil),
		};
	})?;

	meta.set("__index", index_hook)?;

	// Set all primitive types to use modern meta-data.
	// ...

	/*let mut symbol_registry: HashMap<&str, Symbol> = HashMap::new();
	symbol_registry.insert("to_string", Symbol::new(Some("to_string".to_string())));
	mlua_magic_macros::load!(lua, Symbol);
	moon.set("symbol_registry", symbol_registry)?;*/

	globals.set("__MOON__", moon)?;

	// TODO: Update to use a console.
	let print: mlua::prelude::LuaFunction = lua.create_function(move |lua, value: mlua::Value| {
		println!("{:?}", value);

		return Ok(());
	})?;
	globals.set("print", print)?;

	let chunk = lua.load(include_str!("moon.luau"));

	chunk.set_name("moon.luau").exec().unwrap();

	return Ok(());
}

// === UTILS ===

pub(self) fn to_string(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<mlua::String> {
	let tostring: mlua::Function = lua.globals().get("tostring")?;

	return match value {
		::mlua::Value::Table(table) => {
			todo!();
		},

		::mlua::Value::Nil => lua.create_string("nil"),

		// TODO: Determine appropriate behaviour.
		_ => lua.create_string("???"),
	};
}

/*// === LUA MECHANICS ===

#[derive(Clone)]
#[mlua_magic_macros::structure]
pub struct Symbol {
	pub(self) id: u128,
	pub label: Option<String>,
}

#[mlua_magic_macros::implementation]
impl Symbol {
	pub fn new(label: Option<String>) -> Self {
		return Self {
			id: rand::random(),
			label: label,
		};
	}

	pub fn of(string: mlua::String) -> mlua::Result<Self> {
		let lua = extract_lua_from_string(string.clone())?;

		let label = string.to_str()?.to_string();

		println!("{:#?}", lua.globals().get_path::<mlua::Value>("__MOON__.symbol_registry"));

		let symbol_registry: mlua::Table = lua.globals().get_path("__MOON__.symbol_registry")?;

		return if let Some(symbol) = symbol_registry.get::<Option<Self>>(label.clone())? { Ok(symbol) } else { Ok(Self::new(Some(label))) };
	}

	pub fn __foreward_eq(self: &Self, other: Self) -> bool {
		return Self::eq(self, &other);
	}

	pub fn __foreward_ne(self: &Self, other: Self) -> bool {
		return Self::ne(self, &other);
	}
}

impl PartialEq for Symbol {
	fn eq(self: &Self, other: &Self) -> bool {
		println!("Got here...");
		return self.id == other.id;
	}

	fn ne(self: &Self, other: &Self) -> bool {
		return self.id != other.id;
	}
}

mlua_magic_macros::compile!(type_path = Symbol, fields = true, methods = true);
*/
