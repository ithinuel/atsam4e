#![no_std]

use atsam4e_hal::pac::{efc, EFC};
use usbd_dfu::{Error, Result};

macro_rules! impl_capabilities {
    ($name:ty) => {
        impl usbd_dfu::Capabilities for $name {
            const CAN_UPLOAD: bool = true;
            const CAN_DOWNLOAD: bool = true;
            const IS_MANIFESTATION_TOLERANT: bool = true;
            const WILL_DETACH: bool = false;
            const DETACH_TIMEOUT: u16 = 5000;
            const TRANSFER_SIZE: u16 = 4096;
        }
    };
}

pub struct DFURuntimeImpl;
impl_capabilities!(DFURuntimeImpl);
impl usbd_dfu::runtime::DeviceFirmwareUpgrade for DFURuntimeImpl {
    fn on_reset(&mut self) {
        // trigger mcu reset to enter bootloader!
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

    fn on_detach_request(&mut self, _timeout_ms: u16) {}
}

pub struct DFUModeImpl {
    /// Embedded Flash Controller
    efc: EFC,
}
impl DFUModeImpl {
    pub fn new(efc: EFC) -> Self {
        Self { efc }
    }
}
impl_capabilities!(DFUModeImpl);
impl usbd_dfu::mode::DeviceFirmwareUpgrade for DFUModeImpl {
    const POLL_TIMEOUT: u32 = 1000;

    fn is_firmware_valid(&mut self) -> bool {
        //TODO: actually validate the flash !
        true
    }

    fn is_transfer_complete(&mut self) -> bool {
        true
    }

    fn is_manifestation_in_progress(&mut self) -> bool {
        false
    }

    fn upload(&mut self, _block_number: u16, _buf: &mut [u8]) -> Result<usize> {
        Ok(25)
    }
    fn download(&mut self, _block_number: u16, _buf: &[u8]) -> core::result::Result<(), Error> {
        let _fc = &mut self.efc;
        // write data to the latch buffer with 32bit operations aligned to 32bits addresses.
        //
        //
        //

        use efc::fcr::*;
        let cmd: u32 = u8::from(FCMD_AW::EWP).into();
        let page_number = 0;
        let key: u32 = u8::from(FKEY_AW::PASSWD).into();

        let iap_arg = (key << 24) | (page_number << 8) | cmd;
        let iap_fsr = unsafe {
            let iap: fn(u32) -> u32 = *(0x0080_0008 as *const usize as *const _);
            cortex_m::interrupt::free(|_| iap(iap_arg))
        };
        assert_eq!(
            iap_fsr & 1,
            1,
            "IAP method only returns once the EFC is ready."
        );
        if iap_fsr & 2 == 2 {
            Err(Error::Programming)
        } else if iap_fsr & 8 == 8 {
            Err(Error::Verify)
        } else if iap_fsr != 0 {
            Err(Error::Unknown)
        } else {
            Ok(())
        }
    }
}
