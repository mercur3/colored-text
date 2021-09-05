/// An `enum` that represents 8 possible colors that can be displayed in terminal
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

/// An `enum` that represents the possible style a text can be displayed in terminal
pub enum Style {
	Normal        = 0,
	Bold          = 1,
	Dark          = 2,
	Italic        = 3,
	Underline     = 4,
	BlinkSlow     = 5,
	BlinkFast     = 6,
	Reverse       = 7,
	Hidden        = 8,
	Strikethrough = 9,
}
