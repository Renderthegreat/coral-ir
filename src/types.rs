use ::mlua_magic_macros;

use ::thiserror::Error;

///
/// A type error.
///
#[derive(Clone, Error, Debug)]
#[mlua_magic_macros::enumeration]
pub enum TypeError {
	#[error("Cannot dereference `{0}` because it is not a reference type.")]
	InvalidDereference(Type),
}

mlua_magic_macros::compile!(type_path = TypeError, variants = true);

// TODO: Documentation.
///
/// Represents data.
///
#[derive(Clone, Debug)]
#[mlua_magic_macros::enumeration]
pub enum Type {
	Integer(u64, bool),
	Float(u64),
	Vector(u64),

	Reference(Box<[Self]>),

	/// An unknown type.
	/// This is used when the type cannot be determined, and is not important to determine.
	/// Unlike `Any`, this type doesn't intersect with any other types.
	Unknown(),
	/// A wildcard type that can be any type.
	///
	/// Be warned however that this should only be used when you know the type that you want to use, because it can lead to compile-time errors if used incorrectly.
	Any(),
}

#[mlua_magic_macros::implementation]
impl Type {
	pub fn dereference(&self) -> Result<Type, TypeError> {
		return match self {
			Self::Reference(inner) => Ok((*inner)[0].clone()),
			_ => Err(TypeError::InvalidDereference(self.clone())),
		};
	}

	pub fn reference(&self) -> Type {
		return Type::Reference(Box::new([self.clone()]));
	}
}

// TODO: Replace when `logos` is used.
impl ::std::fmt::Display for Type {
	fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
		match &self {
			&&Self::Integer(size, signed) => {
				let prefix: char = if signed { 'i' } else { 'u' };
				write!(f, "{}{}", prefix, size)
			},
			&&Self::Float(size) => write!(f, "f{}", size),
			// TODO: SIMD.
			&&Self::Vector(size) => write!(f, "v{}", size),

			&&Self::Reference(ref size) => write!(f, "&{}", size[0]),

			&&Self::Any() => write!(f, "*"),

			&&Self::Unknown() => write!(f, "?"),
		}
	}
}

mlua_magic_macros::compile!(type_path = Type, variants = true, methods = true);
