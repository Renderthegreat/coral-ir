use ::mlua;
use ::mlua_magic_macros;

#[derive(Clone)]
#[mlua_magic_macros::structure]
pub struct Coercible {
	pub(self) value: mlua::Value,
}

#[mlua_magic_macros::implementation]
impl Coercible {
	pub fn extract(&self) -> mlua::Value {
		return self.value.clone();
	}
}

/*impl mlua::UserData for Coercible {
	fn add_methods<M: mlua::prelude::LuaUserDataMethods<Self>>(methods: &mut M) -> () {

	}
	fn add_fields<F: mlua::prelude::LuaUserDataFields<Self>>(fields: &mut F) -> () {

	}
	fn register(registry: &mut mlua::prelude::LuaUserDataRegistry<Self>) -> () {

	}
}*/

mlua_magic_macros::compile!(type_path = Coercible, fields = false, methods = false, variants = false,);
