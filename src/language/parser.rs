use crate::language::lexer::{
	Token,
};

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
	($lexer:expr, $token_type:ident, $($item:ident),*) => {
		{
			let token = $lexer.next();
			match token {
				Some(Ok(Token::$token_type($($item),*))) => ($($item),*),
				// TODO: Add positional data.
				Some(Ok(token)) => return Err(Box::new(ParseError::UnexpectedToken(token))),
				Some(Err(error)) => return Err(Box::new(error)),
				None => return Err(Box::new(ParseError::UnexpectedEOF)),
			}
		}
	}
}

pub fn parse<'source>(
	lexer: &mut logos::Lexer<'source, Token<'source>>,
) -> Result<Vec<Item>, Box<dyn Error + 'source>> {
	let items: Vec<Item> = Vec::new();

	while let Some(Ok(token)) = lexer.next() {
		match token {
			Token::Whitespace => continue,
			Token::CommentSingle(_) | Token::CommentMulti(_) => continue,

			Token::Function => {
				let name = match_for_token!(lexer, Identifier, name);
				println!("{:#?}", name);

				let parameters_lexer =
					Token::lexer(match match_for_token!(lexer, CurvedBrackets, source) {
						Some(source) => source,
						None => return Err(Box::new(ParseError::UnknownSyntax)),
					});

				let parameters: Vec<(String, Type)> = Vec::new();

				loop {}

				todo!();
			},

			// TODO:
			_ => todo!("Parsing for token `{token:?}` is not implemented yet."),
		}
	}

	if let Some(Err(error)) = lexer.next() {
		println!("{}", error);
		return Err(Box::new(error));
	};

	return Ok(items);
}
