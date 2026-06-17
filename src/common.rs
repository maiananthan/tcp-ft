#[derive(Debug, Copy, Clone)]
pub enum TransferState {
    TransferReq = 0,
    TransferReqAck = 1,
    TransferInit = 2,
    TransferComp = 3,
    Unknown = 4,
}

pub fn get_state(val: u8) -> TransferState {
    match val {
        0 => TransferState::TransferReq,
        1 => TransferState::TransferReqAck,
        2 => TransferState::TransferInit,
        3 => TransferState::TransferComp,
        _ => TransferState::Unknown,
    }
}

pub fn get_value(state: TransferState) -> u8 {
    match state {
        TransferState::TransferReq => 0,
        TransferState::TransferReqAck => 1,
        TransferState::TransferInit => 2,
        TransferState::TransferComp => 3,
        TransferState::Unknown => 4,
    }
}

pub const DATA_SIZE: usize = 1024 * 1024;
// pub const DATA_SIZE: usize = 1024 * 100;

//
// end of file
//
