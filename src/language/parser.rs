use crate::language::lexer::{
	Token,
};

use crate::language::parser::ParseError::UnknownSyntax;
use crate::types::Type;

use ::std::{
	error::{
		Error,
	},
};

use ::logos::{
	self,
	Logos as _,
};

use ::thiserror;

#[derive(Clone, Debug)]
pub struct Position {
	pub line: u64,
	pub column: u64,
}

impl ToString for Position {
	fn to_string(&self) -> String {
		return format!("{0}:{1}", self.line, self.column);
	}
}

#[derive(Clone, Debug)]
pub enum Item {
	Function(Function),
}

#[derive(Clone, Debug)]
pub struct Function {
	pub name: String,
	pub parameters: Vec<(String, Type)>,
	pub return_type: Type,
	pub body: Vec</* TODO: `Statement` */ ()>,
}

#[derive(Clone, Debug)]
pub struct PropertyAccess {}

#[derive(thiserror::Error, Default, Debug)]
pub enum ParseError<'source> {
	#[error("Unexpected end of file.")]
	UnexpectedEOF,
	#[error("Unexpected token: '{0:?}'.")]
	UnexpectedToken(Token<'source>),

	#[default]
	#[error("An unknown syntax error has occurred")]
	UnknownSyntax,
}

macro_rules! match_for_token {
	// For unit enums with no extra arguments.
	($lexer:expr, $token_type:ident) => {
		{
			let token = $lexer.next();
			match token {
				Some(Ok(Token::$token_type)) => (),
				Some(Ok(token)) => return Err(Box::new(ParseError::UnexpectedToken(token))),
				Some(Err(error)) => return Err(Box::new(error)),
				None => return Err(Box::new(ParseError::UnexpectedEOF)),
			}
		}
	};

	// For tuple enums with one or more arguments.
	($lexer:expr, $token_type:ident, $first_item:ident $(, $item:ident)*) => {
		{
			match $lexer.next() {
				Some(Ok(Token::$token_type($first_item $(, $item)*))) => ($first_item $(, $item)*),
				Some(Ok(token)) => return Err(Box::new(ParseError::UnexpectedToken(token))),
				Some(Err(error)) => return Err(Box::new(error)),
				None => return Err(Box::new(ParseError::UnexpectedEOF)),
			}
		}
	};
}

macro_rules! match_for_path {
	// TODO: ...
	($lexer:expr) => {{
		match $lexer.next() {
			Some(Ok(Token::Path(path))) => path,
			Some(Ok(Token::Identifier(path))) => path,
			Some(Ok(token)) => return Err(Box::new(ParseError::UnexpectedToken(token))),
			Some(Err(error)) => return Err(Box::new(error)),
			None => return Err(Box::new(ParseError::UnexpectedEOF)),
		}
	}};
}

macro_rules! trailing_comma {
	($lexer:expr) => {
		match $lexer.next() {
			Some(Ok(Token::Comma)) => true,
			Some(Ok(token)) => return Err(Box::new(ParseError::UnexpectedToken(token))),
			Some(Err(error)) => return Err(Box::new(error)),
			None => false,
		}
	};
}

pub fn parse<'source>(lexer: &mut logos::Lexer<'source, Token<'source>>) -> Result<Vec<Item>, Box<dyn Error + 'source>> {
	let mut items: Vec<Item> = Vec::new();

	while let Some(Ok(token)) = lexer.next() {
		items.push(match token {
			Token::Whitespace => continue,
			Token::CommentSingle(_) | Token::CommentMulti(_) => continue,

			// 'fn'.
			Token::Function => {
				let name = match_for_token!(lexer, Identifier, name);

				let mut parameters_lexer = Token::lexer(match match_for_token!(lexer, CurvedBrackets, source) {
					Some(source) => source,
					None => return Err(Box::new(ParseError::UnknownSyntax)),
				});

				let mut parameters: Vec<(String, Type)> = Vec::new();

				loop {
					if parameters_lexer.remainder().trim_start().is_empty() {
						break;
					};

					let name = match_for_token!(parameters_lexer, Identifier, name);
					match_for_token!(parameters_lexer, Colon);
					let r#type = match_for_path!(parameters_lexer);

					parameters.push((name, Type::try_from(r#type)?));

					if !trailing_comma!(parameters_lexer) {
						break;
					};
				}

				match_for_token!(lexer, Arrow);

				let return_type = match_for_path!(lexer);

				println!(
					r"
						name: {},
						params: {:?},
						returns: {}
					",
					name, parameters, return_type
				);

				todo!();
			},

			// '$identifier'.
			Token::Path(path) | Token::Identifier(path) => {
				let property_access = todo!();
			},

			// TODO:
			_ => todo!("Parsing for token `{token:?}` is not implemented yet."),
		});
	}

	if let Some(Err(error)) = lexer.next() {
		println!("{}", error);
		return Err(Box::new(error));
	};

	return Ok(items);
}
