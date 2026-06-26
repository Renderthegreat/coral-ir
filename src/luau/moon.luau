_G.__MOON__ = {
	symbol_registry = {},
	meta_symbol_registry = {},

	-- Better proxy generator.
	meta_symbol_proxy = function()
		local proxy = {}

		for symbol, metamethod in pairs(__MOON__.meta_symbol_registry) do
			if metamethod == '__index' then
				-- Special handling to prevent recursion.
				proxy[metamethod] = function(self, key)
					-- First try raw lookup (normal fields).
					local value = rawget(self, key);
					if value ~= nil then
						return value;
					end;

					-- Then try symbol-based handler.
					local handler = rawget(self, symbol);  -- `rawget` to avoid recursion.
					if type(handler) == "function" then
						return handler(self, key);
					end;

					return nil
					-- error("attempt to index " .. tostring(self) .. " with unknown key: " .. tostring(key), 2);
				end;
			else
				-- Normal wrapper for other metamethods.
				proxy[metamethod] = function(self, ...)
					local handler = rawget(self, symbol);  -- Avoid recursive `__index`.
					if type(handler) == "function" then
						return handler(self, ...);
					end;

					return nil;
					-- error("no handler for " .. tostring(symbol) .. " on " .. tostring(self), 2);
				end;
			end;
		end;

		__MOON__.meta_symbol_proxy = proxy;
		return proxy;
	end,
}

_G.Symbol = {
	new = function (label)
		local sentinel = {}; -- Each `object` is unique.

		-- This is a private symbol, so it isn't added to the registry.
		local symbol = {
			label = label,
			sentinel = sentinel,
		};

		setmetatable(symbol, {
			['__eq'] = function (self, other)
				return self.sentinel == other.sentinel;
			end,
		});

		return symbol;
	end,

	of = function (label)
		if (__MOON__.symbol_registry[label] ~= nil) then
			return __MOON__.symbol_registry[label];
		end;

		-- This is a public symbol, so it is added to the registry.
		local symbol = Symbol.new(label);

		__MOON__.symbol_registry[label] = symbol;

		return symbol;
	end,
};

setmetatable(Symbol, {
	['__call'] = function (self, label)
		print(label);

		return Symbol.new(label);
	end,
});

_G.__MOON__.meta_symbol_registry = {
	--> Calculation operators. <--
	[Symbol.of('+')] = '__add',
	[Symbol.of('-')] = '__sub',
	[Symbol.of('*')] = '__mul',
	[Symbol.of('/')] = '__div',
	[Symbol.of('%')] = '__mod',
	[Symbol.of('//')] = '__idiv',
	[Symbol.of('^')] = '__pow',
	[Symbol.of('(-)')] = '__unm',

	--> Bitwise operators <--
	[Symbol.of('&')] = '__band',
	[Symbol.of('|')] = '__bor',
	[Symbol.of('~')] = '__bxor',
	[Symbol.of('(~)')] = '__bnot',
	[Symbol.of('<<')] = '__bshl',
	[Symbol.of('>>')] = '__bshr',

	--> Equation operators <--
	[Symbol.of('==')] = '__eq',
	[Symbol.of('<')] = '__lt',
	[Symbol.of('<=')] = '__le',

	--> General purpose operators <--
	[Symbol.of('..')] = '__concat',
	[Symbol.of('#')] = '__len',

	--> Special meta <--
	[Symbol.of('index')] = '__index',
	[Symbol.of('new_index')] = '__newindex',

	[Symbol.of('to_string')] = '__tostring',

	[Symbol.of('call')] = '__call',
	
	[Symbol.of('meta')] = '__metatable',

	[Symbol.of('pairs')] = '__pairs',
	[Symbol.of('ipairs')] = '__ipairs',

	[Symbol.of('gc')] = 'gc',
	[Symbol.of('strength')] = '__mode',

	[Symbol.of('name')] = '__name',
};

__MOON__.meta_symbol_proxy();


local foo = {
	count = 1,

	[Symbol.of('+')] = function (self, other)
		print("Perfoming my op!");
		self.count = self.count + other.count;
		
		return self;
	end,

	[Symbol.of('to_string')] = function (self, other)
		return self.count;
	end,
};


