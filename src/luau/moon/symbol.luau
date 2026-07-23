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
				return `Symbol.of({console:format(if self.shared then self.label else self.pointer, 0, true)})`;
			end,

			-- Without the same pointers, indexing doesn't work.
			-- So we return a reference.
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

	--> Bitwise operators. <--
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
	
	[Symbol.of('meta')] = '__metatable',

	[Symbol.of('iterator')] = '__iter',

	-- [Symbol.of('pairs')] = '__pairs',
	-- [Symbol.of('ipairs')] = '__ipairs',

	-- [Symbol.of('gc')] = '__gc',
	[Symbol.of('strength')] = '__mode',

	[Symbol.of('name')] = '__name',

	--> Moon. <--
	[Symbol.of('new')] = '__constructor',
	[Symbol.of('clone')] = '__clone',
};

__MOON__.generate_meta_symbol_proxy();