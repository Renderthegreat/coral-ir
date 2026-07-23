use ::mlua;
use ::mlua_magic_macros;

use ::std::{
	io::{
		Read,
		Write,
		stdout,
		stdin,
		stderr,
	},
};
use ::core::{
	fmt,
};
use ::console::{
	Color,
	style,
};

use ::lune_utils::fmt::{
	ValueFormatConfig,
	pretty_format_value,
	pretty_format_multi_value,
};

pub(self) const FORMAT_CONFIG: ValueFormatConfig = ValueFormatConfig::new().with_colors_enabled(true).with_max_depth(4);
pub(self) const SINGLE_FORMAT_CONFIG: ValueFormatConfig = ValueFormatConfig::new().with_colors_enabled(true).with_max_depth(0);

#[derive(Clone, Copy)]
pub(self) enum Label {
	Log,
	Warning,
	Error,
}

impl Label {
	pub(self) fn color(&self) -> Color {
		return match *self {
			Self::Log => Color::Blue,
			Self::Warning => Color::Yellow,
			Self::Error => Color::Red,
		};
	}

	pub(self) fn name(&self) -> &'static str {
		return match *self {
			Self::Log => "LOG",
			Self::Warning => "WARN",
			Self::Error => "ERROR",
		};
	}
}

impl fmt::Display for Label {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		return write!(f, "{}{}{}", style("[").dim(), style(self.name()).fg(self.color()), style("]").dim(),);
	}
}

#[derive(Clone)]
pub struct Console {
	pub output: fn() -> Box<dyn Write>,
	pub input: fn() -> Box<dyn Read>,
	pub error: fn() -> Box<dyn Write>,
}

#[mlua_magic_macros::implementation]
impl Console {
	pub fn log(&self, values: mlua::MultiValue) -> mlua::Result<()> {
		let formatted = format!("{} {}\n", Label::Log, pretty_format_multi_value(&values, &FORMAT_CONFIG));
		(self.output)().write_all(formatted.as_bytes())?;

		return Ok(());
	}

	pub fn warn(&self, values: mlua::MultiValue) -> mlua::Result<()> {
		let formatted = format!("{} {}\n", Label::Warning, pretty_format_multi_value(&values, &FORMAT_CONFIG));
		(self.output)().write_all(formatted.as_bytes())?;

		return Ok(());
	}

	pub fn error(&self, values: mlua::MultiValue) -> mlua::Result<()> {
		let formatted = format!("{} {}\n", Label::Error, pretty_format_multi_value(&values, &FORMAT_CONFIG));
		(self.output)().write_all(formatted.as_bytes())?;

		return Ok(());
	}

	pub fn format(&self, value: mlua::Value, max_depth: Option<usize>, format_strings: Option<bool>) -> mlua::Result<String> {
		let format_strings = format_strings.unwrap_or(false);

		// Check if it's a string first to avoid unnecessary pretty-printing overhead.
		if let (mlua::Value::String(lua_string), true) = (&value, format_strings) {
			let raw_str = lua_string.to_string_lossy();

			let escaped = raw_str.replace('\\', "\\\\").replace('"', "\\\"");

			return Ok(style(format!("\"{}\"", escaped)).fg(Color::Green).to_string());
		};

		// Fallback to default pretty-printing for other types.
		let config = if let Some(depth) = max_depth { FORMAT_CONFIG.with_max_depth(depth) } else { SINGLE_FORMAT_CONFIG };

		let string = pretty_format_value(&value, &config);
		return Ok(string);
	}

	pub fn format_multi(&self, values: mlua::MultiValue) -> mlua::Result<String> {
		let mut string = String::new();

		for value in values {
			// TODO.
			string = format!("{}, ", self.format(value, None, None)?);
		}

		return Ok(string);
	}
}

impl Default for Console {
	fn default() -> Self {
		return Self {
			output: || Box::new(stdout()),
			input: || Box::new(stdin()),
			error: || Box::new(stderr()),
		};
	}
}

mlua_magic_macros::compile!(type_path = Console, fields = false, methods = true, variants = false);
