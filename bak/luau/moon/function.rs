use ::mlua;

pub fn environment(lua: &mlua::Lua, function: mlua::Function) -> mlua::Result<mlua::Table> {
	return function.environment().ok_or(mlua::Error::external("TODO"));
}
