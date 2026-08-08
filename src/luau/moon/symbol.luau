--> TODO: Replace with `const` when possible!
local Symbol = {
	new = function (label: string)
		local sentinel: {} = {}; -- Each `object` is unique.

		-- This is a private symbol, so it isn't added to the registry.
		local symbol = {
			label = label,
			sentinel = sentinel,
			pointer = nil,
			shared = false,
		};

		symbol.pointer = _G.__MOON__.pointer(symbol.sentinel);

		setmetatable(symbol, {
			['__eq'] = function (self, other): boolean
				return self.sentinel == other.sentinel;
			end,

			-- TODO: Add `display` directive.
			['__tostring'] = function (self): string
				-- TODO: Choose correct version.
				return `Symbol.of({console:format(if self.shared then self.label else self.pointer, { max_depth = 0, format_strings = true, })})`;
			end,

			-- Without the same pointers, indexing doesn't work.
			-- So, we return a reference.
			['__clone'] = function (self): Symbol
				return self;
			end,
		});

		return symbol;
	end,
};

function Symbol.of(label: string)
	if (_G.__MOON__.symbol_registry[label] ~= nil) then
		return _G.__MOON__.symbol_registry[label];
	end;

	-- This is a public symbol, so it is added to the registry.
	local symbol = Symbol.new(label);

	symbol.shared = true;

	_G.__MOON__.symbol_registry[label] = symbol;

	return symbol;
end;

_G.Symbol = Symbol;

setmetatable(Symbol, {
	-- TODO: Remove? This only adds another way to make a symbol.
	['__call'] = function (self, label)
		return Symbol.new(label);
	end,
});

-- TODO: Allow custom registries!
_G.__MOON__.global_symbol_registry = {
	--> Calculation operators. <--
	[Symbol.of('+')] = '__add',
	[Symbol.of('-')] = '__sub',
	[Symbol.of('*')] = '__mul',
	[Symbol.of('/')] = '__div',
	[Symbol.of('%')] = '__mod',
	[Symbol.of('//')] = '__idiv',
	[Symbol.of('^')] = '__pow',
	[Symbol.of('(-)')] = '__unm',

	--> Bitwise operators. <--
	[Symbol.of('(~)')] = '__bnot',
	-- Not supported in *Luau*.
	--[[
	[Symbol.of('&')] = '__band',
	[Symbol.of('|')] = '__bor',
	[Symbol.of('~')] = '__bxor',
	[Symbol.of('(~)')] = '__bnot',
	[Symbol.of('<<')] = '__bshl',
	[Symbol.of('>>')] = '__bshr',
	--]]

	--> Equation operators. <--
	[Symbol.of('==')] = '__eq',
	[Symbol.of('<')] = '__lt',
	[Symbol.of('<=')] = '__le',

	--> General purpose operators. <--
	[Symbol.of('..')] = '__concat',
	[Symbol.of('#')] = '__len',

	--> Special meta. <--
	[Symbol.of('index')] = '__index',
	[Symbol.of('new_index')] = '__newindex',

	[Symbol.of('to_string')] = '__tostring',
	[Symbol.of('to_debug_string')] = '__todebugstring',

	[Symbol.of('call')] = '__call',
	
	[Symbol.of('meta_table')] = '__metatable',

	[Symbol.of('iterator')] = '__iter',

	-- [Symbol.of('gc')] = '__gc',
	[Symbol.of('strength')] = '__mode',

	[Symbol.of('type')] = '__type',

	--> Moon. <--
	[Symbol.of('new')] = '__constructor',
	[Symbol.of('clone')] = '__clone',
};

-- _G.__MOON__.generate_global_symbol_proxy = function ()
-- 	local map = {};

-- 	for symbol, meta_method in pairs(_G.__MOON__.global_symbol_registry) do
-- 		if meta_method == '__index' or meta_method == '__newindex' then
-- 			-- Special handling to prevent recursion.
-- 			map[meta_method] = function (self, key)
-- 				-- First try raw lookup (normal fields).
-- 				local value = rawget(self, key);
-- 				if value ~= nil then
-- 					return value;
-- 				end;

-- 				-- Then try symbol-based handler.
-- 				local handler = rawget(self, symbol); -- `rawget` to avoid recursion.
-- 				if typeof(handler) == 'function' then
-- 					return handler(self, key);
-- 				end;

-- 				return nil;
-- 				-- error("attempt to index " .. tostring(self) .. " with unknown key: " .. tostring(key), 2); -- TODO: Do we want this?
-- 			end;
-- 		elseif meta_method == '__clone' then
-- 			map[meta_method] = nil;
-- 		else
-- 			-- Normal wrapper for other meta-methods.
-- 			map[meta_method] = function (self, ...)
-- 				console:warn(symbol);
-- 				local handler = rawget(self, symbol); -- Avoid recursive `__index`.
-- 				console:warn(handler);

-- 				if typeof(handler) == 'function' then
-- 					return handler(self, ...);
-- 				end;

-- 				-- return nil;
-- 				error(`No handler for \`{meta_method}\`.`);
-- 			end;
-- 		end;
-- 	end;

-- 	print(map);

-- 	_G.__MOON__.meta_symbol_map = map;
-- 	return map;
-- end;

-- __MOON__.generate_global_symbol_proxy();