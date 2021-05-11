//! Makes the duet2 appear as a USB serial port loop back device with dfu support.
//! Repeats back all characters sent to it, but in upper case.
//! Uppon dfu request it gets ready for re-enumeration and writes/reads data from the last
//! flash sector.

#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(maybe_uninit_ref)]
#![feature(panic_info_message)]

extern crate alloc;

use alloc_cortex_m::CortexMHeap;
use atsam4e_hal::dbgprint;
use atsam4e_hal::gpio::GpioExt;
use atsam4e_hal::pmc::{MainClock, PmcExt};
use atsam4e_hal::time::U32Ext;
use atsam4e_hal::usb::*;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::exception;
use embedded_hal::digital::v2::OutputPin;
use usb_device::prelude::*;
use usbd_dfu::runtime::DFURuntimeClass;
use usbd_dfu_demo::DFURuntimeImpl;
use usbd_serial::{SerialPort, /* CDC_SUBCLASS_ACM,*/ USB_CLASS_CDC};

fn reset() -> ! {
    let rstc = unsafe { &*atsam4e_hal::pac::RSTC::ptr() };
    rstc.cr.write_with_zero(|w| {
        w.procrst()
            .set_bit()
            .perrst()
            .set_bit()
            .key()
            .variant(atsam4e_hal::pac::rstc::cr::KEY_AW::PASSWD)
    });
    loop {
        cortex_m::asm::nop();
    }
}
#[panic_handler]
fn on_panic(info: &core::panic::PanicInfo) -> ! {
    unsafe {
        use core::fmt::Write;
        let err = ERROR.assume_init_mut();
        if let Some(msg) = info.message() {
            let _ = write!(err, "{}", msg);
        } else {
            let _ = write!(err, "Woops {:#?}", info);
        }
        reset();
    }
}
#[alloc_error_handler]
fn oom(layout: core::alloc::Layout) -> ! {
    panic!(
        "oom with: {:?}\r\nused: {}\r\nfree: {}\r\n",
        layout,
        ALLOCATOR.used(),
        ALLOCATOR.free()
    );
}
#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[exception]
#[allow(non_snake_case)]
fn HardFault(ef: &cortex_m_rt::ExceptionFrame) -> ! {
    panic!("HardFault: {:#?}", ef);
}

#[exception]
#[allow(non_snake_case)]
unsafe fn DefaultHandler(irqn: i16) {
    panic!("DefaultHandler: {}", irqn);
}

struct LastPanicMessage {
    len: usize,
    buffer: [u8; 1024],
}
impl core::fmt::Write for LastPanicMessage {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let len = core::cmp::min(s.len(), self.buffer.len());
        let start = self.len;
        let end = start + len;
        self.buffer[start..end].copy_from_slice(&s.as_bytes()[..len]);
        self.len = end;
        Ok(())
    }
}

#[link_section = ".uninit"]
static mut ERROR: core::mem::MaybeUninit<LastPanicMessage> = core::mem::MaybeUninit::uninit();

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

    // Initialize the allocator BEFORE you use it
    let start = cortex_m_rt::heap_start() as usize;
    let size = 80 * 1024; // in bytes
    unsafe { ALLOCATOR.init(start, size) };

    let mut led = p.PIOC.split().pc16.into_push_pull_output(false);

    cp.SYST.set_clock_source(SystClkSource::External);
    cp.SYST.set_reload(clocks.master_clock.0 / (8 * 1_000));
    cp.SYST.clear_current();
    cp.SYST.enable_counter();

    let usb_bus = usb_device::bus::UsbBusAllocator::new(UsbBus::new(p.UDP, (DDP, DDM), clocks));
    let mut serial = SerialPort::new_with_store(
        &usb_bus,
        unsafe { core::mem::MaybeUninit::<[u8; 128]>::uninit().assume_init() },
        unsafe { core::mem::MaybeUninit::<[u8; 1024]>::uninit().assume_init() },
    );
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

    let mut efc = p.EFC;
    let mut timestamp = 0u64;
    let mut prev_ts = 0;

    duet2_v1_03_wifi::executor::block_on(async move {
        loop {
            if cp.SYST.has_wrapped() {
                dfu.poll(1);
                timestamp += 1;
            }

            usb_dev.poll(&mut [&mut serial, &mut dfu]);
            let mut buf = [0u8; 256];

            let mut count = match serial.read(&mut buf) {
                Ok(count) => {
                    let _ = led.set_low(); // Turn on

                    // Echo back in upper case
                    buf.iter_mut().take(count).for_each(|c| {
                        if let 0x61..=0x7a = *c {
                            *c &= !0x20;
                        }
                    });
                    count
                }
                _ => 0,
            };

            if let &[0x0D, ..] = &buf {
                // read flash desc
                let info = usbd_dfu_demo::FlashInfo::read(&mut efc).await;
                dbgprint!("{:#?}", info);
            }
            if prev_ts != timestamp {
                prev_ts = timestamp;
                if (timestamp % 1000) == 0 {
                    dbgprint!("{}\r\n", timestamp / 1000);
                    if timestamp == 2000 {
                        unsafe {
                            let err = ERROR.assume_init_mut();
                            if err.len > 0 {
                                dbgprint!(
                                    "{} \"{}\"\r\n",
                                    err.len,
                                    core::str::from_utf8_unchecked(&err.buffer[..err.len])
                                );
                                err.len = 0;
                            }
                        };
                    }
                }
            }
            atsam4e_hal::debug_on_buffer::consume(|dbg| {
                let len = core::cmp::min(dbg.len(), buf.len() - count);
                buf[count..count + len].copy_from_slice(&dbg[..len]);
                count += len;
                len
            });
            if count == 0 {
                continue;
            }

            let mut wr_ptr = &buf[..count];
            while !wr_ptr.is_empty() {
                let _ = serial.write(wr_ptr).map(|len| {
                    wr_ptr = &wr_ptr[len..];
                });
            }

            let _ = led.set_high(); // Turn off
        }
    });
    unreachable!();
}
