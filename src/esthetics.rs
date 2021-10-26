/// An `enum` that represents the possible style a text can be displayed in terminal
#[derive(Clone)]
pub enum Style {
	Normal        = 0,
	Bold          = 1,
	Dimmed        = 2,
	Italic        = 3,
	Underline     = 4,
	BlinkSlow     = 5,
	BlinkFast     = 6,
	Reverse       = 7,
	Hidden        = 8,
	Strikethrough = 9,
}

impl Default for Style {
	fn default() -> Self {
		return Style::Normal;
	}
}

/// An `enum` that represents 8 possible colors that can be displayed in terminal
#[derive(Clone)]
pub enum Color {
	Black  = 30,
	Red    = 31,
	Green  = 32,
	Yellow = 33,
	Blue   = 34,
	Purple = 35,
	Cyan   = 36,
	White  = 37,
}

impl Default for Color {
	fn default() -> Self {
		return Color::White;
	}
}


#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn clone_works() {
		let a = Color::Black;
		let b = a.clone();
		matches!(a, _b);
		matches!(b, _a);

		let x = Style::Italic;
		let y = x.clone();
		matches!(x, _y);
		matches!(y, _x);
	}

	#[test]
	fn default_works() {
		let a = Color::White;
		let b = Color::default();
		matches!(a, _b);
		matches!(b, _a);

		let x = Style::Normal;
		let y = Style::default();
		matches!(x, _y);
		matches!(y, _x);
	}
}
