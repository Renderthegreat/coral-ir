-- TODO: require(path.to.module);

local function runTests()
	local passed = 0;
	local failed = 0;

	local function expect(condition, message)
		if condition then
			passed += 1;
		else
			failed += 1;
			print("❌ Test Failed: " .. (message or "Assertion failed"));
		end;
	end;

	-- TEST 1: Primitive types should pass through unchanged.
	do
		expect(clone(42) == 42, "Should clone numbers");
		expect(clone("hello") == "hello", "Should clone strings");
		expect(clone(true) == true, "Should clone booleans");
		expect(clone(nil) == nil, "Should handle nil");
	end;

	-- TEST 2: Basic table cloning (Shallow check).
	do
		local original = { a = 1, b = 2, };
		local copy = clone(original);

		expect(copy ~= original, "Cloned table must have a different reference");
		expect(copy.a == 1 and copy.b == 2, "Cloned table must have identical values");
	end;

	-- TEST 3: Deep table cloning (Nested tables).
	do
		local original = { nested = { data = "value" } };
		local copy = clone(original);

		print(copy, original);

		expect(copy.nested ~= original.nested, "Nested tables must be deeply cloned");
		expect(copy.nested.data == "value", "Nested table data must be preserved");
	end;

	-- TEST 4: Mixed keys (Arrays and Dictionaries).
	do
		local original = { "apple", "banana", nested = { true } };
		local copy = clone(original);

		expect(copy[1] == "apple" and copy[2] == "banana", "Array parts must clone correctly");
		expect(copy.nested[1] == true, "Nested array parts must clone correctly");
	end;

	-- TEST 5: Metatable preservation.
	do
		local mt = { __index = { secret = 100 } };
		local original = setmetatable({ data = 5 }, mt);
		local copy = clone(original);

		expect(__MOON__.pointer(getmetatable(copy)) ~= __MOON__.pointer(mt), "Metatable reference must not be preserved");
		expect(copy.secret == 100, "Metatable behavior must work on clone");
	end;

	-- TEST 6: Cyclic/Recursive references (Prevents infinite loops).
	do
		local original = { name = "Node" };
		original.self = original; -- Self reference.

		local success, copy = pcall(clone, original);

		expect(success == true, "Should handle cyclic references without crashing");
		if success then
			expect(copy.self == copy, "Cyclic reference should point to the clone, not the original");
		end;
	end;

	print(string.format("\n📋 Test Results: %d Passed, %d Failed", passed, failed));
end;

local Bar = Class({
	count = 0,
	
	[Symbol.of('new')] = function (self, number: number): ()
		self.count = number;
	end,

	increment = function (self): number
		self.count += 1;

		return self.count;
	end,

	[Symbol.of('+')] = function (self, other)
		local new_self = clone(self);

		new_self.count = self.count + other.count;

		return new_self;
	end,
});


-- TODO: Remove...
-- local a = Coercible.new("ABC");
-- local b = Coercible.new(0xC0FFEE);

local bar_a = Bar.new(1);
local bar_b = Bar.new(1);

print(bar_a, bar_b);

-- print(getmetatable(bar_a));

print(bar_a + bar_b);

local object = Object.from({
	[Symbol.of('+')] = function (self, other)
		return 1;
	end,
});
-- object[Symbol.of('+')] = function (self, other)
--	return 1;
-- end;

-- TODO: Remove `fenv`.

-- print(object);

-- print(object + object);