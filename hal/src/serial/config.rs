use crate::time::{Bps, U32Ext};

#[derive(Debug, Copy, Clone)]
pub enum Parity {
    /// Even parity
    Even,
    /// Odd parity
    Odd,
    /// Forced to 0
    Space,
    /// Forced to 1
    Mark,
    /// No Parity
    None,
}

#[derive(Debug, Copy, Clone)]
pub struct Config {
    pub parity: Parity,
    pub baudrate: Bps,
}

impl Config {
    pub fn parity(mut self, parity: Parity) -> Self {
        self.parity = parity;
        self
    }

    pub fn baudrate(mut self, baudrate: Bps) -> Self {
        self.baudrate = baudrate;
        self
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            parity: Parity::None,
            baudrate: 115_200.bps(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Error {
    UnreachableBaudrate,
}
