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

const TARGET: LazyLock<coral_ir::architecture::Architecture> = LazyLock::new(|| {
	coral_ir::architecture::Architecture {
		endianness: coral_ir::architecture::Endianness::Big(),
		name: "ARM64".to_string(),
	}
});

#[test]
pub fn instance() -> Result<(), Box<dyn Error>> {
	let target = TARGET.clone();

	let instance = coral_ir::Instance::new(target);

	return Ok(());
}

#[test]
pub fn lex() -> Result<(), Box<dyn Error>> {
	let mut lexer: logos::Lexer<'_, coral_ir::language::lexer::Token> = coral_ir::language::lexer::Token::lexer(SOURCE);
	// let items = coral_ir::language::parser::parse(&mut lexer)?;

	// println!("{:#?}", items);

	return Ok(());
}
