
// Traits
pub trait VescSendable {
    // TODO - convert to binary or binary CAN signal
    /// Converts the object to a binary representation so it can be sent easier.
    fn to_header_binary(&self) -> Vec<u8>;
    fn to_body_binary(&self) -> Vec<u8>;
}

pub trait CanBusSendable {
    fn to_can_binary(&self) -> Vec<u8>;
}

// Message Struct
#[derive(Clone, Copy)]
pub struct Message {
    command: CommandType,
    target: u8,
    payload: f32,
}
impl Message {
    pub fn new(command: CommandType, target: u8, payload: f32) -> Self {
        return Self {
            command,
            target,
            payload,
        };
    }
}
impl VescSendable for Message {
    fn to_header_binary(&self) -> Vec<u8> {
        // target is stored in the lower byte, the rest of the space is used for the command
        ((self.target as u32) | ((self.command as u32) << 8))
            .to_ne_bytes()
            .to_vec()
    }

    fn to_body_binary(&self) -> Vec<u8> {
        self.command
            .pack_payload_data(self.payload)
            .to_ne_bytes()
            .to_vec()
    }
}


// Helpers and simple impls
impl<T: VescSendable> CanBusSendable for T {
    fn to_can_binary(&self) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];

        out.extend(self.to_header_binary());
        out.extend(self.to_body_binary());
        todo!(); // Needs to include other CAN information in proper format
        out
    }
}


// Command type enum
#[derive(Clone, Copy)]
pub enum CommandType {
    // Numeric value is the command id in VESC.
	// These will be cast to 3 bytes, so these will all be lower than 16,777,216
    TEST = 0,
}
impl CommandType {
    /// Converts data to be transmitted into the form expected by VESC.
    fn pack_payload_data(self, payload: f32) -> u64 {
        match self {
            CommandType::TEST => (payload as f32 * 0x100000 as f32) as u64,
        }
    }
}
