--[=[
TODO:
]=]
function Class<T, P...>(value: { [Symbol]: Constructor<T, P...>?, } & T): Class<T>
	setmetatable(value :: any, __MOON__.meta_symbol_proxy);

	local meta: { [Symbol]: Constructor<T, P...>?, } = value;
	local prototype: { } = clone(value);

	-- Remove the constructor from the prototype.
	prototype[Symbol.of('new')] = nil;
	-- ...

	return {
		new = function<P...> (...: P...): T
			local constructor = expect(meta[Symbol.of('new')]);

			local run = function (...)
				return constructor(clone(prototype) --[[ Another clone for the `prototype` for each instance. ]], ...);
			end;

			return run(...);
		end,

		['__brand'] = 'class',
	};
end;