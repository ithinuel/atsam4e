//! Makes the duet2 appear as a USB serial port loop back device with dfu support.
//! Repeats back all characters sent to it, but in upper case.
//! Uppon dfu request it gets ready for re-enumeration and writes/reads data from the last
//! flash sector.

#![no_std]
#![no_main]

use atsam4e_hal::pmc::{MainClock, PmcExt};
use atsam4e_hal::time::U32Ext;
use cortex_m::peripheral::syst::SystClkSource;
use panic_halt as _;

use atsam4e_hal::usb::*;
use usb_device::prelude::*;
use usbd_dfu::mode::DFUModeClass;
use usbd_dfu_demo::DFUModeImpl;

#[cortex_m_rt::entry]
fn main() -> ! {
    // Get access to the device specific peripherals from the peripheral access crate
    let p = atsam4e_hal::pac::Peripherals::take().unwrap_or_else(|| unreachable!());
    let mut cp = cortex_m::Peripherals::take().unwrap_or_else(|| unreachable!());

    p.WDT.mr.write(|w| w.wddis().set_bit());

    // configure the clocks
    let pmc = p.PMC.constrain(); // constrain comes form a trait in the sam4e hal

    // Freeze the configuration of all the clocks in the system and store
    // the frozen frequencies in `clocks`
    let clocks = pmc
        .main_clock(MainClock::External(12.mhz().into()))
        .master_clock(120.mhz())
        .use_usb()
        .freeze();

    // Configures Systick to wrap every millisecond.
    cp.SYST.set_clock_source(SystClkSource::External);
    cp.SYST.set_reload(clocks.master_clock.0 / (8 * 1_000)); // systick has a /8 clock divider
    cp.SYST.clear_current();
    cp.SYST.enable_counter();

    // Setup the Usb stack
    let usb_bus = usb_device::bus::UsbBusAllocator::new(UsbBus::new(p.UDP, (DDP, DDM), clocks));
    let mut dfu = DFUModeClass::new(&usb_bus, DFUModeImpl::new(p.EFC));
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .manufacturer("Fake company")
        .product("Serial port")
        .serial_number("TEST")
        .max_packet_size_0(64)
        .build();

    loop {
        if cp.SYST.has_wrapped() {
            dfu.poll(1);
        }

        usb_dev.poll(&mut [&mut dfu]);
    }
}
