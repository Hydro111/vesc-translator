// Traits
pub trait VescSendable: VescSendableExtending + VescSendableValue {}
impl<T> VescSendable for T where T: VescSendable + VescSendableExtending {}
// Above technique from https://stackoverflow.com/questions/26983355/is-there-a-way-to-combine-multiple-traits-in-order-to-define-a-new-trait

pub trait VescSendableValue {
    // TODO - convert to binary or binary CAN signal
    /// Converts the object to a binary representation so it can be sent easier.
    fn to_header_binary(&self) -> Vec<u8>;
    fn to_body_binary(&self) -> Vec<u8>;
}

pub trait VescSendableExtending {
    fn extend_header_binary(&self, out: &mut Vec<u8>);
    fn extend_body_binary(&self, out: &mut Vec<u8>);
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
impl VescSendableExtending for Message {
    fn extend_header_binary(&self, out: &mut Vec<u8>) {
        // target is stored in the lower byte, the rest of the space is used for the command
        out.extend(((self.target as u32) | ((self.command as u32) << 8)).to_ne_bytes());
    }

    fn extend_body_binary(&self, out: &mut Vec<u8>) {
        out.extend(self.command.pack_payload_data(self.payload).to_ne_bytes());
    }
}

// Helpers and simple impls
impl<T: VescSendable> CanBusSendable for T {
    fn to_can_binary(&self) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];

        out.extend(self.to_header_binary());
        out.extend(self.to_body_binary());
        todo!(); // Needs to include other CAN information in proper format
    }
}

impl<T: VescSendableExtending> VescSendableValue for T {
    fn to_header_binary(&self) -> Vec<u8> {
        let mut out = vec![];
        self.extend_header_binary(&mut out);
        out
    }
    fn to_body_binary(&self) -> Vec<u8> {
        let mut out = vec![];
        self.extend_body_binary(&mut out);
        out
    }
}

// Command type enum
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum CommandType {
    // Numeric value is the command id in VESC.
    // These will be cast to 3 bytes, so these will all be lower than 16,777,216
    TEST = 0,
}
impl CommandType {
    /// Converts data to be transmitted into the form expected by VESC.
    fn pack_payload_data(self, payload: f32) -> u64 {
        match self {
            CommandType::TEST => (payload as f32 * 0x1000 as f32) as u64,
        }
    }
}
