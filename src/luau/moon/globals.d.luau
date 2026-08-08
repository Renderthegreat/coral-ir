--[=[
TODO:
--]=]
type MoonInternals = {
	pointer: (value: any) -> number,
	raw_get_meta: ({ any }) -> { any }?,

	meta_symbol_map: { [Symbol]: string, },
	generate_global_symbol_map: () -> (),
	global_symbol_registry: { [Symbol]: string, },
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

	format: (self: ConsoleImpl, any, {
		max_depth: number?,
		format_strings: boolean?,
		colors_enabled: boolean?,
	}?) -> string,
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


export type Object<T> = T & {
	[Symbol]: any, -- TODO: ...
};

export type ObjectImpl = {
	from: <T>(T) -> Object<T>,
};

declare Object: ObjectImpl;


export type Class<T, P...> = {
	new: (P...) -> (),
} & Brand<'class'>;
export type Constructor<T, P...> = (T, P...) -> T;

declare function Class<T, P...>(table: { [Symbol]: Constructor<T, P...>, } & T): Class<T>;


export type Coercible<T> = {

};

-- TODO: Use `Class`.
export type CoercibleImpl = {
	new: <T>(T) -> Coercible<T>
};

declare Coercible: CoercibleImpl;


export type Pointer<T> = T & {
	__inner: T,
};

export type PointerImpl = {
	refer: <T>(T) -> Pointer<T>,
	derefer: <T>(T) -> Pointer<T>,
};

declare Pointer: PointerImpl;

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