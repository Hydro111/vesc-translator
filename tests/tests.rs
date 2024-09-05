use vesc_translator::*;

#[test]
fn body_binary_test() {
    let msg = Message::new(CommandType::SetDutyCycle, 0, 1.0);
    assert_eq!(msg.to_body_binary(), 100000_u64.to_ne_bytes().to_vec());
}
