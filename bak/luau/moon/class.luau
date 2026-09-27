--[=[
TODO:
]=]
function Class<T, P...>(value: { [Symbol]: Constructor<T, P...>?, } & T): Class<T>
	

	-- print(getmetatable(value), __MOON__.meta_symbol_map);

	local meta: { [Symbol]: Constructor<T, P...>?, } = value;
	local prototype: { } = Object.from(value); -- No need to clone since `Object.from` makes a clone.

	-- Remove the constructor from the prototype.
	prototype[Symbol.of('new')] = nil;
	-- ...

	return {
		new = function<P...> (...: P...): T
			local constructor = expect(meta[Symbol.of('new')]);

			local instance = clone(prototype); -- Another clone to ensure `prototype` is not mutated.

			local run = function (...)
				return constructor(instance, ...);
			end;

			run(...);

			return instance;
		end,

		['__brand'] = 'class',
	};
end;