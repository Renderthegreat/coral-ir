---!strict

-- TODO: Move!
local merge = function (self, other)
	for key, value in pairs(other) do
		self[key] = value;
	end;

	return self;
end;

-- TODO: Freeze!
local __MOON = {
	symbol_registry = {},
	meta_symbol_registry = {},

	--> Better proxy generator. <--
	generate_meta_symbol_proxy = function()
		local proxy = {};

		for symbol, meta_method in pairs(_G.__MOON__.meta_symbol_registry) do
			if meta_method == '__index' or meta_method == '__newindex' then
				-- Special handling to prevent recursion.
				proxy[meta_method] = function(self, key)
					-- First try raw lookup (normal fields).
					local value = rawget(self, key);
					if value ~= nil then
						return value;
					end;

					-- Then try symbol-based handler.
					local handler = rawget(self, symbol); -- `rawget` to avoid recursion.
					if typeof(handler) == 'function' then
						return handler(self, key);
					end;

					return nil;
					-- error("attempt to index " .. tostring(self) .. " with unknown key: " .. tostring(key), 2); -- TODO: Do we want this?
				end;
			else
				-- Normal wrapper for other meta-methods.
				proxy[meta_method] = function(self, ...)
					local handler = rawget(self, symbol); -- Avoid recursive `__index`.
					if typeof(handler) == 'function' then
						return handler(self, ...);
					end;

					-- return nil;
					error(`No handler for \`{meta_method}\`.`);
				end;
			end;
		end;

		_G.__MOON__.meta_symbol_proxy = proxy;
		return proxy;
	end,
};

_G.__MOON__ = merge(_G.__MOON__, __MOON);

--[=[
TODO:
]=]
local Coercible = {

};
