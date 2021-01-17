use crate::serial::*;
use cortex_m::interrupt::free as interrupt_free;

pub static mut WRITER: DbgWriter = DbgWriter { uart: None };

pub struct DbgWriter {
    uart: Option<Tx<crate::pac::UART0>>,
}

impl ::core::fmt::Write for DbgWriter {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        match &mut self.uart {
            Some(uart) => uart.write_str(s),
            None => Ok(()),
        }
    }
}

pub fn wire_uart(uart: Tx<crate::pac::UART0>) {
    interrupt_free(|_| unsafe {
        WRITER = DbgWriter { uart: Some(uart) };
    });
}

#[macro_export]
macro_rules! dbgprint {
    ($($arg:tt)*) => {
        {
            use cortex_m::interrupt::free as interrupt_free;
            interrupt_free(|_| unsafe {
                {
                    use ::core::fmt::Write;
                    use $crate::debug::WRITER;
                    WRITER.write_fmt(format_args!($($arg)*)).unwrap_or(());
                };
            });
        }
    };
}
