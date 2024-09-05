use core::slice::SlicePattern;

use socketcan::{socket::*, CanFrame, EmbeddedFrame};

use crate::messages::{CommandType, Message, VescSendable};

pub trait Motor {
    fn new(id: u8) -> Self;
	fn send_message(&self, command: CommandType, payload: f32) -> Result<()>;

	// All motor commands should just use send_message with different parameters
    fn set_rpm(&self, percent: f32) -> Result<()> {self.send_message(CommandType::SetRpm, percent)}
	
    // TODO add more commands and requests
}

pub struct VescCanMotor {
	id: u8,
    soc: Socket,
}
impl Motor for VescCanMotor {
    fn new(id: u8) -> Self {
        Self {
			id,
            soc: CanSocket::open_addr(id),
        }
    }

    fn set_rpm(&self, percent: f32) {
		
    }

	fn send_message(&self, command: CommandType, payload: f32) -> Result<()> {
		// Create the message object
		let msg = Message::new(command, self.id, payload);
		// Turn it into socketcan's message object
		let frame: CanFrame = CanFrame::new(self.id, msg.to_body_binary().as_slice());
		// Send it
        self.soc.write_frame_insist(&frame)
	}
}
