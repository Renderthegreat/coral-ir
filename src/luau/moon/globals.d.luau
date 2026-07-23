--[=[
TODO:
--]=]
type MoonInternals = {
	pointer: (value: any) -> number,
	meta_symbol_proxy: { [Symbol]: string, },
	generate_meta_symbol_proxy: () -> (),
};
--[=[
Moon internals.
]=]
declare __MOON__: MoonInternals;


--[=[
Returns a clone of the given value.
]=]
declare function clone<T>(value: T, clone_meta: boolean?): T;


--[=[
Asserts that a value is not nil.
]=]
declare function expect<T>(value: T?): T --[[TODO]];


type ConsoleImpl = {
	log: (self: ConsoleImpl, ...any) -> (),
	warn: (self: ConsoleImpl, ...any) -> (),
	error: (self: ConsoleImpl, ...any) -> (),

	-- TODO: `depth`.
	format: (self: ConsoleImpl, any, depth: number? --[[TODO: Option<T>]], format_strings: boolean?) -> string,
};

declare console: ConsoleImpl;


--[=[
TODO:
]=]
export type Symbol = {
	label: string,
	sentinel: {},
};

type SymbolImpl = {
	new: (label: string) -> Symbol,
	of: (label: string) -> Symbol
} & (label: string) -> Symbol;

declare Symbol: SymbolImpl;


export type Class<T, P...> = {
	new: (P...) -> T,
} & Brand<'class'>;
export type Constructor<T, P...> = (T, P...) -> T;

declare function Class<T, P...>(table: { [Symbol]: Constructor<T, P...>, } & T): Class<T>;


--[[
export type function _(): type


	return types.boolean;
end;

export type function Index(table, index)

	return index(table, index);
end;
--]]

export type Brand<T> = {
	__brand: T,
};

--[==[
Exports
--]=]
return {
	Symbol = Symbol,

	console = console,
	class = class,
}
--]==]