use usb_device::{
    bus::{PollResult, UsbBus as UsbBusTrait},
    endpoint::{EndpointAddress, EndpointType},
    Result as UsbResult, UsbDirection, UsbError,
};

use crate::pac::udp::csr::{EPTYPE_A, W as CSRWrite};

use crate::pmc::Clocks;

use core::cell::RefCell;
use cortex_m::interrupt::{self, Mutex};

fn usb() -> &'static crate::pac::udp::RegisterBlock {
    unsafe { &*crate::pac::UDP::ptr() }
}

#[inline]
fn no_effect(w: &mut CSRWrite) -> &mut CSRWrite {
    w.txcomp()
        .set_bit()
        .rx_data_bk0()
        .set_bit()
        .rxsetup()
        .set_bit()
        .stallsent()
        .set_bit()
        .rx_data_bk1()
        .set_bit()
}

struct EndpointMetadata {
    max_packet_size: u16,
    types: [EndpointType; 3],
}

const EP_METADATA: [EndpointMetadata; 8] = [
    EndpointMetadata {
        max_packet_size: 64,
        types: [
            EndpointType::Control,
            EndpointType::Bulk,
            EndpointType::Interrupt,
        ],
    },
    EndpointMetadata {
        max_packet_size: 64,
        types: [
            EndpointType::Isochronous,
            EndpointType::Bulk,
            EndpointType::Interrupt,
        ],
    },
    EndpointMetadata {
        max_packet_size: 64,
        types: [
            EndpointType::Isochronous,
            EndpointType::Bulk,
            EndpointType::Interrupt,
        ],
    },
    EndpointMetadata {
        max_packet_size: 64,
        types: [
            EndpointType::Control,
            EndpointType::Bulk,
            EndpointType::Interrupt,
        ],
    },
    EndpointMetadata {
        max_packet_size: 512,
        types: [
            EndpointType::Isochronous,
            EndpointType::Bulk,
            EndpointType::Interrupt,
        ],
    },
    EndpointMetadata {
        max_packet_size: 512,
        types: [
            EndpointType::Isochronous,
            EndpointType::Bulk,
            EndpointType::Interrupt,
        ],
    },
    EndpointMetadata {
        max_packet_size: 64,
        types: [
            EndpointType::Isochronous,
            EndpointType::Bulk,
            EndpointType::Interrupt,
        ],
    },
    EndpointMetadata {
        max_packet_size: 64,
        types: [
            EndpointType::Isochronous,
            EndpointType::Bulk,
            EndpointType::Interrupt,
        ],
    },
];

#[derive(Debug, Copy, Clone)]
enum Bank {
    Bank0,
    Bank1,
}

#[derive(Debug, Copy, Clone)]
struct EndpointInfo {
    ep_type: EPTYPE_A,
    max_packet_size: u16,

    /// indicates if last write was max_packet_size. In such case more write operations are
    /// expected and txcomp must not be cleared until the next write occurs.
    expect_more_writes: bool,
    active_bank: Bank,

    /// Used for debug purposes
    last_csr: u32,
}
impl EndpointInfo {
    fn data_ready(&self, csr_val: &crate::pac::udp::csr::R) -> bool {
        match self.active_bank {
            Bank::Bank0 => csr_val.rx_data_bk0().bit_is_set(),
            Bank::Bank1 => csr_val.rx_data_bk1().bit_is_set(),
        }
    }
    fn clear_data_ready(&mut self, csr: &crate::pac::udp::CSR) {
        match self.active_bank {
            Bank::Bank0 => {
                csr.modify(|_, w| no_effect(w).rx_data_bk0().clear_bit());
                while csr.read().rx_data_bk0().bit_is_set() {}

                if self.ep_type != EPTYPE_A::CTRL {
                    self.active_bank = Bank::Bank1
                }
            }
            Bank::Bank1 => {
                csr.modify(|_, w| no_effect(w).rx_data_bk1().clear_bit());
                while csr.read().rx_data_bk1().bit_is_set() {}
                self.active_bank = Bank::Bank0
            }
        };
    }
}

#[derive(Debug)]
struct AllEndpoints([Option<EndpointInfo>; 8]);
impl AllEndpoints {
    fn find_free(
        &self,
        dir: UsbDirection,
        ep_type: EndpointType,
        max_packet_size: u16,
    ) -> UsbResult<EndpointAddress> {
        // EP0  single bank    64   Control/Bulk/Interrupt
        // EP1    dual bank    64   Bulk/Isochonous/Interrupt
        // EP2    dual bank    64   Bulk/Isochonous/Interrupt
        // EP3  single bank    64   Control/Bulk/Interrupt
        // EP4    dual bank   512   Bulk/Isochonous/Interrupt
        // EP5    dual bank   512   Bulk/Isochonous/Interrupt
        // EP6    dual bank    64   Bulk/Isochonous/Interrupt
        // EP7    dual bank    64   Bulk/Isochonous/Interrupt

        // Lookup for an appropriate endpoint:
        // - check for best max_packet_size fit first.
        //   leave the 2 biggest for the end to give a chance to smaller EP to be assigned to EP6
        //   or 7.
        // - keep 3 for late ctrl registration (I don't know if it's possible to have 2 ctrl
        //   channels on usb, but that's just in case).
        for &idx in &[1, 2, 6, 7, 4, 5, 3] {
            let meta = &EP_METADATA[idx];
            let info = &self.0[idx];
            if info.is_none()
                && meta.max_packet_size >= max_packet_size
                && meta.types.contains(&ep_type)
            {
                return Ok(EndpointAddress::from_parts(idx, dir));
            }
        }
        Err(UsbError::EndpointOverflow)
    }

    fn allocate(
        &mut self,
        ep_addr: EndpointAddress,
        ep_type: EndpointType,
        max_packet_size: u16,
    ) -> UsbResult<()> {
        let idx = ep_addr.index();
        let meta = &EP_METADATA[idx];
        let info = &self.0[idx];

        // TODO:deal with multiple ctrl ep and in/out on the same but making sure we're not
        // allocating the same ep+dir twice
        if info.is_some() && ep_addr.index() != 0
            || meta.max_packet_size < max_packet_size
            || !meta.types.contains(&ep_type)
        {
            return Err(UsbError::InvalidEndpoint);
        }

        let ep_type = match (ep_type, ep_addr.direction()) {
            (EndpointType::Isochronous, UsbDirection::In) => EPTYPE_A::ISO_IN,
            (EndpointType::Isochronous, UsbDirection::Out) => EPTYPE_A::ISO_OUT,
            (EndpointType::Interrupt, UsbDirection::In) => EPTYPE_A::INT_IN,
            (EndpointType::Interrupt, UsbDirection::Out) => EPTYPE_A::INT_OUT,
            (EndpointType::Bulk, UsbDirection::In) => EPTYPE_A::BULK_IN,
            (EndpointType::Bulk, UsbDirection::Out) => EPTYPE_A::BULK_OUT,
            (EndpointType::Control, _) => EPTYPE_A::CTRL,
        };

        self.0[idx] = Some(EndpointInfo {
            ep_type,
            max_packet_size,
            expect_more_writes: false,
            active_bank: Bank::Bank0,
            last_csr: 0,
        });

        Ok(())
    }

    fn configure(&self) {
        for (i, info) in self
            .0
            .iter()
            .enumerate()
            .filter_map(|(idx, opt_ep_info)| opt_ep_info.map(|ep_info| (idx, ep_info)))
        {
            let csr = &usb().csr()[i];
            csr.modify(|_, w| {
                // set NoEffect bits
                no_effect(w)
                    // enable endpoint
                    .epeds()
                    .set_bit()
                    // set endpoint type & direction
                    .eptype()
                    .variant(info.ep_type)
            });
            while csr.read().eptype() != info.ep_type {}
        }
    }
}

struct Inner {
    endpoints: AllEndpoints,
    /// Previous isr
    last_isr: u32,
}
impl Inner {
    fn enumerate_active_ep_mut<'a>(
        &'a mut self,
    ) -> impl Iterator<Item = (usize, &'a mut EndpointInfo)> {
        self.endpoints
            .0
            .iter_mut()
            .enumerate()
            .filter_map(|(idx, opt_ep_info)| match opt_ep_info {
                None => None,
                Some(ep_info) => Some((idx, ep_info)),
            })
    }
    fn enumerate_active_ep<'a>(&'a self) -> impl Iterator<Item = (usize, &'a EndpointInfo)> {
        self.endpoints
            .0
            .iter()
            .enumerate()
            .filter_map(|(idx, opt_ep_info)| match opt_ep_info {
                None => None,
                Some(ep_info) => Some((idx, ep_info)),
            })
    }
    fn get_ep_mut(&mut self, ep_addr: EndpointAddress) -> Option<&mut EndpointInfo> {
        self.endpoints
            .0
            .get_mut(ep_addr.index())
            .and_then(Option::as_mut)
    }
    fn get_ep(&self, ep_addr: EndpointAddress) -> Option<&EndpointInfo> {
        self.endpoints
            .0
            .get(ep_addr.index())
            .and_then(Option::as_ref)
    }
}

pub struct UsbBus {
    inner: Mutex<RefCell<Inner>>,
}

impl UsbBus {
    // TODO: actually take the pins in the right mode
    pub fn new(_usb: crate::pac::UDP, _ddp: (), _ddm: (), clocks: Clocks) -> Self {
        dbgprint!("New USB Bus\n");
        assert!(clocks.usb_clock_enabled);

        // enable usb clock on pmc
        let pmc = unsafe { &*crate::pac::PMC::ptr() };
        pmc.pmc_scer.write_with_zero(|w| w.udp().set_bit());
        pmc.pmc_pcer1.write_with_zero(|w| w.pid35().set_bit());

        Self {
            inner: Mutex::new(RefCell::new(Inner {
                endpoints: AllEndpoints([None; 8]),
                last_isr: 0,
            })),
        }
    }
}

impl UsbBusTrait for UsbBus {
    fn alloc_ep(
        &mut self,
        ep_dir: UsbDirection,
        ep_addr: Option<EndpointAddress>,
        ep_type: EndpointType,
        max_packet_size: u16,
        _interval: u8,
    ) -> UsbResult<EndpointAddress> {
        dbgprint!(
            "UsbBus::alloc_ep dir={:?} addr={:?} type={:?} max_packet_size={} interval={} -> ",
            ep_dir,
            ep_addr,
            ep_type,
            max_packet_size,
            _interval
        );
        interrupt::free(|cs| {
            let mut inner = self.inner.borrow(cs).borrow_mut();

            let ep_addr = match ep_addr {
                None => inner
                    .endpoints
                    .find_free(ep_dir, ep_type, max_packet_size)?,
                Some(addr) => EndpointAddress::from(addr),
            };

            inner
                .endpoints
                .allocate(ep_addr, ep_type, max_packet_size)?;
            dbgprint!(
                "EP{}-{:?}: {:?}\n",
                ep_addr.index(),
                ep_addr.direction(),
                inner.get_ep(ep_addr)
            );
            Ok(ep_addr)
        })
    }

    fn enable(&mut self) {
        dbgprint!("UsbBus::enable\n");
        usb().txvc.modify(|_, w| w.puon().set_bit());
    }
    fn reset(&self) {
        dbgprint!("UsbBus::reset\n");
        interrupt::free(|cs| {
            let inner = self.inner.borrow(cs).borrow_mut();

            inner.endpoints.configure();

            usb().ier.write_with_zero(|w| w.ep0int().set_bit());
            usb().txvc.modify(|_, w| w.txvdis().clear_bit());
        });
    }

    fn set_device_address(&self, addr: u8) {
        dbgprint!("UsbBus::set_device_address({})\n", addr);
        let usb = usb();
        usb.faddr
            .modify(|_, w| unsafe { w.fadd().bits(addr).fen().set_bit() });
        usb.glb_stat
            .write_with_zero(|w| w.fadden().set_bit().confg().set_bit());

        /*interrupt::free(|cs| {
            let inner = self.inner.borrow(cs).borrow();
            // skip ep 0
            for ep_idx in inner.enumerate_active_ep().skip(1).map(|(idx, _)| idx) {
                usb.ier.write_with_zero(|w| match ep_idx {
                    1 => w.ep1int().set_bit(),
                    2 => w.ep2int().set_bit(),
                    3 => w.ep3int().set_bit(),
                    4 => w.ep4int().set_bit(),
                    5 => w.ep5int().set_bit(),
                    6 => w.ep6int().set_bit(),
                    7 => w.ep7int().set_bit(),
                    _ => {
                        dbgprint!("idx: {}", ep_idx);
                        unreachable!()
                    }
                })
            }
        })*/
    }

    fn write(&self, ep_addr: EndpointAddress, buf: &[u8]) -> UsbResult<usize> {
        dbgprint!(
            "UsbBus::write({}-{:?}, {:x?}) -> ",
            ep_addr.index(),
            ep_addr.direction(),
            buf
        );

        let ret = cortex_m::interrupt::free(|cs| {
            let mut inner = self.inner.borrow(cs).borrow_mut();

            let ep_info = match inner.get_ep_mut(ep_addr) {
                Some(ep_info) if ep_addr.is_in() => ep_info,
                _ => {
                    return Err(UsbError::InvalidEndpoint);
                }
            };

            let csr = &usb().csr()[ep_addr.index()];
            let csr_val = csr.read();

            dbgprint!(
                "(csr: {:08b}:{}) ",
                csr_val.bits() & 0xFF,
                csr_val.rxbytecnt().bits()
            );

            if csr_val.txpktrdy().bit_is_set() {
                return Err(UsbError::WouldBlock);
            }

            // TODO: check that buf is not too long for this EP

            // If ctrl ep: switch direction
            if ep_info.ep_type == EPTYPE_A::CTRL {
                if csr_val.dir().bit_is_clear() {
                    csr.modify(|_, w| no_effect(w).dir().set_bit());
                    while csr.read().dir().bit_is_clear() {}
                }
            }

            let fdr = &usb().fdr[ep_addr.index()];
            for b in buf {
                fdr.write_with_zero(|w| unsafe { w.fifo_data().bits(*b) });
            }

            csr.modify(|_, w| no_effect(w).txpktrdy().set_bit());
            while csr.read().txpktrdy().bit_is_clear() {}
            csr.modify(|_, w| no_effect(w).txcomp().clear_bit());
            while csr.read().txcomp().bit_is_set() {}

            // TODO: replace buf.len() with MAX_CTRL_EP_LEN
            ep_info.expect_more_writes = buf.len() == ep_info.max_packet_size.into();

            let csr_val = csr.read();
            dbgprint!(
                "(csr: {:08b}:{}) ",
                csr_val.bits() & 0xFF,
                csr_val.rxbytecnt().bits()
            );
            Ok(buf.len())
        });
        dbgprint!("{:?}\n", ret);
        ret
    }
    fn read(&self, ep_addr: EndpointAddress, buf: &mut [u8]) -> UsbResult<usize> {
        dbgprint!(
            "UsbBus::read({}-{:?}, {}) -> ",
            ep_addr.index(),
            ep_addr.direction(),
            buf.len()
        );

        let ret = interrupt::free(|cs| {
            let mut inner = self.inner.borrow(cs).borrow_mut();

            let ep_info = match inner.get_ep_mut(ep_addr) {
                Some(ep_info) if ep_addr.is_out() => ep_info,
                _ => return Err(UsbError::InvalidEndpoint),
            };

            let csr = &usb().csr()[ep_addr.index()];
            let csr_val = csr.read();

            dbgprint!(
                "({:?} csr: {:08b}:{}) ",
                ep_info.active_bank,
                csr_val.bits() & 0xFF,
                csr_val.rxbytecnt().bits()
            );

            if !ep_info.data_ready(&csr_val) && csr_val.rxsetup().bit_is_clear() {
                return Err(UsbError::WouldBlock);
            }

            // filter Time-of-check cases where ZLP OUT might be discarded by hardware by a setup
            // packet between UsbBus::poll and UsbBus::read
            if ep_info.ep_type == EPTYPE_A::CTRL && buf.is_empty() {
                ep_info.clear_data_ready(csr);
                return Err(UsbError::WouldBlock);
            }

            let bytecnt = csr_val.rxbytecnt().bits().into();
            if bytecnt > buf.len() {
                return Err(UsbError::BufferOverflow);
            }

            let fdr = &usb().fdr[ep_addr.index()];
            for (_, pbyte) in (0..bytecnt).zip(buf.iter_mut()) {
                *pbyte = fdr.read().fifo_data().bits();
            }

            if csr_val.rxsetup().bit_is_set() {
                csr.modify(|_, w| no_effect(w).rxsetup().clear_bit());
                while csr.read().rxsetup().bit_is_set() {}
            } else if ep_info.data_ready(&csr_val) {
                ep_info.clear_data_ready(csr);
            }

            let csr_val = csr.read();
            dbgprint!(
                "(csr: {:08b}:{}) ",
                csr_val.bits() & 0xFF,
                csr_val.rxbytecnt().bits()
            );

            Ok(&buf[..bytecnt])
        });

        dbgprint!("{:x?}\n", ret);
        ret.map(|v| v.len())
    }

    // TODO: Not sure this is done correctly
    fn set_stalled(&self, ep_addr: EndpointAddress, stalled: bool) {
        dbgprint!(
            "UsbBus::set_stalled({}-{:?}, {})\n",
            ep_addr.index(),
            ep_addr.direction(),
            stalled
        );

        let csr = &usb().csr()[ep_addr.index()];

        if stalled {
            csr.modify(|_, w| no_effect(w).forcestall().set_bit());
            // during data stage and status stage on ctrl ep, indicates the request cannot be
            // completed
            while csr.read().stallsent().bit_is_clear() {}
        } else {
            csr.modify(|_, w| {
                no_effect(w)
                    .forcestall()
                    .clear_bit()
                    .stallsent()
                    .clear_bit()
            });
            loop {
                let csr_val = csr.read();
                if csr_val.forcestall().bit_is_clear() && csr_val.stallsent().bit_is_clear() {
                    break;
                }
            }
        }
    }
    fn is_stalled(&self, ep_addr: EndpointAddress) -> bool {
        dbgprint!(
            "UsbBus::is_stalled({}-{:?})\n",
            ep_addr.index(),
            ep_addr.direction(),
        );
        let csr_val = usb().csr()[ep_addr.index()].read();
        csr_val.forcestall().bit_is_set() || csr_val.stallsent().bit_is_set()
    }

    fn suspend(&self) {
        //dbgprint!("UsbBus::suspend\n");
        // TODO: We need to delegate that to the application as it may not allow to mess with
        // clocks
        //usb().txvc.modify(|_, w| w.txvdis().set_bit());
    }
    fn resume(&self) {
        //dbgprint!("UsbBus::resume\n");
        // TODO: We need to delegate that to the application as it may not allow to mess with
        // clocks
        //usb().txvc.modify(|_, w| w.txvdis().clear_bit());
    }

    fn poll(&self) -> PollResult {
        cortex_m::interrupt::free(|cs| {
            let usb = usb();
            let mut inner = self.inner.borrow(cs).borrow_mut();
            // ignore sof and wakeup
            let mut print_dbg = false;
            let isr_val = usb.isr.read();
            let new_isr = isr_val.bits() & 0x000037FF;
            if inner.last_isr != new_isr {
                inner.last_isr = new_isr;
                print_dbg = true;
            }

            // ignore sof and wakeup
            usb.icr
                .write_with_zero(|w| w.sofint().set_bit().wakeup().set_bit().extrsm().set_bit());

            let isr_val = usb.isr.read();
            if isr_val.rxsusp().bit_is_set() {
                usb.icr.write_with_zero(|w| w.rxsusp().set_bit());
                return PollResult::Suspend;
            }
            if isr_val.rxrsm().bit_is_set() {
                usb.icr.write_with_zero(|w| w.rxrsm().set_bit());
                return PollResult::Resume;
            }

            if usb.isr.read().endbusres().bit_is_set() {
                usb.icr.write_with_zero(|w| w.endbusres().set_bit());
                return PollResult::Reset;
            }

            let mut ep_out = 0;
            let mut ep_in_complete = 0;
            let mut ep_setup = 0;

            for (ep_idx, mut ep_info) in inner.enumerate_active_ep_mut() {
                let mask = 1 << ep_idx;

                let csr = &usb.csr()[ep_idx];
                let csr_val = csr.read();

                if (ep_info.last_csr & 0xFFFF) != (csr_val.bits() & 0xFFFF) {
                    ep_info.last_csr = csr_val.bits();
                    print_dbg = true;
                }

                if csr_val.rxsetup().bit_is_set() {
                    ep_setup |= mask;
                }
                if csr_val.txcomp().bit_is_set() {
                    if !ep_info.expect_more_writes {
                        csr.modify(|_, w| no_effect(w).txcomp().clear_bit());
                        while csr.read().txcomp().bit_is_set() {}
                    }
                    ep_in_complete |= mask;
                }
                if ep_info.data_ready(&csr_val) {
                    ep_out |= mask;
                }
            }

            if print_dbg {
                dbgprint!(
                    "frm: {:>4} isr: {:016b}",
                    usb.frm_num.read().bits() & 0x7FF,
                    inner.last_isr
                );

                for (idx, ep_info) in inner.enumerate_active_ep() {
                    dbgprint!(
                        " EP{}:{:04x}:{:>3}",
                        idx,
                        ep_info.last_csr & 0xFFFF,
                        ep_info.last_csr >> 16
                    );
                }
                dbgprint!("\n");
            }

            if ep_out == 0 && ep_in_complete == 0 && ep_setup == 0 {
                return PollResult::None;
            }

            dbgprint!(
                "ep_out: {:b} ep_in:{:b} ep_setup: {:b}\n",
                ep_out,
                ep_in_complete,
                ep_setup
            );

            PollResult::Data {
                ep_out,
                ep_in_complete,
                ep_setup,
            }
        })
    }
}
