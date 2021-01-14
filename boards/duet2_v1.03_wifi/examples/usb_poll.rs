//! Makes the pygamer appear as a USB serial port loop back device.
//! Repeats back all characters sent to it, but in upper case.

#![no_std]
#![no_main]

//use panic_halt as _;
#[panic_handler]
fn on_panic(info: &core::panic::PanicInfo) -> ! {
    atsam4e_hal::dbgprint!("Woops: {:?}", info);
    loop {}
}

use atsam4e_hal::gpio::GpioExt;
use atsam4e_hal::pmc::{MainClock, PmcExt};
use atsam4e_hal::serial::*;
use atsam4e_hal::time::U32Ext;
use atsam4e_hal::usb::*;

use embedded_hal::digital::v2::OutputPin;
use usb_device::prelude::*;
use usbd_serial::{SerialPort, /* CDC_SUBCLASS_ACM,*/ USB_CLASS_CDC};

#[cortex_m_rt::entry]
fn main() -> ! {
    // Get access to the device specific peripherals from the peripheral access crate
    let p = atsam4e_hal::pac::Peripherals::take().unwrap_or_else(|| unreachable!());
    //let cp = cortex_m::Peripherals::take().unwrap_or_else(|| unreachable!());

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

    let usb_bus = usb_device::bus::UsbBusAllocator::new(UsbBus::new(p.UDP, (), (), clocks));

    let mut serial = SerialPort::new(&usb_bus);

    let mut led = p.PIOC.split().pc16.into_push_pull_output(false);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .manufacturer("Fake company")
        .product("Serial port")
        .serial_number("TEST")
        .device_class(USB_CLASS_CDC)
        //.device_class(CDC_SUBCLASS_ACM)
        .device_class(2)
        .build();

    let mut prev = usb_dev.ctrl_pipe_state();
    loop {
        let new = usb_dev.ctrl_pipe_state();
        if prev != new {
            prev = new;
            //atsam4e_hal::dbgprint!("{:?}\n", prev)
        }

        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }

        let mut buf = [0u8; 64];

        match serial.read(&mut buf) {
            Ok(count) if count > 0 => {
                let _ = led.set_high(); // Turn on

                // Echo back in upper case
                for c in buf[0..count].iter_mut() {
                    if 0x61 <= *c && *c <= 0x7a {
                        *c &= !0x20;
                    }
                }

                let mut write_offset = 0;
                while write_offset < count {
                    match serial.write(&buf[write_offset..count]) {
                        Ok(len) if len > 0 => {
                            write_offset += len;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }

        let _ = led.set_low(); // Turn off
    }
}
