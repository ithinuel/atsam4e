#![no_std]
#![no_main]

use panic_halt as _;

use atsam4_hal::delay::Delay;
use atsam4_hal::gpio::GpioExt;
use atsam4_hal::pmc::{MainClock, PmcExt};
use atsam4_hal::time::U32Ext;
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v2::ToggleableOutputPin;

#[cortex_m_rt::entry]
fn main() -> ! {
    // Get access to the device specific peripherals from the peripheral access crate
    let p = atsam4_hal::pac::Peripherals::take().unwrap_or_else(|| unreachable!());
    let cp = cortex_m::Peripherals::take().unwrap_or_else(|| unreachable!());

    p.WDT.mr.write(|w| w.wddis().set_bit());

    // configure the clocks
    let pmc = p.PMC.constrain(); // constrain comes form a trait in the sam4e hal

    // Freeze the configuration of all the clocks in the system and store
    // the frozen frequencies in `clocks`
    let clocks = pmc
        .main_clock(MainClock::External(12.mhz().into()))
        .master_clock(120.mhz())
        .freeze();

    // Acquire the GPIOC peripheral
    let pioc = p.PIOC.split();

    let mut delay = Delay::new(cp.SYST, clocks).ok().unwrap();

    let mut e1_stop = pioc.pc16.into_push_pull_output(false);
    loop {
        delay.delay_ms(500.ms());

        e1_stop.toggle().unwrap_or(());
    }
}
