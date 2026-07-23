use ::logos::{
	self,
};

use ::thiserror;

#[derive(Clone, Default, thiserror::Error, PartialEq, Debug)]
pub enum LexError {
	// #[error("Invalid token: '{0}'.")]
	// InvalidToken(String),
	#[error("Failed to parse integer.")]
	IntegerParseError,
	#[error("Failed to parse float.")]
	FloatParseError,

	#[error("Failed to unescape string: '{0}'.")]
	UnescapeError(String),

	#[default]
	#[error("An unknown error occurred during lexing.")]
	UnknownError,
}

impl From<::std::num::ParseIntError> for LexError {
	fn from(_: ::std::num::ParseIntError) -> Self {
		return Self::IntegerParseError;
	}
}

impl From<::std::num::ParseFloatError> for LexError {
	fn from(_: ::std::num::ParseFloatError) -> Self {
		return Self::FloatParseError;
	}
}

impl From<String> for LexError {
	fn from(s: String) -> Self {
		return Self::UnescapeError(s);
	}
}

#[derive(logos::Logos, Debug, PartialEq)]
#[logos(error = LexError)]
pub enum Token<'source> {
	#[token(r"fn")]
	Function,
	#[token(r"const")]
	Constant,
	#[token(r"let")]
	LocalVariable,

	#[regex(r"\$")]
	// TODO: Rename? I just came up with this name on the spot.
	System,

	#[regex(r"[\p{XID_Start}_]\p{XID_Continue}*", |lexer| lexer.slice().to_string())]
	Identifier(String),
	// TODO: I don't think CORAL will support diamond syntax. Check back later.
	#[regex(r"[a-zA-Z0-9_]+(::[a-zA-Z0-9_]+)+", |lexer| lexer.slice().to_string())]
	Path(String),

	#[regex(r"0x[0-9a-fA-F]+", |lexer| u128::from_str_radix(&lexer.slice()[2..], 16))]
	#[regex(r"[0-9]+", |lexer| lexer.slice().parse::<u128>())]
	Integer(u128),

	#[regex(r"[0-9]*\.[0-9]+([eE][+-]?[0-9]+)?", |lexer| lexer.slice().parse::<f64>())]
	Float(f64),

	#[regex(r#""([^"\\]|.)*""#, |lexer| unescape_string(&lexer.slice()[1..lexer.slice().len() - 1]))]
	String(String),

	#[token("(", |lexer| extract_nested(lexer, ('(', ')')))]
	CurvedBrackets(Option<&'source str>),
	#[token("[", |lexer| extract_nested(lexer, ('[', ']')))]
	SquareBrackets(Option<&'source str>),

	#[token("{", |lexer| extract_nested(lexer, ('{', '}')))]
	Block(Option<&'source str>),

	#[regex("#[^\n]*", extract_luau_block, allow_greedy = true)]
	LuauBlock(&'source str),

	#[token(">>")]
	Out,
	#[token("<<")]
	In,

	#[token(">1>")]
	BitShiftRight1,
	#[token(">0>")]
	BitShiftRight0,
	#[token("<1<")]
	BitShiftLeft1,
	#[token("<0<")]
	BitShiftLeft0,

	#[token("==")]
	Equal,
	#[token("!=")]
	NotEqual,
	#[token(">=")]
	MoreOrEqual,
	#[token("<=")]
	LessOrEqual,
	#[token(">")]
	More,
	#[token("<")]
	Less,

	#[token("->")]
	Arrow,
	#[token(":")]
	Colon,
	#[token(";")]
	Semicolon,
	#[token("=")]
	Equals,
	#[token(",")]
	Comma,

	#[regex(r"(//[^\n]*)", |lexer| lexer.slice().to_string(), allow_greedy = true)]
	CommentSingle(String),
	#[regex(r"/\*[^*]*\*+(?:[^/*][^*]*\*+)*/", |lex| lex.slice().to_string(), allow_greedy = true)]
	CommentMulti(String),

	#[regex(r"[\s\t\n\f]+", logos::skip)]
	Whitespace,
}

pub(self) fn extract_nested<'source>(lexer: &mut logos::Lexer<'source, Token<'source>>, chars: (char, char)) -> Option<&'source str> {
	let remainder = lexer.remainder();
	let mut depth = 1;
	let mut end_index = 0;

	for (index, char) in remainder.char_indices() {
		if char == chars.0 {
			depth += 1;
		} else if char == chars.1 {
			depth -= 1;
		};

		if depth == 0 {
			end_index = index;
			break;
		};
	}

	if depth == 0 {
		let inner_content = &remainder[.. end_index];
		lexer.bump(end_index + 1); // Advance the lexer past the end.

		return Some(inner_content);
	} else {
		return None; // Unmatched error.
	};
}

fn extract_luau_block<'source>(lexer: &mut logos::Lexer<'source, Token<'source>>) -> &'source str {
	dbg!(lexer.remainder());
	return lexer.slice().strip_prefix('#').unwrap_or("");
}

pub(self) fn unescape_string(input: &str) -> Result<String, String> {
	let mut result = String::with_capacity(input.len());
	let mut chars = input.chars().peekable();

	while let Some(c) = chars.next() {
		if c == '\\' {
			match chars.next() {
				Some('n') => result.push('\n'),
				Some('s') => result.push(' '),
				Some('r') => result.push('\r'),
				Some('t') => result.push('\t'),
				Some('0') => result.push('\0'),
				Some('\\') => result.push('\\'),
				Some('\'') => result.push('\''),
				Some('"') => result.push('"'),

				// Handle Hexadecimal escape codes: \x41 → 'A'.
				Some('x') => {
					let mut hex = String::new();
					for _ in 0 .. 2 {
						if let Some(&h) = chars.peek() {
							if h.is_ascii_hexdigit() {
								hex.push(chars.next().unwrap());
							} else {
								return Err("Invalid hex escape sequence: expected 2 hex digits".to_string());
							};
						};
					}

					if hex.len() != 2 {
						return Err("Invalid hex escape sequence: truncated".to_string());
					};

					let byte = u8::from_str_radix(&hex, 16).map_err(|_| "Failed to parse hex escape")?;

					result.push(byte as char);
				},

				// Handle Unicode escape codes: \u{1F600} → '😀'.
				Some('u') => {
					if chars.next() != Some('{') {
						return Err("Invalid unicode escape sequence: missing leading '{'.".to_string());
					}
					let mut hex = String::new();
					while let Some(&u) = chars.peek() {
						if u == '}' {
							chars.next(); // Consume '}'.
							break;
						} else if u.is_ascii_hexdigit() && hex.len() < 6 {
							hex.push(chars.next().unwrap());
						} else {
							return Err("Invalid character in unicode escape.".to_string());
						};
					}
					let code_point = u32::from_str_radix(&hex, 16).map_err(|_| "Failed to parse unicode hex.".to_string())?;
					let unicode_char = std::char::from_u32(code_point).ok_or_else(|| "Invalid unicode code point.".to_string())?;
					result.push(unicode_char);
				},

				Some(invalid) => return Err(format!("Unknown escape sequence: '\\{}'.", invalid)),
				None => return Err("Trailing backslash at end of string.".to_string()),
			};
		} else {
			result.push(c);
		};
	}

	return Ok(result);
}
