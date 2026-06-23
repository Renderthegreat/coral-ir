use ::mlua_magic_macros;

///
/// Represents a register that stores data.
///
#[derive(Clone, Debug)]
#[mlua_magic_macros::enumeration]
pub enum Register {
	/// The nth general purpose register.
	General(u8),
	/// The nth float register.
	Float(u8),

	// TODO: ...
	/// ???
	Vector(u8, VectorRegister),
}

mlua_magic_macros::compile!(type_path = Register, variants = true,);

#[derive(Clone, Debug)]
#[mlua_magic_macros::enumeration]
pub enum VectorRegister {
	Bx(), //   8 bits.
	Hx(), //  16 bits.
	Sx(), //  32 bits.
	Dx(), //  64 bits.
	Qx(), // 128 bits.
	Vx(), // 128 bits.
}
mlua_magic_macros::compile!(type_path = VectorRegister, variants = true);

#[derive(Clone, Debug)]
#[mlua_magic_macros::enumeration]
pub enum Endianness {
	Big(),
	Little(),
}

mlua_magic_macros::compile!(type_path = Endianness, variants = true);

///
/// Information about the target architecture.
/// Includes information such as endianness, and bit count.
///
#[derive(Clone, Debug)]
#[mlua_magic_macros::structure]
pub struct Architecture {
	pub name: String,
	pub endianness: Endianness,
}
