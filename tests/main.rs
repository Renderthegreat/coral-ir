use ::coral_ir::{
	self,
	architecture::{
		Architecture,
		Endianness,
		Register,
		RegisterKind,
		RegisterSet,
	},
	items::*,
	store::Slot,
	types,
};

use ::std::sync::{
	LazyLock,
	Arc,
};

use ::std::{
	error::{
		Error,
	},
};

const DEMO_ABI: coral_ir::abi::ABI = coral_ir::abi::ABI {
	name: "DEMO",

	determine_slot: |state, r#type| {
		// TODO: We are being really lazy...
		return Slot::Heap;
	},

	determine_block_layout: |state, parameter_types| {
		let mut layout: Vec<Slot> = Vec::new();

		for parameter_type in parameter_types {
			let slot = (DEMO_ABI.determine_slot)(state, parameter_type);

			layout.push(slot);
		}

		return layout;
	},
};

static ARM64: LazyLock<Architecture> = LazyLock::new(|| {
	Architecture {
		name: "ARM64".to_string(),
		endianness: Endianness::Big,
		abi: Arc::new(DEMO_ABI),
		register_set: RegisterSet {
			general_registers: Vec::from([]),
			float_registers: Vec::from([]),
			flag_registers: Vec::from([]),
			vector_registers: Vec::from([]),
		},
	}
});

#[test]
pub fn instance() -> Result<(), Box<dyn Error>> {
	let target = ARM64.clone();

	let compiler = coral_ir::Compiler::new(target)?;

	let mut context = compiler.create_context();

	let message = context.add_literal(Literal::String(Box::from("Hello, World!\n")));

	let my_function = context.add_function(Function {
		abi: None,

		name: String::from("_start"),

		parameters: Vec::from([]),
		bindings: Option::Some(Vec::from([message])),
		block_function: |block| {
			let unk1 = block.get_parameter(0)?;

			dbg!(&unk1);

			block.close(unk1)?;

			return Ok(());
		},
		returns: types::Type::Float(64),
	});

	dbg!(&my_function);

	dbg!(context.evaluate()?);

	return Ok(());
}
