use crate::{
	abi::ABI,
	types::Type,
};

use ::enum_kinds;

use ::std::sync::Arc;

#[derive(Clone, Copy, Debug)]
pub enum Endianness {
	Big,
	Little,
}

///
/// Represents a register that stores data.
///
#[derive(enum_kinds::EnumKind, Clone, Copy, Eq, PartialEq, Hash, Debug)]
#[enum_kind(RegisterKind)]
pub enum Register {
	General(usize),
	Float(usize),
	// Flag(!), // TODO: ...
	/// (number, size).
	Vector(usize, u64),
}

impl Register {
	pub fn kind(&self) -> RegisterKind {
		return self.into();
	}
}

impl RegisterKind {
	pub fn register_type(r#type: &Type) -> Option<Self> {
		return Some(match r#type {
			Type::Integer(..) | Type::Reference(..) | Type::List(..) => Self::General,
			Type::Float(..) => Self::Float,

			// TODO: Tuple optimizations, ...
			_ => return None,
		});
	}
}

/// TODO: ...
#[derive(Clone, Debug)]
pub enum VectorRegister {
	Bx(), //   8 bits.
	Hx(), //  16 bits.
	Sx(), //  32 bits.
	Dx(), //  64 bits.
	Qx(), // 128 bits.
	Vx(), // 128 bits.
}

///
/// Information about the target architecture.
/// Includes information such as endianness, and bit count.
///
#[derive(Clone, Debug)]
pub struct Architecture {
	pub name: String,

	pub abi: Arc<ABI>,
	pub register_set: RegisterSet,

	pub endianness: Endianness,
}

#[derive(Clone, Debug)]
pub struct RegisterSet {
	pub general_registers: Vec<Register>,
	pub float_registers: Vec<Register>,
	pub flag_registers: Vec<Register>, // TODO: ...
	pub vector_registers: Vec<Register>,
}

impl RegisterSet {
	pub fn get(&self, register_kind: &RegisterKind, index: usize) -> Option<Register> {
		return match *register_kind {
			RegisterKind::General => self.general_registers.get(index).copied(),
			RegisterKind::Float => self.float_registers.get(index).copied(),
			RegisterKind::Vector => self.vector_registers.get(index).copied(),
		};
	}
}
