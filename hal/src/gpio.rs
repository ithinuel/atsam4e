use core::marker::PhantomData;

/// Extension trait to split a GPIO peripheral in independent pins and registers
pub trait GpioExt {
    /// The parts to split the GPIO into
    type Parts;

    /// Splits the GPIO block into independent pins and registers
    fn split(self) -> Self::Parts;
}

// The following collection of types is used to encode the
// state of the pin at compile time and helps to avoid misuse.

/// Floating Input (type state)
pub struct Floating;
/// Pulled down Input (type state)
pub struct PullDown;
/// Pulled up Input (type state)
pub struct PullUp;

/// Totem Pole aka Push-Pull (type state)
pub struct PushPull;
/// Open drain output (type state)
pub struct OpenDrain;

/// Peripheral Function A
pub struct PfA;
/// Peripheral Function B
pub struct PfB;
/// Peripheral Function C
pub struct PfC;

/// Filter Enabled
pub trait FilterSlowClockDivider {
    const DIV: u16;
}

/// Debounce filter is disabled
pub struct Unfiltered;
/// Debounce filter enabled using the peripheral clock
pub struct FilteredOnPClk;
/// Debounce filter enabled using the divided slow clock
pub struct FilteredOnSlowClock<T: FilterSlowClockDivider> {
    _divider: PhantomData<T>,
}

/// Some alternate mode (type state)
pub struct Alternate<MODE> {
    _mode: PhantomData<MODE>,
}

/// Input mode (type state)
pub struct Input {}

/// Output mode (type state)
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

macro_rules! pin {
    ($port:ident, $port_id:expr, $pin_id:expr) => {
        $crate::paste! {
            pub struct [<P $port:lower $pin_id>]<Mode, Filter, PullState> {
                _mode: PhantomData<Mode>,
                _pull: PhantomData<PullState>,
                _filter: PhantomData<Filter>,
            }
            //impl<T,U,V> [<P $port:lower $pin_id>]<T,U,V> {
                //const PORT_ID: u8 = $port_id;
                //const PIN_ID: u8 = $pin_id;
            //}

            impl<Mode, Filter, PullState> OutputPin for
                [<P $port:lower $pin_id>]<Output<Mode>, Filter, PullState> {
                type Error = Infallible;

                fn set_high(&mut self) -> Result<(), Self::Error> {
                    let port = unsafe { &*[<PIO $port:upper>]::ptr() };
                    port.sodr.write_with_zero(|w| w.[<p $pin_id>]().set_bit());
                    Ok(())
                }
                fn set_low(&mut self) -> Result<(), Self::Error> {
                    let port = unsafe { &*[<PIO $port:upper>]::ptr() };
                    port.codr.write_with_zero(|w| w.[<p $pin_id>]().set_bit());
                    Ok(())
                }
            }

            impl<Mode, Filter, PullState> StatefulOutputPin for
                [<P $port:lower $pin_id>]<Output<Mode>, Filter, PullState> {
                fn is_set_high(&self) -> Result<bool, Self::Error> {
                    let port = unsafe { &*[<PIO $port:upper>]::ptr() };
                    Ok(port.odsr.read().[<p $pin_id>]().bit_is_set())
                }
                fn is_set_low(&self) -> Result<bool, Self::Error> {
                    let port = unsafe { &*[<PIO $port:upper>]::ptr() };
                    Ok(port.odsr.read().[<p $pin_id>]().bit_is_clear())
                }
            }

            impl<Mode, Filter, PullState> toggleable::Default for
                [<P $port:lower $pin_id>]<Output<Mode>, Filter, PullState> {}

            impl<Mode, Filter, PullState> InputPin for
                [<P $port:lower $pin_id>]<Mode, Filter, PullState> {
                type Error = Infallible;

                fn is_high(&self) -> Result<bool, Self::Error> {
                    let port = unsafe { &*[<PIO $port:upper>]::ptr() };
                    Ok(port.pdsr.read().[<p $pin_id>]().bit_is_set())
                }

                fn is_low(&self) -> Result<bool, Self::Error> {
                    let port = unsafe { &*[<PIO $port:upper>]::ptr() };
                    Ok(port.pdsr.read().[<p $pin_id>]().bit_is_clear())
                }
            }

            impl<Mode, Filter, PullState> [<P $port:lower $pin_id>]<Mode, Filter, PullState> {
                pub fn into_push_pull_output(self, is_default_high: bool)->
                    [<P $port:lower $pin_id>]<Output<PushPull>, Filter, PullState> {
                    let port = unsafe { &*[<PIO $port:upper>]::ptr() };
                    // set pin as push-pull
                    port.mddr.write_with_zero(|w| w.[<p $pin_id>]().set_bit());

                    // set default value
                    if is_default_high {
                        port.sodr.write_with_zero(|w| w.[<p $pin_id>]().set_bit());
                    } else {
                        port.codr.write_with_zero(|w| w.[<p $pin_id>]().set_bit());
                    }

                    // Configure pin as output
                    port.oer.write_with_zero(|w| w.[<p $pin_id>]().set_bit());
                    port.per.write_with_zero(|w| w.[<p $pin_id>]().set_bit());
                    [<P $port:lower $pin_id>] { _mode: PhantomData, _pull: PhantomData, _filter: PhantomData }
                }
            }
            impl<Mode, Filter, PullState> [<P $port:lower $pin_id>]<Mode, Filter, PullState> {
                pub fn into_function_a(self)->
                    [<P $port:lower $pin_id>]<Alternate<PfA>, Filter, PullState> {
                    let port = unsafe { &*[<PIO $port:upper>]::ptr() };
                    // Configure pin as output
                    port.abcdsr[0].modify(|_, w| w.[<p $pin_id>]().clear_bit());
                    port.abcdsr[1].modify(|_, w| w.[<p $pin_id>]().clear_bit());

                    port.odr.write_with_zero(|w| w.[<p $pin_id>]().set_bit());
                    port.pdr.write_with_zero(|w| w.[<p $pin_id>]().set_bit());
                    [<P $port:lower $pin_id>] { _mode: PhantomData, _pull: PhantomData, _filter: PhantomData }
                }
            }
            impl<Mode, Filter, PullState> [<P $port:lower $pin_id>]<Mode, Filter, PullState> {
                pub fn into_function_b(self)->
                    [<P $port:lower $pin_id>]<Alternate<PfB>, Filter, PullState> {
                    let port = unsafe { &*[<PIO $port:upper>]::ptr() };
                    // Configure pin as output
                    port.abcdsr[0].modify(|_, w| w.[<p $pin_id>]().set_bit());
                    port.abcdsr[1].modify(|_, w| w.[<p $pin_id>]().clear_bit());

                    port.odr.write_with_zero(|w| w.[<p $pin_id>]().set_bit());
                    port.pdr.write_with_zero(|w| w.[<p $pin_id>]().set_bit());
                    [<P $port:lower $pin_id>] { _mode: PhantomData, _pull: PhantomData, _filter: PhantomData }
                }
            }
            impl<Mode, Filter, PullState> [<P $port:lower $pin_id>]<Mode, Filter, PullState> {
                pub fn into_function_c(self)->
                    [<P $port:lower $pin_id>]<Alternate<PfC>, Filter, PullState> {
                    let port = unsafe { &*[<PIO $port:upper>]::ptr() };
                    // Configure pin as output
                    port.abcdsr[0].modify(|_, w| w.[<p $pin_id>]().clear_bit());
                    port.abcdsr[1].modify(|_, w| w.[<p $pin_id>]().set_bit());

                    port.odr.write_with_zero(|w| w.[<p $pin_id>]().set_bit());
                    port.pdr.write_with_zero(|w| w.[<p $pin_id>]().set_bit());
                    [<P $port:lower $pin_id>] { _mode: PhantomData, _pull: PhantomData, _filter: PhantomData }
                }
            }
        }
    };
}

macro_rules! port {
    ($($port:ident, $port_id:expr, { $( $pin_id:expr ),+ }),+) => {
        $crate::paste! {
            $(
            pub mod [<pio $port:lower>] {
                use super::*;

                use core::marker::PhantomData;
                use core::convert::Infallible;

                use embedded_hal::digital::v2::{InputPin, OutputPin, StatefulOutputPin, toggleable};

                use $crate::pac::[<PIO $port:upper>];


                $(
                    pin!($port, $port_id, $pin_id);
                 )+

                pub struct Parts {
                    $(
                        pub [<p $port:lower $pin_id>]: [<P $port:lower $pin_id>]<Input, Floating, Unfiltered>
                     ),+
                }
                impl GpioExt for $crate::pac::[<PIO $port:upper>] {
                    type Parts = Parts;

                    fn split(self) -> Parts {
                        let pmc = unsafe { &(*$crate::pac::PMC::ptr()) };
                        pmc.pmc_pcer0.write_with_zero(|w| w.[<pid $port_id>]().set_bit());

                        Parts {
                            $(
                                [<p $port:lower $pin_id>]: [<P $port:lower $pin_id>] {
                                    _mode: PhantomData,
                                    _pull: PhantomData,
                                    _filter: PhantomData
                                }
                             ),+
                        }
                    }
                }
            })+
        }
    };
}

// TODO: Pb 4, 5, 6, 7, 10, 11 and 12 are system io by default.
// It might be better not to obtain them from the port but from the matrix

port!(
    A, 9,  { 0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31 },
    B, 10, { 0,1,2,3,4,5,6,7,8,9,10,11,12,13,14 },
    C, 11, { 0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31 },
    D, 12, { 0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31 },
    E, 13, { 0,1,2,3,4,5 }
);
