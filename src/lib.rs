use std::fmt::{Display, Formatter, Result};
pub mod esthetics;

pub struct ColoredString<'a> {
	text: &'a str,
	style: String,
	color: String,
}

impl ColoredString<'_> {
	pub fn new(text: &str, style: esthetics::Style, color: esthetics::Color) -> ColoredString {
		return ColoredString {
			text,
			style: (style as i32).to_string(),
			color: (color as i32).to_string(),
		};
	}
}

impl Display for ColoredString<'_> {
	fn fmt(&self, f: &mut Formatter) -> Result {
		const CLEAR: &str = "\x1b[0m";

		let mut display = String::from("\x1b[");
		display.push_str(&self.style);
		display.push(';');
		display.push_str(&self.color);
		display.push('m');

		return write!(
			f,
			"{style}{text}{clear}",
			style = display,
			text = self.text,
			clear = CLEAR
		);
	}
}
