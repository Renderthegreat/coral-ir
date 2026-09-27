use crate::context::Block;

use ::std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Section<'a> {
	/// TODO: ...
	/// `.text`
	Blocks(HashMap<String, Block<'a>>),
	/// Stores static data for the program.
	/// `.data`
	Static(Box<[u8]>),
	/// Section for uninitialized variables.
	/// `.bss`
	Uninitialized(u128),
}

impl<'a> Section<'a> {
	// pub fn to_assembly(&self) -> String {
	// 	let mut assembly: String = String::new();

	// 	// Section header.
	// 	assembly.push_str("section ");
	// 	assembly.push_str(match self {
	// 		Self::Blocks(..) => ".text",
	// 		Self::Static(..) => ".data",
	// 		Self::Uninitialized(..) => ".bss",
	// 	});

	// 	match self {
	// 		Self::Blocks(blocks) => todo!(),
			
	// 	};

	// 	// ...

	// 	return assembly;
	// }
}