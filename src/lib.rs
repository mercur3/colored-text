//! A lightweight library for displaying colored text on a UNIX environment terminal
//!
//! # Examples
//! ```rust
//! use colored_text::{esthetics, ColoredString};
//!
//! let x = ColoredString::new(
//! 	"Hello, World",
//! 	esthetics::Style::Underline,
//! 	esthetics::Color::Cyan,
//! );
//! println!("{}", x);
//! ```
//! Will print in the terminal "Hello World" (without "") with cyan color and underlined

use std::fmt::{Display, Formatter, Result};
pub mod esthetics;

pub struct ColoredString<'a> {
	text: &'a str,
	style: String,
	color: String,
}

impl ColoredString<'_> {
	/// Creates a new [ColoredString] object with the following text, style and color
	pub fn new(text: &str, style: esthetics::Style, color: esthetics::Color) -> ColoredString {
		return ColoredString {
			text,
			style: (style as i32).to_string(),
			color: (color as i32).to_string(),
		};
	}
}

impl Display for ColoredString<'_> {
	/// Formats this instace after the following pattern:
	/// `{style}{text}{CLEAR}`
	/// making sure `stdin` will not be corrupted by printing this object
	///
	/// # Note
	/// The API is not tolerant to string injection.
	/// ```rust
	/// use colored_text::{esthetics, ColoredString};
	///
	/// let c = ColoredString::new(
	/// 	"\x1b[9;31mabc",
	/// 	esthetics::Style::Normal,
	/// 	esthetics::Color::White,
	/// );
	/// ```
	/// The code above will not display the string abc with a normal style, but because of the injection it will be displayed with red font and strikethrough style
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
