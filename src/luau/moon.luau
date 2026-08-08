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
};

_G.__MOON__ = merge(_G.__MOON__, __MOON);

--[=[
TODO: Remove!
]=]
local Coercible = {

};