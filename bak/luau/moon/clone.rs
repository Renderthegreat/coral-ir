use std::collections::HashMap;

use mlua::{
	Function,
	Lua,
	Result,
	Table,
	Value,
};

// TODO: Add options!
pub fn clone(lua: &Lua, (value, ignore_method): (Value, Option<bool>)) -> Result<Value> {
	let ignore_method = ignore_method.unwrap_or(false);

	let mut seen = HashMap::<usize, Table>::new();

	return clone_impl(lua, value, &mut seen, ignore_method);
}

fn clone_impl(lua: &Lua, value: Value, seen: &mut HashMap<usize, Table>, ignore_method: bool) -> Result<Value> {
	Ok(match value {
		Value::Buffer(buffer) => Value::Buffer(lua.create_buffer(buffer.to_vec())?),

		// Immutable / shared values.
		error @ Value::Error(_) => error,
		user_data @ Value::UserData(_) => user_data,

		Value::Function(function) => Value::Function(function.deep_clone()?),

		#[allow(clippy::clone_on_copy)]
		Value::Vector(vector) => Value::Vector(vector.clone()),

		Value::Table(table) => {
			let pointer = table.to_pointer() as usize;

			// Already cloned?
			if let Some(existing) = seen.get(&pointer) {
				return Ok(Value::Table(existing.clone()));
			};

			// Create placeholder first so cyclic references work.
			let cloned = lua.create_table()?;
			seen.insert(pointer, cloned.clone());

			// Custom clone meta-method.
			if let Some(meta_table) = table.metatable() {
				if !ignore_method && let Ok(method) = meta_table.get::<Function>("__clone") {
					let result = method.call::<Value>(table.clone())?;

					// Update the cache with the returned table.
					if let Value::Table(ref new_table) = result {
						seen.insert(pointer, new_table.clone());
					};

					return Ok(result);
				};
			};

			// Clone every key/value pair.
			for pair in table.pairs::<Value, Value>() {
				let (key, value) = pair?;

				let key = clone_impl(lua, key, seen, false)?;
				let value = clone_impl(lua, value, seen, false)?;

				cloned.set(key, value)?;
			}

			// Clone the metatable.
			if let Some(meta) = table.metatable() {
				// TODO: Should we use `false`?
				if let Value::Table(new_meta) = clone_impl(lua, Value::Table(meta), seen, false)? {
					cloned.set_metatable(Some(new_meta))?;
				};
			};

			Value::Table(cloned)
		},

		Value::Thread(_) => {
			return Err(mlua::Error::RuntimeError("Cannot clone threads".into()));
		},

		// Other primitives are handled here.
		// We can just return them since they are not stored by reference.
		primitive => primitive,
	})
}
