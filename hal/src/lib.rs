#![no_std]

use embedded_hal as hal;
use paste::paste;

mod math;

#[cfg(feature = "debug_on_uart0")]
#[macro_use]
pub mod debug;

#[cfg(not(feature = "debug_on_uart0"))]
#[macro_export]
#[doc(hidden)]
macro_rules! dbgprint {
    ($($arg:tt)*) => {{}};
}

#[cfg(feature = "sam4e8c")]
pub use atsam4e8c as pac;

#[cfg(feature = "sam4e8e")]
pub use atsam4e8e as pac;

#[cfg(feature = "sam4e16c")]
pub use atsam4e16c as pac;

#[cfg(feature = "sam4e16e")]
pub use atsam4e16e as pac;

pub mod delay;
pub mod gpio;
pub mod pmc;
pub mod serial;
pub mod time;
pub mod usb;
