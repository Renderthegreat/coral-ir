#![feature(type_info)]

pub mod architecture;

pub mod store;
pub mod types;

pub mod operations;

pub mod language;

pub mod checking;

pub mod luau;

use ::mlua;

#[derive(Clone)]
pub struct Instance {
	pub(crate) lua: mlua::Lua,
	pub(crate) target: architecture::Architecture,
}

impl Instance {
	pub fn new(target: architecture::Architecture) -> mlua::Result<Instance> {
		let lua: mlua::Lua = luau::create()?;

		let chunk = lua.load(include_str!("luau/test.luau"));

		chunk.set_name("test.lua").exec().unwrap();

		return Ok(Instance {
			lua: lua,
			target: target,
		});
	}
}
