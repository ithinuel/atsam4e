use cortex_m::interrupt;

pub static mut WRITER: Option<DbgWriter> = None;

pub struct DbgWriter {
    buffer: &'static mut [u8],
    len: usize,
}
impl DbgWriter {
    pub fn using_buffer(buffer: &'static mut [u8]) -> Self {
        Self { buffer, len: 0 }
    }
}

impl ::core::fmt::Write for DbgWriter {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        if self.len > self.buffer.len() {
            // invalid state, that sucks
            self.len = 0;
        }
        let len = core::cmp::min(self.buffer.len() - self.len, s.len());
        let from = self.len;
        self.len += len;
        self.buffer[from..self.len].copy_from_slice(&s.as_bytes()[..len]);
        Ok(())
    }
}

pub fn setup_with_buffer(buffer: &'static mut [u8]) {
    interrupt::free(move |_| unsafe {
        WRITER = Some(DbgWriter::using_buffer(buffer));
    });
}
pub fn consume(mut reader: impl FnMut(&[u8]) -> usize) {
    interrupt::free(|_| unsafe {
        WRITER.as_mut().map(|w| {
            let len = w.len;
            w.len -= reader(&w.buffer[..len]);
            w.buffer.copy_within(w.len..len, 0);
        });
    });
}

#[macro_export]
macro_rules! dbgprint {
    ($($arg:tt)*) => {
        {
            use cortex_m::interrupt::free as interrupt_free;
            interrupt_free(|_| unsafe {
                use ::core::fmt::Write;
                use $crate::debug_on_buffer::WRITER;
                WRITER.as_mut().map(|w| w.write_fmt(format_args!($($arg)*)));
            });
        }
    };
}
