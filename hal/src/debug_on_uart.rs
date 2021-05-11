use cortex_m::interrupt::free as interrupt_free;

pub static mut WRITER: DbgWriter = DbgWriter { uart: None };

pub struct DbgWriter {
    uart: Option<*mut dyn core::fmt::Write>,
}

impl ::core::fmt::Write for DbgWriter {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        self.uart
            .map(move |w| {
                let w = unsafe { &mut *w };
                w.write_str(s)
            })
            .unwrap_or(Ok(()))
    }
}

pub fn wire_uart<'a, T: core::fmt::Write + 'a>(mut uart: T) {
    interrupt_free(|_| unsafe {
        let writer: &mut dyn core::fmt::Write = &mut uart;
        WRITER = DbgWriter {
            uart: Some(core::mem::transmute(writer)),
        };
        core::mem::forget(uart);
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
                    use $crate::debug_on_uart::WRITER;
                    WRITER.write_fmt(format_args!($($arg)*)).unwrap_or(());
                };
            });
        }
    };
}
