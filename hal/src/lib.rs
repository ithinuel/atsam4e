#![no_std]

use embedded_hal as hal;
use paste::paste;

#[cfg(feature = "debug_on_buffer")]
extern crate alloc;

mod math;

#[cfg(all(feature = "debug_on_uart0", feature = "debug_on_buffer"))]
compile_error!("Only 1 debug feature may be enabled at a time");

#[cfg(feature = "debug_on_uart0")]
#[macro_use]
pub mod debug_on_uart;

#[cfg(feature = "debug_on_buffer")]
#[macro_use]
pub mod debug_on_buffer;

#[cfg(not(any(feature = "debug_on_uart0", feature = "debug_on_buffer")))]
#[macro_export]
#[doc(hidden)]
macro_rules! dbgprint {
    ($($arg:tt)*) => {{}};
}

#[cfg(not(any(feature = "sam4e8c", feature = "sam4e8e", feature = "sam4e16c", feature = "sam4e16e")))]
compile_error!("You need to enable one of the chip feature");

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
