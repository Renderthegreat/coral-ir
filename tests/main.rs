use ::coral_ir;

use ::logos::Logos as _;

use ::std::sync::{
	LazyLock,
};

use ::std::{
	error::{
		Error,
	},
};

const SOURCE: &str = include_str!("demo.coral");

static TARGET: LazyLock<coral_ir::architecture::Architecture> = LazyLock::new(|| {
	coral_ir::architecture::Architecture {
		endianness: coral_ir::architecture::Endianness::Big(),
		name: "ARM64".to_string(),
	}
});

#[test]
pub fn instance() -> Result<(), Box<dyn Error>> {
	let target = TARGET.clone();

	let instance = coral_ir::Compiler::new(target);

	return Ok(());
}

#[test]
pub fn full() -> Result<(), Box<dyn Error>> {
	let mut top_scope = coral_ir::language::scope::Scope::default();

	let mut lexer: logos::Lexer<'_, coral_ir::language::lexer::Token> = coral_ir::language::lexer::Token::lexer(SOURCE);
	let items = coral_ir::language::parser::parse(&mut lexer, &mut top_scope)?;

	println!("{:#?}", items);

	return Ok(());
}
