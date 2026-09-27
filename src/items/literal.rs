use crate::{
	architecture::Endianness,
	items,
	types,
};

use ::core::mem::{
	size_of,
	size_of_val,
};

///
/// Represents static data that is stored in the binary.
///
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Literal {
	String(Box<str>),
	Char(char),

	Integer(u128),
	Float(u128),
	// TODO: Add support for a new number type?
}

impl Literal {
	pub fn size(&self) -> usize {
		return match self {
			Self::String(string) => size_of_val(&**string),
			Self::Char(..) => size_of::<char>(),

			Self::Integer(..) => size_of::<i128>(),
			Self::Float(..) => size_of::<f128>(),
		};
	}

	pub fn as_bytes(&self, endianness: Endianness) -> Box<[u8]> {
		return match self {
			Self::Char(x) => Box::new([*x as u8]),
			Self::String(x) => Box::from(x.as_bytes()),

			Self::Float(x) => {
				Box::new({
					let x = f128::from_bits(*x);
					if matches!(endianness, Endianness::Little) { x.to_le_bytes() } else { x.to_be_bytes() }
				})
			},
			Self::Integer(x) => Box::new(if matches!(endianness, Endianness::Little) { x.to_le_bytes() } else { x.to_be_bytes() }),
		};
	}

	pub fn get_type(&self) -> types::Type {
		return match self {
			Self::String(..) => types::Type::Reference(Box::new(types::Type::Char())),
			Self::Char(..) => types::Type::Char(),

			_ => todo!(),
		};
	}
}
