use vesc_translator::*;

#[test]
fn example_test() {
	Message::new(CommandType::TEST, 0, 0.0);
}