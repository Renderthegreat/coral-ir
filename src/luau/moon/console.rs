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
#[mlua_magic_macros::structure]
pub struct FormatConfig {
	pub max_depth: Option<usize>,
	pub format_strings: Option<bool>,
	pub colors_enabled: Option<bool>,
}

impl Default for FormatConfig {
	fn default() -> Self {
		return Self {
			max_depth: None,
			format_strings: None,
			colors_enabled: None,
		};
	}
}

mlua_magic_macros::compile!(type_path = FormatConfig, fields = true, methods = false, variants = false);

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

	pub fn format(&self, value: mlua::Value, config: Option<FormatConfig>) -> mlua::Result<String> {
		let config = config.unwrap_or_default();
		let format_strings = config.format_strings.unwrap_or(false);

		// Check if it's a string first to avoid unnecessary pretty-printing overhead.
		if let (mlua::Value::String(lua_string), true) = (&value, format_strings) {
			let raw_str = lua_string.to_string_lossy();

			let escaped = raw_str.replace('\\', "\\\\").replace('"', "\\\"");

			return Ok(style(format!("\"{}\"", escaped)).fg(Color::Green).to_string());
		};

		// Fallback to default pretty-printing for other types.
		let mut format_config = if let Some(depth) = config.max_depth { FORMAT_CONFIG.with_max_depth(depth) } else { SINGLE_FORMAT_CONFIG };

		if let Some(colors_enabled) = config.colors_enabled {
			format_config = format_config.with_colors_enabled(colors_enabled);
		};

		let string = pretty_format_value(&value, &format_config);
		return Ok(string);
	}

	pub fn format_multi(&self, values: mlua::MultiValue) -> mlua::Result<String> {
		let mut string = String::new();

		for value in values {
			// TODO.
			string = format!("{}, ", self.format(value, None)?);
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
