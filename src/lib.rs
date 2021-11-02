//! A lightweight library for displaying colored text on a UNIX environment terminal
//!
//! # Examples
//! ```rust
//! use colored_text::{esthetics, ColoredText};
//!
//! let x = ColoredText::text("Hello, World!")
//! 	.style(esthetics::Style::Underline)
//! 	.color(esthetics::Color::Cyan);
//!
//! println!("{}", x);
//! ```
//! Will print in the terminal "Hello World" (without "") with cyan color and underlined

use std::fmt::{Display, Formatter, Result};

pub mod esthetics;

#[derive(Default, PartialEq, Eq, Debug)]
pub struct ColoredText<'a> {
	text: &'a str,
	style: esthetics::Style,
	color: esthetics::Color,
}

impl<'a> ColoredText<'a> {
	/// Creates a new [ColoredText] object with the base text
	pub fn text(text: &'a str) -> ColoredText {
		return ColoredText {
			text,
			..Default::default()
		};
	}

	/// Creates a new [ColoredText] from an existing one, with a specific color
	///
	/// ```rust
	/// use colored_text::{esthetics, ColoredText};
	///
	/// let text = "abc";
	/// let blue = ColoredText::text(text).color(esthetics::Color::Blue);
	///
	/// assert_eq!(format!("{}", blue), format!("\x1b[0;34m{}\x1b[0m", text));
	/// ```
	pub fn color(mut self, color: esthetics::Color) -> ColoredText<'a> {
		self.color = color;
		return self;
	}

	/// Creates a new [ColoredText] from an existing one, with a specific style
	///
	/// ```rust
	/// use colored_text::{esthetics, ColoredText};
	///
	/// let text = "abc";
	/// let bold = ColoredText::text(text).style(esthetics::Style::Bold);
	///
	/// assert_eq!(format!("{}", bold), format!("\x1b[1;37m{}\x1b[0m", text));
	/// ```
	pub fn style(mut self, style: esthetics::Style) -> ColoredText<'a> {
		self.style = style;
		return self;
	}
}

impl Display for ColoredText<'_> {
	/// Formats this instace after the following pattern:
	/// `{style}{text}{CLEAR}`
	/// making sure `stdin` will not be corrupted by printing this object
	///
	/// # Note
	/// The API intolerant to string injection.
	fn fmt(&self, f: &mut Formatter) -> Result {
		const CLEAR: &str = "\x1b[0m";

		let mut display = String::from("\x1b[");
		display.push_str(&(self.style.clone() as i32).to_string());
		display.push(';');
		display.push_str(&(self.color.clone() as i32).to_string());
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

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn default_works() {
		let x = ColoredText::default();
		let string_default = "";
		let y = ColoredText {
			text: string_default,
			style: esthetics::Style::Normal,
			color: esthetics::Color::White,
		};

		assert!(x.eq(&y));
		assert!(y.eq(&x));
	}
}
