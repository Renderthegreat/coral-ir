local object_meta = {};

for symbol, key in pairs(__MOON__.global_symbol_registry) do
	object_meta[key] = function (self, ...)
		local method: ((...any) -> any)? = rawget(self, symbol);

		-- print(key, symbol);

		local meta = __MOON__.raw_get_meta(self);

		-- TODO: Decide what should be used.
		if method == nil and meta[key] ~= nil and key ~= '__clone' then
			method = meta[key];
		end;

		if method == nil then
			if key == '__index' then
				return nil;
			elseif key == '__newindex' then
				local set_key, set_value = ...;
				rawset(self, set_key, set_value);

				return nil; -- TODO: Check if this is right.
			elseif key == '__clone' then
				return clone(self, true);
			elseif key == '__tostring' then
				return console:format(self, {
					colors_enabled = false,
				});
			end;

			for key, value in pairs(self) do
				print(key, value);
			end;

			return error(`No handler for \`{key}\`!`);
		end;

		return method(self, ...);
	end;
end;

_G.Object = {
	from = function (base): Object<{}>
		local new_base = clone(base); -- TODO: ...

		-- TODO: Change to `set_meta_table`.
		setmetatable(new_base, object_meta);

		return new_base;
	end,
};