use colored_text::esthetics::*;
use colored_text::ColoredText;

#[test]
fn test_display() {
	let c1_1 = ColoredText::text("x")
		.style(Style::Normal)
		.color(Color::Black);
	assert_eq!(format!("{}", c1_1), "\x1b[0;30mx\x1b[0m");

	let c1_2 = ColoredText::text("x")
		.style(Style::Normal)
		.color(Color::Red);
	assert_eq!(format!("{}", c1_2), "\x1b[0;31mx\x1b[0m");

	let c1_3 = ColoredText::text("x")
		.style(Style::Normal)
		.color(Color::Green);
	assert_eq!(format!("{}", c1_3), "\x1b[0;32mx\x1b[0m");

	let c2_6 = ColoredText::text("x")
		.style(Style::Bold)
		.color(Color::Purple);
	assert_eq!(format!("{}", c2_6), "\x1b[1;35mx\x1b[0m");

	let c3_4 = ColoredText::text("x")
		.style(Style::Dimmed)
		.color(Color::Yellow);
	assert_eq!(format!("{}", c3_4), "\x1b[2;33mx\x1b[0m");

	let c4_5 = ColoredText::text("x")
		.style(Style::Italic)
		.color(Color::Blue);
	assert_eq!(format!("{}", c4_5), "\x1b[3;34mx\x1b[0m");

	let c5_6 = ColoredText::text("x")
		.style(Style::Underline)
		.color(Color::Purple);
	assert_eq!(format!("{}", c5_6), "\x1b[4;35mx\x1b[0m");

	let c6_7 = ColoredText::text("x")
		.style(Style::BlinkSlow)
		.color(Color::Cyan);
	assert_eq!(format!("{}", c6_7), "\x1b[5;36mx\x1b[0m");

	let c7_8 = ColoredText::text("x")
		.style(Style::BlinkFast)
		.color(Color::White);
	assert_eq!(format!("{}", c7_8), "\x1b[6;37mx\x1b[0m");

	let c8_1 = ColoredText::text("x")
		.style(Style::Reverse)
		.color(Color::Black);
	assert_eq!(format!("{}", c8_1), "\x1b[7;30mx\x1b[0m");

	let c9_2 = ColoredText::text("x")
		.style(Style::Hidden)
		.color(Color::Red);
	assert_eq!(format!("{}", c9_2), "\x1b[8;31mx\x1b[0m");

	let c10_3 = ColoredText::text("x")
		.style(Style::Strikethrough)
		.color(Color::Green);
	assert_eq!(format!("{}", c10_3), "\x1b[9;32mx\x1b[0m");
}
