// Traits

pub trait VESCSendable {
	// TODO - convert to binary or binary CAN signal
	/// Converts the object to a binary representation so it can be sent easier.
	fn to_header_binary(&self) -> Vec<u8>;
	fn to_body_binary(&self) -> Vec<u8>;
}

pub trait CANSendable {
	fn to_can_binary(&self) -> Vec<u8>;
}

pub trait ByteConvertable {
	fn as_bytes(self) -> Vec<u8>;
}




// Message Struct

pub struct Message {
	command: CommandType,
	target: u8,
	payload: f32
} impl Message {
	pub fn new(command: CommandType, target: u8, payload: f32) -> Self {
		return Self {
			command,
			target,
			payload
		}
	}
} impl VESCSendable for Message {

	fn to_header_binary(&self) -> Vec<u8> {
		// target is stored in the lower byte, the rest of the space is used for the command
		((self.target as u32 ) | ((self.command as u32) << 8)).as_bytes() 
	}

	fn to_body_binary(&self) -> Vec<u8> {
		self.command.pack_payload_data(self.payload).as_bytes()
	}
}



// Helpers and simple impls

impl<T: VESCSendable> CANSendable for T {
	fn to_can_binary(&self) -> Vec<u8> {
		let mut out: Vec<u8> = vec![];

		out.extend(self.to_header_binary());
		out.extend(self.to_body_binary());

		out
	}
}

impl ByteConvertable for u32 {
	fn as_bytes(self) -> Vec<u8> {
		vec![
			(self >> 0)  as u8,
			(self >> 8)  as u8,
			(self >> 16) as u8,
			(self >> 24) as u8,
		]
	}
}

impl ByteConvertable for u64 {
	fn as_bytes(self) -> Vec<u8> {
		vec![
			(self >> 0)  as u8,
			(self >> 8)  as u8,
			(self >> 16) as u8,
			(self >> 24) as u8,
			(self >> 32)  as u8,
			(self >> 40)  as u8,
			(self >> 48) as u8,
			(self >> 56) as u8,
		]
	}
}


// Command type enum

#[derive(Clone, Copy)]
pub enum CommandType {
	TEST = 0
} impl CommandType {
	/// Converts data to be transmitted into the form expected by VESC.
	fn pack_payload_data(self, payload: f32) -> u64 {
		match self {
			CommandType::TEST => (payload as f32 * 0x100000 as f32) as u64,
		}
	}
}