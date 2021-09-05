use colored_text::{self, esthetics, ColoredString};

#[test]
fn test_display() {
	let c1 = ColoredString::new("xxx", esthetics::Style::Bold, esthetics::Color::Red);
	assert_eq!(format!("{}", c1), "\x1b[1;31mxxx\x1b[0m");

	let c2 = ColoredString::new(
		"xxx",
		esthetics::Style::Strikethrough,
		esthetics::Color::Red,
	);
	assert_eq!(format!("{}", c2), "\x1b[9;31mxxx\x1b[0m");
}
