extern crate heapless;

use core::convert::TryInto;

#[derive(Debug, Clone)]
pub enum Command {
    Pos(i32),
    SetPID(i32, i32, i32),
    SendPID(f32, f32, f32),
    Data([i32; 1000]),
    Start,
    Stop,
    Speed(i32),
    Reset(i32),
}

impl Eq for Command {}

impl PartialEq for Command {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Command::Pos(a), Command::Pos(b)) => a == b,
            (Command::SetPID(a, b, c), Command::SetPID(d, e, f)) => a == d && b == e && c == f,
            (Command::SendPID(a, b, c), Command::SendPID(d, e, f)) => a == d && b == e && c == f,
            (Command::Data(a), Command::Data(b)) => a == b,
            (Command::Start, Command::Start) => true,
            (Command::Stop, Command::Stop) => true,
            (Command::Speed(a), Command::Speed(b)) => a == b,
            (Command::Reset(a), Command::Reset(b)) => a == b,
            _ => false,
        }
    }
}

impl Command {
    /// When reading the buffer its important to know how many bytes to read
    /// This function returns the length of the buffer for each command
    /// So we read the first byte to know which command it is and then this function tells us how many bytes to read
    pub fn length(ty: u8) -> usize {
        match ty {
            0 => 5,    // Pos
            1 => 13,   // SetPID
            2 => 13,   // SendPID
            3 => 4005, // Data
            4 => 1,    // Start
            5 => 1,    // Stop
            6 => 5,    // Speed
            7 => 5,    // Reset
            _ => {
                panic!("Unknown command: {}", ty);
            }
        }
    }

    pub fn length_c(&self) -> usize {
        match self {
            Command::Pos(_) => 5,            // Pos
            Command::SetPID(_, _, _) => 13,  // SetPID
            Command::SendPID(_, _, _) => 13, // SendPID
            Command::Data(_) => 4005,        // Data
            Command::Start => 1,             // Start
            Command::Stop => 1,              // Stop
            Command::Speed(_) => 5,          // Speed
            Command::Reset(_) => 5,          // Reset
        }
    }

    pub fn encode(&self) -> [u8; 20] {
        let mut buffer = [0u8; 20];
        let mut index = 0;

        match self {
            Command::Pos(val) => {
                buffer[index] = 0; // Identifier for Pos
                index += 1;
                buffer[index..index + 4].copy_from_slice(&val.to_le_bytes());
                index += 4;
            }
            Command::SetPID(p, i, d) => {
                buffer[index] = 1; // Identifier for SetPID
                index += 1;
                buffer[index..index + 4].copy_from_slice(&p.to_le_bytes());
                index += 4;
                buffer[index..index + 4].copy_from_slice(&i.to_le_bytes());
                index += 4;
                buffer[index..index + 4].copy_from_slice(&d.to_le_bytes());
                index += 4;
            }
            Command::SendPID(p, i, d) => {
                buffer[index] = 2; // Identifier for SendPID
                index += 1;
                buffer[index..index + 4].copy_from_slice(&p.to_le_bytes());
                index += 4;
                buffer[index..index + 4].copy_from_slice(&i.to_le_bytes());
                index += 4;
                buffer[index..index + 4].copy_from_slice(&d.to_le_bytes());
                index += 4;
            }
            Command::Data(arr) => {
                // buffer[index] = 3; // Identifier for Data
                // index += 1;
                // let data_len = arr.len() * 4; // 4 bytes per i32
                // buffer[index..index + 4].copy_from_slice(&(data_len as u32).to_le_bytes());
                // index += 4;
                // for val in arr.iter() {
                //     buffer[index..index + 4].copy_from_slice(&val.to_le_bytes());
                //     index += 4;
                // }
                todo!()
            }
            Command::Start => {
                buffer[index] = 4; // Identifier for Start
                index += 1;
            }
            Command::Stop => {
                buffer[index] = 5; // Identifier for Stop
                index += 1;
            }
            Command::Speed(val) => {
                buffer[index] = 6; // Identifier for Speed
                index += 1;
                buffer[index..index + 4].copy_from_slice(&val.to_le_bytes());
                index += 4;
            }
            Command::Reset(val) => {
                buffer[index] = 7; // Identifier for Reset
                index += 1;
                buffer[index..index + 4].copy_from_slice(&val.to_le_bytes());
            }
        }

        buffer
    }
}
impl Command {
    pub fn decode(bytes: &[u8]) -> Option<Self> {
        let mut cursor = 0;
        let id = bytes.get(cursor)?;
        cursor += 1;
        match id {
            0 => {
                let val = i32::from_le_bytes(bytes[cursor..cursor + 4].try_into().ok()?);
                Some(Command::Pos(val))
            }
            1 => {
                let p = i32::from_le_bytes(bytes[cursor..cursor + 4].try_into().ok()?);
                cursor += 4;
                let i = i32::from_le_bytes(bytes[cursor..cursor + 4].try_into().ok()?);
                cursor += 4;
                let d = i32::from_le_bytes(bytes[cursor..cursor + 4].try_into().ok()?);
                Some(Command::SetPID(p, i, d))
            }
            2 => {
                let p = f32::from_le_bytes(bytes[cursor..cursor + 4].try_into().ok()?);
                cursor += 4;
                let i = f32::from_le_bytes(bytes[cursor..cursor + 4].try_into().ok()?);
                cursor += 4;
                let d = f32::from_le_bytes(bytes[cursor..cursor + 4].try_into().ok()?);
                Some(Command::SendPID(p, i, d))
            }
            3 => {
                let data_len = u32::from_le_bytes(bytes[cursor..cursor + 4].try_into().ok()?);
                cursor += 4;
                if data_len == 0 {
                    return Some(Command::Data([0i32; 1000]));
                }
                let mut data = [0i32; 1000];
                for val in data.iter_mut() {
                    *val = i32::from_le_bytes(bytes[cursor..cursor + 4].try_into().ok()?);
                    cursor += 4;
                }
                Some(Command::Data(data))
            }
            4 => Some(Command::Start),
            5 => Some(Command::Stop),
            6 => {
                let val = i32::from_le_bytes(bytes[cursor..cursor + 4].try_into().ok()?);
                Some(Command::Speed(val))
            }
            7 => {
                let val = i32::from_le_bytes(bytes[cursor..cursor + 4].try_into().ok()?);
                Some(Command::Reset(val))
            }
            _ => None,
        }
    }
}
