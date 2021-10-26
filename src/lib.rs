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

#[derive(Default, PartialEq, Eq, Debug)]
pub struct ColoredString<'a> {
	text: &'a str,
	style: esthetics::Style,
	color: esthetics::Color,
}

impl ColoredString<'_> {
	/// Creates a new [ColoredString] object with the following text, style and color
	pub fn new(text: &str, style: esthetics::Style, color: esthetics::Color) -> ColoredString {
		return ColoredString { text, style, color };
	}

	/// Creates a new [ColoredString] with a specific color, but style set to
	/// `esthetics::Style::Normal`
	///
	/// ```rust
	/// use colored_text::{esthetics, ColoredString};
	///
	/// let text = "abc";
	/// let blue1 = ColoredString::with_color(text, esthetics::Color::Blue);
	/// let blue2 = ColoredString::new(text, esthetics::Style::Normal, esthetics::Color::Blue);
	///
	/// assert_eq!(format!("{}", blue1), format!("{}", blue2));
	/// ```
	pub fn with_color(text: &str, color: esthetics::Color) -> ColoredString {
		return ColoredString::new(text, esthetics::Style::Normal, color);
	}

	/// Creates a new [ColoredString] with a specific style, but color set to
	/// `esthetics::Color::White`
	///
	/// ```rust
	/// use colored_text::{esthetics, ColoredString};
	///
	/// let text = "abc";
	/// let bold1 = ColoredString::with_style(text, esthetics::Style::Bold);
	/// let bold2 = ColoredString::new(text, esthetics::Style::Bold, esthetics::Color::White);
	///
	/// assert_eq!(format!("{}", bold1), format!("{}", bold2));
	/// ```
	pub fn with_style(text: &str, style: esthetics::Style) -> ColoredString {
		return ColoredString::new(text, style, esthetics::Color::White);
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
	/// The code above will not display the string abc with a normal style, but because of the
	/// injection it will be displayed with red font and strikethrough style
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
		let x = ColoredString::default();
		let string_default = "";
		let y = ColoredString::new(
			string_default,
			esthetics::Style::Normal,
			esthetics::Color::White,
		);

		assert!(x.eq(&y));
		assert!(y.eq(&x));
	}
}
