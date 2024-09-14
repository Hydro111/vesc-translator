use socketcan::{socket::*, CanFrame, EmbeddedFrame, ExtendedId};

use crate::messages::{CommandType, Message, VescSendable};

/// The Motor trait is the basis for interactions with vesc-translator that use comms.
/// A common, provided implementer for this trait is VescCanMotor.
pub trait Motor {
    fn new(id: u8) -> Self;
    fn send_message(&self, command: CommandType, payload: f32);

    // All motor commands should just use send_message with different parameters
    fn set_rpm(&self, percent: f32) {
        self.send_message(CommandType::SetRpm, percent);
    }
    fn set_duty_cycle(&self, duty_cycle: f32) {
        self.send_message(CommandType::SetDutyCycle, duty_cycle);
    }

    // TODO add more commands and requests
}

pub struct VescCanMotor {
    id: u8,
    soc: CanSocket,
}
impl Motor for VescCanMotor {
    fn new(id: u8) -> Self {
        Self {
            id,
            soc: CanSocket::open_addr(&CanAddr::new(id as u32)).expect("CAN addressing failed."),
        }
    }

    fn send_message(&self, command: CommandType, payload: f32) {
        // Create the message object
        let msg = Message::new(command, self.id, payload);

        match merge_bytes_small(message.to_header_binary()) {
            Ok(id) => {
                let id = ExtendedId::new(id).unwrap();
                // Turn it into socketcan's message object
                let frame: canFrame = canFrame::new(id, msg.to_body_binary().as_slice()).unwrap();
                // Send it
                _ = self.soc.write_frame_insist(&frame);
            }
            Err(e) => {
                panic!("error merging bytes: {}", e);
            }
        }
    }
}

// Helpers
fn merge_bytes_small(bytes: Vec<u8>) -> Result<u32, String> {
    if bytes.len() > 4 {
        return Err("merge_bytes_small can only be called on series of bytes smaller than 4".to_string());
    }

    let mut shift_val = 0;
    let mut out = 0_u32;

    for byte in bytes {
        out |= (byte as u32) << shift_val;
        shift_val += 8;
    }

    Ok(out)
}
