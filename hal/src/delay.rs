//! Delays

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::peripheral::SYST;

use crate::hal::blocking::delay::{DelayMs, DelayUs};
use crate::pmc::Clocks;
use crate::time::*;

/// System timer (SysTick) as a delay provider
pub struct Delay {
    /// Number of sysclock tick in 1 μs.
    tick_per_us: u32,
    syst: SYST,
}

pub struct InvalidClockError;

impl Delay {
    /// Configures the system timer (SysTick) as a delay provider
    pub fn new(mut syst: SYST, clocks: Clocks) -> Result<Self, InvalidClockError> {
        if clocks.master_clock.0 < 8_000_000 {
            return Err(InvalidClockError);
        }

        syst.set_clock_source(SystClkSource::External);
        Ok(Delay {
            syst,
            tick_per_us: clocks.master_clock.0 / 8_000_000,
        })
    }

    /// Releases the system timer (SysTick) resource
    pub fn free(self) -> SYST {
        self.syst
    }
}

impl<T: Into<Miliseconds>> DelayMs<T> for Delay {
    fn delay_ms(&mut self, ms: T) {
        self.delay_us(ms.into());
    }
}

impl<T: Into<Microseconds>> DelayUs<T> for Delay {
    fn delay_us(&mut self, us: T) {
        let us: Microseconds = us.into();
        // The SysTick Reload Value register supports values between 1 and 0x00FFFFFF.
        const MAX_RVR: u32 = 0x00FF_FFFF;

        let mut total_rvr = us.0 * self.tick_per_us;

        while total_rvr != 0 {
            let current_rvr = if total_rvr <= MAX_RVR {
                total_rvr
            } else {
                MAX_RVR
            };

            self.syst.set_reload(current_rvr);
            self.syst.clear_current();
            self.syst.enable_counter();

            // Update the tracking variable while we are waiting...
            total_rvr -= current_rvr;

            while !self.syst.has_wrapped() {}

            self.syst.disable_counter();
        }
    }
}
