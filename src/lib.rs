#![feature(f128)]

pub mod context;

pub mod abi;
pub mod architecture;

pub mod store;

pub mod section;

pub mod types;

pub mod instructions;
pub mod items;

// pub mod operations;

// pub mod language;

// pub mod checking;

pub mod luau;

use ::std::collections::HashMap;

use ::mlua;

#[derive(Clone)]
pub struct Compiler {
	pub(crate) lua: mlua::Lua,
	pub(crate) target: Box<architecture::Architecture>,
}

impl Compiler {
	pub fn new(target: architecture::Architecture) -> mlua::Result<Self> {
		let lua: mlua::Lua = luau::create()?;

		let chunk = lua.load(include_str!("luau/test.luau"));

		chunk.set_name("test.luau").exec().unwrap();

		return Ok(Self {
			lua: lua,
			target: Box::new(target),
		});
	}

	pub fn create_context(&self) -> context::Context {
		return context::Context {
			target: self.target.clone(), // TODO: ...

			functions: HashMap::new(),
			literals: Vec::new(),
		};
	}
}

///
/// Locks data to this crate.
///
#[derive(Clone, Copy, Debug)]
pub struct Pass<T> {
	pub(self) value: T,
	pub(self) pure: bool,
}

impl<T> Pass<T> {
	pub(crate) fn new(value: T) -> Self {
		return Self {
			value: value,
			pure: true,
		};
	}

	pub fn new_unpure(value: T) -> Self {
		return Self {
			value: value,
			pure: false,
		};
	}

	pub fn get(self) -> (T, bool) {
		return (self.value, self.pure);
	}
}
