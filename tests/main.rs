use ::coral_ir;

use ::logos::Logos as _;

use ::std::{
	fs,
	error::{
		Error,
	},
};

#[test]
pub fn lex<'source>() -> Result<(), Box<dyn Error>> {
	let source: &'source str = &fs::read_to_string("./tests/demo.coral")?;
	let mut lexer: logos::Lexer<'source, coral_ir::language::lexer::Token> =
		coral_ir::language::lexer::Token::lexer(source);
	let items = coral_ir::language::parser::parse::<'source>(&mut lexer)?;

	println!("{:#?}", items);

	return Ok(());
}
