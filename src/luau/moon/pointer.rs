use ::mlua;
use ::mlua_magic_macros;

#[derive(Clone, Debug)]
// #[mlua_magic_macros::structure]
pub struct Pointer {
	value: mlua::Value,
}

#[mlua_magic_macros::implementation]
impl Pointer {
	pub fn refer(value: mlua::Value) -> Self {
		return Self {
			value: value,
		};
	}

	pub fn derefer(pointer: Self) -> mlua::Value {
		return pointer.value;
	}
}

/*
impl mlua::UserData for Pointer {
	fn register(registry: &mut mlua::UserDataRegistry<Self>) -> () {
		Self::_to_mlua_methods(registry);

		registry.add_meta_method(mlua::MetaMethod::ToString, |lua: &mlua::Lua, pointer: &Self, _: ()| {
			return Ok(pointer.value.clone());
		});
	}
}
*/

mlua_magic_macros::compile!(type_path = Pointer, fields = false, methods = true, variants = false);
