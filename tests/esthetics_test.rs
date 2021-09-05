use colored_text::esthetics;

#[test]
fn import_works() {
	assert_eq!(esthetics::Color::Red as i32, 31);
	assert_eq!(esthetics::Style::Hiden as i32, 8);
}
