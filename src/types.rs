use ::strum::Display;
use ::thiserror;

///
/// A type error.
///
#[derive(Clone, thiserror::Error, Debug)]
pub enum TypeError {
	#[error("Cannot dereference `{0}` because it is not a reference type.")]
	InvalidDereference(Type),
}

// TODO: Documentation.
///
/// Represents data.
///
#[derive(Clone, PartialEq, Eq, Display, Debug)]
pub enum Type {
	Integer(u64, bool),
	Float(u64),
	Char(),
	Vector(u64),

	Reference(Box<Self>),

	List(Box<Self>, u128),

	Tuple(Vec<Box<[Self]>>),

	// /// TODO: A magical type that uses *Lua* for typing.
	// Magical(),
	/// An unknown type.
	/// This is used when the type cannot be determined, and is not important to determine.
	/// Unlike `Any`, this type doesn't intersect with any other types.
	Unknown(),
	// /// A wildcard type that can be any type.
	// ///
	// /// Be warned however that this should only be used when you know the type that you want to use, because it can lead to compile-time errors if used incorrectly.
	// /// This only works at **compile time**.
	// Any(),
}

impl Type {
	pub fn dereference(&self) -> Result<Self, TypeError> {
		return match self {
			Self::Reference(inner) => Ok(*inner.clone()),
			_ => Err(TypeError::InvalidDereference(self.clone())),
		};
	}

	pub fn reference(&self) -> Self {
		return Self::Reference(Box::new(self.clone()));
	}
}
