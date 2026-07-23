use ::std::collections::{
	HashMap,
};

use ::mlua::{
	self,
	Value,
	MultiValue,
};

pub fn clone(lua: &mlua::Lua, (value, mut seen): (Value, Option<HashMap<usize, mlua::Table>>)) -> mlua::Result<mlua::Value> {
	let mut binding = HashMap::new();
	let seen: &mut HashMap<usize, mlua::Table> = seen.as_mut().unwrap_or(&mut binding);

	return Ok(match value {
		Value::Buffer(buffer) => {
			let byte_vec = buffer.to_vec();

			Value::Buffer(lua.create_buffer(byte_vec)?)
		},
		// Don't clone since it would not make since to do so.
		error @ Value::Error(_) => error,
		Value::Function(function) => Value::Function(function.deep_clone()?),
		Value::Table(table) => {
			let table_pointer = table.to_pointer() as usize;

			if let Some(cloned_table) = seen.get(&table_pointer) {
				return Ok(Value::Table(cloned_table.clone()));
			};

			let cloned_table = lua.create_table()?;
			seen.insert(table_pointer, cloned_table.clone());

			if let Some(meta_table) = table.metatable()
				&& let Ok(meta_method_value) = meta_table.get::<Value>(String::from("__clone"))
				&& let Some(meta_method) = meta_method_value.as_function()
			{
				let mut multi_value = MultiValue::new();

				multi_value.push_back(Value::Table(table));

				// TODO: IMPORTANT!!! THIS BREAKS WHEN PARAMS ARE WRONG!
				// /!\ /!\ /!\ \\

				return if let Ok(cloned) = meta_method.call::<Value>(multi_value) {
					Ok(cloned)
				} else {
					Err(mlua::Error::RuntimeError(String::from("The `__clone` meta-method is present, but calling it failed.")))
				};
			};

			// Deep clone pairs recursively.
			for pairs in table.pairs::<Value, Value>() {
				let (key, value) = pairs?;
				let cloned_key = clone(lua, (key, Some(seen.clone())))?;
				let cloned_value = clone(lua, (value, Some(seen.clone())))?;
				cloned_table.set(cloned_key, cloned_value)?;
			}

			// Handle metatables safely.
			if let Some(metatable) = table.metatable() {
				let cloned_metatable = clone(lua, (Value::Table(metatable), Some(seen.clone())))?;
				if let Value::Table(mt) = cloned_metatable {
					cloned_table.set_metatable(Some(mt))?;
				};
			};

			Value::Table(cloned_table)
		},
		Value::Thread(thread) => {
			todo!();
		},
		user_data @ Value::UserData(_) => user_data,
		#[allow(clippy::clone_on_copy)] // Since `Vector` might be updated.
		Value::Vector(vector) => Value::Vector(vector.clone()),

		// Other primitives automatically get cloned.
		_ => value,
	});
}
