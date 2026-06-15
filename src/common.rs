#[derive(Debug, Copy, Clone)]
pub enum CommState {
    Request = 0,
    DataTransfer = 1,
    Checksum = 3,
    Unknown = 4,
}

pub fn get_state(val: u8) -> CommState {
    match val {
        0 => CommState::Request,
        1 => CommState::DataTransfer,
        2 => CommState::Checksum,
        _ => CommState::Unknown,
    }
}

pub fn get_value(state: CommState) -> u8 {
    match state {
        CommState::Request => 0,
        CommState::DataTransfer => 1,
        CommState::Checksum => 2,
        CommState::Unknown => 3,
    }
}

// pub const DATA_SIZE: usize = 1024 * 1024;
pub const DATA_SIZE: usize = 128;

//
// end of file
//
