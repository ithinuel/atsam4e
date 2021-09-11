use core::convert::TryInto;
use core::fmt;
use core::marker::PhantomData;

use embedded_hal::serial::{self, Read, Write};
use nb::block;

pub mod config;

use crate::pac::{UART0, UART1, USART0, USART1};
use crate::pmc::Clocks;

//
use crate::gpio::{
    pioa::{Pa10, Pa21, Pa22, Pa5, Pa6, Pa9},
    piob::{Pb0, Pb1},
    Alternate, PfA, PfC,
};

pub trait Pins<USART> {}
pub trait PinTx<USART> {}
pub trait PinRx<USART> {}

impl<USART, TX, RX> Pins<USART> for (TX, RX)
where
    TX: PinTx<USART>,
    RX: PinRx<USART>,
{
}

/// A filler type for when the Tx pin is unnecessary
pub struct NoTx;
/// A filler type for when the Rx pin is unnecessary
pub struct NoRx;

impl PinRx<UART0> for NoRx {}
impl PinTx<UART0> for NoTx {}
impl<Filter, Pull> PinRx<UART0> for Pa9<Alternate<PfA>, Filter, Pull> {}
impl<Filter, Pull> PinTx<UART0> for Pa10<Alternate<PfA>, Filter, Pull> {}

impl PinRx<UART1> for NoRx {}
impl PinTx<UART1> for NoTx {}
impl<Filter, Pull> PinRx<UART1> for Pa5<Alternate<PfC>, Filter, Pull> {}
impl<Filter, Pull> PinTx<UART1> for Pa6<Alternate<PfC>, Filter, Pull> {}

impl PinRx<USART0> for NoRx {}
impl PinTx<USART0> for NoTx {}
impl<Filter, Pull> PinRx<USART0> for Pb0<Alternate<PfC>, Filter, Pull> {}
impl<Filter, Pull> PinTx<USART0> for Pb1<Alternate<PfC>, Filter, Pull> {}

impl PinRx<USART1> for NoRx {}
impl PinTx<USART1> for NoTx {}
impl<Filter, Pull> PinRx<USART1> for Pa21<Alternate<PfA>, Filter, Pull> {}
impl<Filter, Pull> PinTx<USART1> for Pa22<Alternate<PfA>, Filter, Pull> {}

pub struct Serial<SERIAL, PINS> {
    _serial: SERIAL,
    _pins: PINS,
}

pub struct Rx<SERIAL> {
    _serial: PhantomData<SERIAL>,
}

pub struct Tx<SERIAL> {
    _serial: PhantomData<SERIAL>,
}

/// Serial error
#[derive(Debug)]
pub enum Error {
    /// Framing error
    Framing,
    /// RX buffer overrun
    Overrun,
    /// Parity check error
    Parity,
}

macro_rules! impl_uart {
    ($({$name:ident, $id:expr})+) => {
        $crate::paste!{ $(
            use $crate::pac::[<$name:lower>]::mr::PAR_A as [<$name _PAR_A>];
            impl Into<[<$name _PAR_A>]> for config::Parity {
                fn into(self) -> [<$name _PAR_A>] {
                    match self {
                        Self::Even => [<$name _PAR_A>]::EVEN,
                        Self::Odd => [<$name _PAR_A>]::ODD,
                        Self::Space => [<$name _PAR_A>]::SPACE,
                        Self::Mark => [<$name _PAR_A>]::MARK,
                        Self::None => [<$name _PAR_A>]::NO,
                    }
                }
            }

            impl<PINS> Serial<$name, PINS> {
                pub fn [<$name:lower>] (
                    uart: $name,
                    pins: PINS,
                    config: config::Config,
                    clocks: Clocks
                ) -> Result<Self, ($name, PINS, config::Error)>
                    where PINS: Pins<$name>
                {
                    //use self::config::*;
                    let pmc = unsafe { &*$crate::pac::PMC::ptr() };
                    let serial = unsafe { &*$name::ptr() };

                    if $id <= 31 {
                        pmc.pmc_pcer0.write_with_zero(|w| unsafe {
                            w.bits( 1 << ($id % 32) )
                        });
                    } else {
                        pmc.pmc_pcer1.write_with_zero(|w| unsafe {
                            w.bits( 1 << ($id % 32) )
                        });
                    }

                    // Disable PDC channel ?
                    // Reset and disable receiver and transmitter
                    // Configure mode
                    serial.mr.write(|w| {
                        w.par().variant(config.parity.into())
                         .chmode().variant($crate::pac::[<$name:lower>]::mr::CHMODE_A::NORMAL)
                    });

                    // Configure baudrate (asynchronous, no oversampling)
                    let cd = match config.baudrate.0
                        .checked_mul(16)
                        .and_then(|br16| ((clocks.master_clock.0 + (br16/2) - 1) / br16).try_into().ok()) {
                        Some(cd) => cd,
                        None => return Err((uart, pins, config::Error::UnreachableBaudrate))
                    };

                    serial.brgr.write(|w| unsafe {
                        w.cd().bits(cd)
                    });
                    // configure interrupt ?
                    // Enable uart
                    serial.cr.write_with_zero(|w| {
                        w.rxen().set_bit();
                        w.txen().set_bit();
                        w
                    });

                    Ok(Serial {
                        _serial: uart,
                        _pins: pins
                    })
                }

                pub fn split(self) -> (Tx<$name>, Rx<$name>) {
                    (
                        Tx {
                            _serial: PhantomData
                        },
                        Rx {
                            _serial: PhantomData
                        }
                    )
                }
                // TODO: this needs to reset the peripheral
                // also, Tx<_> and Rx<_> may need to carry their respective pin so that a splited
                // serial can be reconstituted
                /*pub fn release(self) -> ($name, PINS) {
                    (self.serial, self.pins)
                }*/
            }

            impl<PINS> Read<u8> for Serial<$name, PINS> {
                type Error = Error;

                fn read(&mut self) -> nb::Result<u8, Self::Error> {
                    let mut rx: Rx<$name> = Rx {
                        _serial: PhantomData
                    };
                    rx.read()
                }
            }
            impl<PINS> Write<u8> for Serial<$name, PINS> {
                type Error = Error;

                fn flush(&mut self) -> nb::Result<(), Self::Error> {
                    let mut tx: Tx<$name> = Tx {
                        _serial: PhantomData
                    };
                    tx.flush()
                }

                fn write(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
                    let mut tx: Tx<$name> = Tx {
                        _serial: PhantomData
                    };
                    tx.write(byte)
                }
            }

            impl Read<u8> for Rx<$name> {
                type Error = Error;

                fn read(&mut self) -> nb::Result<u8, Self::Error> {
                    let serial = unsafe { &*$name::ptr() };

                    if serial.sr.read().rxrdy().bit_is_clear() {
                        Err(nb::Error::WouldBlock)
                    } else {
                        Ok(serial.rhr.read().rxchr().bits())
                    }
                }
            }

            impl Write<u8> for Tx<$name> {
                type Error = Error;

                fn flush(&mut self) -> nb::Result<(), Self::Error> {
                    let serial = unsafe { &*$name::ptr() };

                    if serial.sr.read().txrdy().bit_is_clear() {
                        Err(nb::Error::WouldBlock)
                    } else {
                        Ok(())
                    }

                }

                fn write(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
                    let serial = unsafe { &*$name::ptr() };

                    if serial.sr.read().txrdy().bit_is_clear() {
                        Err(nb::Error::WouldBlock)
                    } else {
                        serial.thr.write_with_zero(|w| unsafe { w.txchr().bits(byte) });
                        Ok(())
                    }
                }
            }
         )+}
    };
}

macro_rules! impl_usart {
    ($({$name:ident, $id:expr})+) => {};
}

impl_uart! {
    { UART0, 7 }
    { UART1, 45 }
}
impl_usart! {
    { USART0, 14 }
    { USART1, 15 }
}

impl<UART> fmt::Write for Tx<UART>
where
    Tx<UART>: serial::Write<u8>,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let _ = s.as_bytes().iter().map(|c| block!(self.write(*c))).last();
        Ok(())
    }
}
