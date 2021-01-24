#![no_std]

pub struct DFURuntimeImpl;
impl usbd_dfu::runtime::DeviceFirmwareUpgrade for DFURuntimeImpl {
    const CAN_UPLOAD: bool = true;
    const CAN_DOWNLOAD: bool = true;
    const IS_MANIFESTATION_TOLERANT: bool = true;
    const WILL_DETACH: bool = false;
    const DETACH_TIMEOUT: u16 = 5000;
    const TRANSFER_SIZE: u16 = 4096;

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
