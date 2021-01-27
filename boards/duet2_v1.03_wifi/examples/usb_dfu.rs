//! Makes the duet2 appear as a USB serial port loop back device with dfu support.
//! Repeats back all characters sent to it, but in upper case.
//! Uppon dfu request it gets ready for re-enumeration and writes/reads data from the last
//! flash sector.

#![no_std]
#![no_main]

use atsam4e_hal::gpio::GpioExt;
use atsam4e_hal::pmc::{MainClock, PmcExt};
use atsam4e_hal::time::U32Ext;
use atsam4e_hal::usb::*;
use cortex_m::peripheral::syst::SystClkSource;
use embedded_hal::digital::v2::OutputPin;
use usb_device::prelude::*;
use usbd_dfu::runtime::DFURuntimeClass;
use usbd_dfu_demo::DFURuntimeImpl;
use usbd_serial::{SerialPort, /* CDC_SUBCLASS_ACM,*/ USB_CLASS_CDC};

#[cfg(not(feature = "debug_on_uart0"))]
use panic_halt as _;
#[cfg(feature = "debug_on_uart0")]
#[panic_handler]
fn on_panic(info: &core::panic::PanicInfo) -> ! {
    atsam4e_hal::dbgprint!("Woops: {:?}", info);
    loop {}
}
#[cfg(feature = "debug_on_uart0")]
use atsam4e_hal::serial::*;

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

    let mut led = p.PIOC.split().pc16.into_push_pull_output(false);

    #[cfg(feature = "debug_on_uart0")]
    {
        let ioa = p.PIOA.split();

        let tx = ioa.pa10.into_function_a();
        let (tx, _) = Serial::uart0(
            p.UART0,
            (tx, NoRx),
            config::Config::default().baudrate(3_750_000.bps()),
            clocks,
        )
        .map(Serial::<atsam4e_hal::pac::UART0, _>::split)
        .unwrap_or_else(|_| unreachable!());

        atsam4e_hal::debug::wire_uart(tx);
    }

    cp.SYST.set_clock_source(SystClkSource::External);
    cp.SYST.set_reload(clocks.master_clock.0 / (8 * 1_000));
    cp.SYST.clear_current();
    cp.SYST.enable_counter();

    let usb_bus = usb_device::bus::UsbBusAllocator::new(UsbBus::new(p.UDP, (DDP, DDM), clocks));
    let mut serial = SerialPort::new(&usb_bus);
    let mut dfu = DFURuntimeClass::new(&usb_bus, DFURuntimeImpl);
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .manufacturer("Fake company")
        .product("Serial port")
        .serial_number("TEST")
        .max_packet_size_0(64)
        .device_class(USB_CLASS_CDC)
        //.device_sub_class(CDC_SUBCLASS_ACM)
        .device_sub_class(2)
        //.device_protocol(CDC_PROTOCOL_NONE)
        .device_protocol(0)
        .build();

    loop {
        if cp.SYST.has_wrapped() {
            dfu.poll(1);
        }

        if !usb_dev.poll(&mut [&mut serial, &mut dfu]) {
            continue;
        }

        let mut buf = [0u8; 64];

        let _ = serial.read(&mut buf).map(|count| {
            if count == 0 {
                return;
            }
            let _ = led.set_low(); // Turn on

            // Echo back in upper case
            buf.iter_mut().take(count).for_each(|c| {
                if let 0x61..=0x7a = *c {
                    *c &= !0x20;
                }
            });

            let mut wr_ptr = &buf[..count];
            while !wr_ptr.is_empty() {
                let _ = serial.write(wr_ptr).map(|len| {
                    wr_ptr = &wr_ptr[len..];
                });
            }
        });

        let _ = led.set_high(); // Turn off
    }
}
