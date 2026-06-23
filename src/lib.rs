#![feature(hash_map_macro)]
pub mod architecture;

pub mod store;
pub mod types;

pub mod operations;

pub mod language;

pub mod checking;

use ::mlua;

pub struct Instance<'lua> {
	pub(crate) lua: &'lua mlua::Lua,
	pub(crate) target: architecture::Architecture,
}

impl<'lua> Instance<'lua> {
	pub fn new(target: architecture::Architecture) -> Instance<'lua> {
		todo!("Instance::new is not implemented yet.");
	}
}
