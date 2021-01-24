use usb_device::class_prelude::*;
use usb_device::Result;

use super::{DFU_FUNCTIONAL, DFU_VERSION, USB_CLASS_DFU, USB_PROTOCOL_DFU, USB_SUB_CLASS_DFU};

enum DFUState {
    Idle,
}

pub struct DFUModeClass<H: Handler> {
    handler: H,
    if_number: InterfaceNumber,
    string_id: Option<StringIndex>,

    state: DFUState,
}
impl<H: Handler> DFUModeClass<H> {
    pub fn new<B: UsbBus>(alloc: &UsbBusAllocator<B>, handler: H) -> Self {
        let string_id = handler.description_string().map(|_| alloc.string());
        Self {
            handler,
            if_number: alloc.interface(),
            string_id,
            state: DFUState::Idle,
        }
    }
}

impl<H: Handler, B: UsbBus> UsbClass<B> for DFUModeClass<H> {
    fn get_configuration_descriptors(&self, writer: &mut DescriptorWriter) -> Result<()> {
        writer.interface(
            self.if_number,
            USB_CLASS_DFU,
            USB_SUB_CLASS_DFU,
            USB_PROTOCOL_DFU,
        )?;

        let attributes = {
            (if self.handler.will_detach() {
                0b0000_1000
            } else {
                0
            }) | (if self.handler.is_manifestation_tolerant() {
                0b0000_0100
            } else {
                0
            }) | (if self.handler.can_upload() {
                0b0000_0010
            } else {
                0
            }) | (if self.handler.can_download() {
                0b0000_0001
            } else {
                0
            })
        };

        let mut descriptor = [attributes, 0, 0, 0, 0, 0, 0];
        descriptor[1..].copy_from_slice(&self.handler.detach_timeout().to_le_bytes());
        descriptor[3..].copy_from_slice(&self.handler.transfer_size().to_le_bytes());
        descriptor[5..].copy_from_slice(&DFU_VERSION.to_le_bytes());
        writer.write(DFU_FUNCTIONAL, &descriptor)?;

        Ok(())
    }

    fn get_string(&self, index: StringIndex, _lang_id: u16) -> Option<&str> {
        if Some(index) != self.string_id {
            None
        } else {
            self.handler.description_string()
        }
    }

    fn reset(&mut self) {}

    fn control_in(&mut self, _xfer: ControlIn<B>) {}

    fn control_out(&mut self, _xfer: ControlOut<B>) {}
}
