use usb_device::{
    bus::{PollResult, UsbBus as UsbBusTrait},
    endpoint::{EndpointAddress, EndpointType},
    Result as UsbResult, UsbDirection, UsbError,
};

use crate::pac::udp::csr::{EPTYPE_A, W as CSRWrite};

use crate::pmc::Clocks;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;

fn usb() -> &'static crate::pac::udp::RegisterBlock {
    unsafe { &*crate::pac::UDP::ptr() }
}

#[inline]
fn no_effect(w: &mut CSRWrite) -> &mut CSRWrite {
    w.rxsetup()
        .set_bit()
        .stallsent()
        .set_bit()
        .rx_data_bk0()
        .set_bit()
        .rx_data_bk1()
        .set_bit()
        .txcomp()
        .set_bit()
}

type EndpointInfo = Option<EPTYPE_A>;

#[derive(Debug)]
struct AllEndpoints([EndpointInfo; 8]);
impl AllEndpoints {
    fn find_free(&self, dir: UsbDirection) -> UsbResult<EndpointAddress> {
        // start with 1 because 0 is reserved for Control
        for idx in 1..8 {
            if self.0[idx].is_none() {
                return Ok(EndpointAddress::from_parts(idx, dir));
            }
        }
        Err(UsbError::EndpointOverflow)
    }

    fn allocate(
        &mut self,
        ep_addr: EndpointAddress,
        ep_type: EndpointType,
        _max_packet_size: u16,
    ) -> UsbResult<()> {
        // usb endpoint
        //
        // EP0  single bank    64   Control/Bulk/Interrupt
        // EP1    dual bank    64   Bulk/Isochonous/Interrupt
        // EP2    dual bank    64   Bulk/Isochonous/Interrupt
        // EP3  single bank    64   Control/Bulk/Interrupt
        // EP4    dual bank   512   Bulk/Isochonous/Interrupt
        // EP5    dual bank   512   Bulk/Isochonous/Interrupt
        // EP6    dual bank    64   Bulk/Isochonous/Interrupt
        // EP7    dual bank    64   Bulk/Isochonous/Interrupt

        // TODO: check max_packet_size
        if ep_type == EndpointType::Isochronous && ep_addr.index() == 3 {
            return Err(UsbError::Unsupported);
        }

        let eptype = match (ep_type, ep_addr.direction()) {
            (EndpointType::Isochronous, UsbDirection::In) => EPTYPE_A::ISO_IN,
            (EndpointType::Isochronous, UsbDirection::Out) => EPTYPE_A::ISO_OUT,
            (EndpointType::Interrupt, UsbDirection::In) => EPTYPE_A::INT_IN,
            (EndpointType::Interrupt, UsbDirection::Out) => EPTYPE_A::INT_OUT,
            (EndpointType::Bulk, UsbDirection::In) => EPTYPE_A::BULK_IN,
            (EndpointType::Bulk, UsbDirection::Out) => EPTYPE_A::BULK_OUT,
            (EndpointType::Control, _) => EPTYPE_A::CTRL,
        };

        self.0[ep_addr.index()] = Some(eptype);

        Ok(())
    }

    fn configure(&self) {
        for (i, info) in self.0.iter().enumerate() {
            if let Some(eptype) = *info {
                let csr = &usb().csr()[i];
                csr.modify(|_, w| {
                    // set NoEffect bits
                    no_effect(w)
                        // enable endpoint
                        .epeds()
                        .set_bit()
                        // set endpoint type & direction
                        .eptype()
                        .variant(eptype)
                });
                while csr.read().eptype() != eptype {}
            }
        }
    }
}

struct Inner {
    clear_txcomp: bool,
    /// (isr, csr)
    prev: (u32, u32),
}

pub struct UsbBus {
    endpoints: AllEndpoints,
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
            endpoints: AllEndpoints([EndpointInfo::default(); 8]),
            inner: Mutex::new(RefCell::new(Inner {
                clear_txcomp: true,
                prev: (0, 0),
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

        let ep_addr = match ep_addr {
            None => self.endpoints.find_free(ep_dir)?,
            Some(addr) => EndpointAddress::from(addr),
        };

        self.endpoints.allocate(ep_addr, ep_type, max_packet_size)?;

        dbgprint!(
            "EP{}-{:?}: {:?}\n",
            ep_addr.index(),
            ep_addr.direction(),
            self.endpoints.0[ep_addr.index()]
        );

        Ok(ep_addr)
    }

    fn enable(&mut self) {
        dbgprint!("UsbBus::enable\n");
        usb().txvc.modify(|_, w| w.puon().set_bit());
    }
    fn reset(&self) {
        dbgprint!("UsbBus::reset\n");
        self.endpoints.configure();
        usb().ier.write_with_zero(|w| w.ep0int().set_bit());
        usb().txvc.modify(|_, w| w.txvdis().clear_bit());
    }

    fn set_device_address(&self, addr: u8) {
        dbgprint!("UsbBus::set_device_address({})\n", addr);
        usb()
            .faddr
            .modify(|_, w| unsafe { w.fadd().bits(addr).fen().set_bit() });
        usb().glb_stat.write_with_zero(|w| w.fadden().set_bit());
    }

    fn write(&self, ep_addr: EndpointAddress, buf: &[u8]) -> UsbResult<usize> {
        dbgprint!(
            "UsbBus::write({}-{:?}, {:x?}) -> ",
            ep_addr.index(),
            ep_addr.direction(),
            buf
        );

        let csr = &usb().csr()[ep_addr.index()];

        let csr_val = csr.read();
        if csr_val.epeds().bit_is_clear() || !ep_addr.is_in() {
            dbgprint!("{:?}\n", UsbError::InvalidEndpoint);
            return Err(UsbError::InvalidEndpoint);
        }

        if csr_val.txpktrdy().bit_is_set() {
            dbgprint!("{:?}\n", UsbError::WouldBlock);
            return Err(UsbError::WouldBlock);
        }

        // TODO: check that buf is not too long for this EP

        // If ctrl ep: switch direction
        if ep_addr.index() == 0 {
            dbgprint!("(csr: {:b}) ", csr_val.bits());
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

        if ep_addr.index() == 0 {
            cortex_m::interrupt::free(|cs| {
                let mut inner = self.inner.borrow(cs).borrow_mut();
                // TODO: replace buf.len() with MAX_CTRL_EP_LEN
                inner.clear_txcomp = buf.len() != 8;
            });
        }

        dbgprint!("{}\n", buf.len());
        Ok(buf.len())
    }
    fn read(&self, ep_addr: EndpointAddress, buf: &mut [u8]) -> UsbResult<usize> {
        dbgprint!(
            "UsbBus::read({}-{:?}) -> ",
            ep_addr.index(),
            ep_addr.direction()
        );
        let csr = &usb().csr()[ep_addr.index()];

        let csr_val = csr.read();
        if csr_val.epeds().bit_is_clear() || !ep_addr.is_out() {
            dbgprint!("{:?}\n", UsbError::InvalidEndpoint);
            return Err(UsbError::InvalidEndpoint);
        }
        if csr_val.rx_data_bk0().bit_is_clear() && csr_val.rxsetup().bit_is_clear() {
            dbgprint!("{:?}\n", UsbError::WouldBlock);
            return Err(UsbError::WouldBlock);
        }

        let bytecnt = csr_val.rxbytecnt().bits().into();
        if bytecnt > buf.len() {
            dbgprint!("{:?}\n", UsbError::BufferOverflow);
            return Err(UsbError::BufferOverflow);
        }

        if ep_addr.index() == 0 {
            dbgprint!("(csr: {:b}) ", csr_val.bits());
        }

        let fdr = &usb().fdr[ep_addr.index()];
        for (_, pbyte) in (0..bytecnt).zip(buf.iter_mut()) {
            *pbyte = fdr.read().fifo_data().bits();
        }

        if csr_val.rxsetup().bit_is_set() {
            csr.modify(|_, w| no_effect(w).rxsetup().clear_bit());
            while csr.read().rxsetup().bit_is_set() {}
        } else if csr_val.rx_data_bk0().bit_is_set() {
            csr.modify(|_, w| no_effect(w).rx_data_bk0().clear_bit());
            while csr.read().rx_data_bk0().bit_is_set() {}
        }

        dbgprint!("{:x?}\n", &buf[..bytecnt]);
        Ok(bytecnt)
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
            while csr.read().forcestall().bit_is_clear() {}
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
        dbgprint!("UsbBus::suspend\n");
        // TODO: We need to delegate that to the application as it may not allow to mess with
        // clocks
        //usb().txvc.modify(|_, w| w.txvdis().set_bit());
    }
    fn resume(&self) {
        dbgprint!("UsbBus::resume\n");
        // TODO: We need to delegate that to the application as it may not allow to mess with
        // clocks
        //usb().txvc.modify(|_, w| w.txvdis().clear_bit());
    }

    fn poll(&self) -> PollResult {
        let usb = usb();

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

        for ep_idx in 0..8 {
            let mask = 1 << ep_idx;

            let csr = &usb.csr()[ep_idx];
            let csr_val = csr.read();
            if csr_val.epeds().bit_is_clear() {
                continue;
            }

            if ep_idx == 0
                && cortex_m::interrupt::free(|cs| {
                    let mut inner = self.inner.borrow(cs).borrow_mut();
                    let new = (isr_val.bits(), csr_val.bits());
                    if inner.prev != new {
                        inner.prev = new;
                        true
                    } else {
                        false
                    }
                })
            {
                dbgprint!(
                    "frm: {} isr: {:b} csr{}: {:b}\n",
                    usb.frm_num.read().bits() & 0x7FF,
                    isr_val.bits(),
                    ep_idx,
                    csr_val.bits()
                );
            }
            if csr_val.rxsetup().bit_is_set() {
                ep_setup |= mask;
            }
            if csr_val.txcomp().bit_is_set() {
                cortex_m::interrupt::free(|cs| {
                    if ep_idx != 0 || self.inner.borrow(cs).borrow().clear_txcomp {
                        csr.modify(|_, w| no_effect(w).txcomp().clear_bit());
                        while csr.read().txcomp().bit_is_set() {}
                    }
                });
                ep_in_complete |= mask;
            }
            if csr_val.rx_data_bk0().bit_is_set() {
                ep_out |= mask;
            }
        }

        if ep_out == 0 && ep_in_complete == 0 && ep_setup == 0 {
            return PollResult::None;
        }

        dbgprint!(
            "ep_setup: {:b} ep_in_complete: {:b} ep_out: {:b}\n",
            ep_setup,
            ep_in_complete,
            ep_out,
        );

        PollResult::Data {
            ep_out,
            ep_in_complete,
            ep_setup,
        }
    }
}
