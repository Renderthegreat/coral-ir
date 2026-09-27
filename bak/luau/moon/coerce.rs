use ::mlua::{
	self,
	MetaMethod,
	UserDataMethods,
};
use ::mlua_magic_macros;

#[derive(Clone)]
#[mlua_magic_macros::structure]
pub struct Coercible {
	pub(self) value: mlua::Value,
}

#[mlua_magic_macros::implementation]
impl Coercible {
	pub fn new(value: mlua::Value) -> mlua::Result<Self> {
		return Ok(Self {
			value: value,
		});
	}
}

impl mlua::UserData for Coercible {
	fn register(registry: &mut mlua::UserDataRegistry<Self>) {
		registry.add_meta_method(MetaMethod::Add, |lua: &mlua::Lua, self_: &Self, other: mlua::Value| -> mlua::Result<Self> {
			dbg!(self_.clone().value, other);

			return todo!();
		});

		Self::_to_mlua_fields(registry);
		Self::_to_mlua_methods(registry);
	}
}

impl mlua::FromLua for Coercible {
	fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
		let output: mlua::Result<Self> = match value {
			mlua::Value::UserData(user_data) => {
				return match user_data.borrow::<Self>() {
					Ok(b) => Ok((*b).clone()),
					Err(_) => {
						Err(mlua::Error::FromLuaConversionError {
							from: "UserData",
							to: String::from("Coercible"),
							message: Some("userdata is not this exact Rust type".into()),
						})
					},
				};
			},
			_ => {
				Err(mlua::Error::FromLuaConversionError {
					from: value.type_name(),
					to: String::from("Coercible"),
					message: Some("expected userdata created by mlua_magic_macros".into()),
				})
			},
		};

		return output;
	}
}
