local TODO = "Foo" .. 0XC0FFEE;

local special = {
	count = 1,
	[Symbol.of('+')] = function (self, other)
		print(self, other);
		self.count += other.count;
		return self;
	end,
};

setmetatable(special, __MOON__.meta_symbol_proxy)

print((special + special).count);
