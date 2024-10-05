use vesc_translator::*;

macro_rules! uart_binary_test {
    ($message:expr, $result:literal, $name:ident) => {
        #[test]
        fn $name() {
            let msg: Message = $message;
            assert_eq!(msg.to_uart_binary(), $result.to_be_bytes()[6..].to_vec());
        }
    }
}

#[test]
fn body_binary_test() {
    let msg = Message::new(CommandType::SetDutyCycle, 0, 1.0);
    assert_eq!(msg.to_body_binary(), 1_u64.to_be_bytes().to_vec());
}

uart_binary_test!(Message::new(CommandType::SetRpm, 0, 1.0), 0x02050800000001120c03_u128, uart_binary_test_set_rpm_1);
uart_binary_test!(Message::new(CommandType::SetRpm, 1, 1.0), 0x02050800000001120c03_u128, uart_binary_test_set_rpm_2);
uart_binary_test!(Message::new(CommandType::SetRpm, 0, 2.0), 0x02050800000002226f03_u128, uart_binary_test_set_rpm_3);
uart_binary_test!(Message::new(CommandType::SetDutyCycle, 0, 1.0), 0x02050500000001337603_u128, uart_binary_test_set_duty_cycle_1);
uart_binary_test!(Message::new(CommandType::SetDutyCycle, 0, 2.0), 0x02050500000002031503_u128, uart_binary_test_set_duty_cycle_2);
uart_binary_test!(Message::new(CommandType::SetDutyCycle, 0, 4.0), 0x0205050000000463d303_u128, uart_binary_test_set_duty_cycle_3);
