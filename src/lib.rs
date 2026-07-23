#![feature(type_info)]
#![feature(bstr)]

pub mod architecture;

pub mod store;
pub mod types;

pub mod operations;

pub mod language;

pub mod checking;

pub mod luau;

use ::mlua;

#[derive(Clone)]
pub struct Compiler {
	pub(crate) lua: mlua::Lua,
	pub(crate) target: architecture::Architecture,
}

impl Compiler {
	pub fn new(target: architecture::Architecture) -> mlua::Result<Self> {
		let lua: mlua::Lua = luau::create()?;

		let chunk = lua.load(include_str!("luau/test.luau"));

		chunk.set_name("test.luau").exec().unwrap();

		return Ok(Self {
			lua: lua,
			target: target,
		});
	}
}
