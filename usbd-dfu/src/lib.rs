#![no_std]

//pub mod dfumode;
pub mod runtime;

pub const USB_CLASS_DFU: u8 = 0xFE;
pub const USB_SUB_CLASS_DFU: u8 = 0x01;
pub const USB_PROTOCOL_DFU: u8 = 0x01;

pub const DFU_FUNCTIONAL: u8 = 0x21;

pub const DFU_VERSION: u16 = 0x0100; // bcdDFUVersion

#[allow(non_snake_case)]
mod Request {
    pub const DFU_DETACH: u8 = 0;
    pub const DFU_DNLOAD: u8 = 1;
    pub const DFU_UPLOAD: u8 = 2;
    pub const DFU_GETSTATUS: u8 = 3;
    pub const DFU_CLRSTATUS: u8 = 4;
    pub const DFU_GETSTATE: u8 = 5;
    pub const DFU_ABORT: u8 = 6;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    AppIdle,
    /// Timestamp (+/- the poll latency) when the detach request was received.
    AppDetach(u32),
}

impl From<State> for u8 {
    fn from(state: State) -> Self {
        match state {
            State::AppIdle => 0,
            State::AppDetach(_) => 1,
        }
    }
}

#[repr(u8)]
enum Status {
    Ok = 0,
}
