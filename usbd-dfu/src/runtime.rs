use core::num::Wrapping;
use usb_device::class_prelude::*;
use usb_device::Result;

use super::{
    Request, State, Status, DFU_FUNCTIONAL, DFU_VERSION, USB_CLASS_DFU, USB_PROTOCOL_DFU,
    USB_SUB_CLASS_DFU,
};

pub trait DeviceFirmwareUpgrade {
    /// If true, the device generates a detach-attach sequence on its own upon receipt of a detach
    /// request. Otherwise the device waits for a USB reset until a time out expires.
    const WILL_DETACH: bool;

    /// If true, the device is able to handle other DFU task after a download has completed.
    /// Otherwise the device expects a USB reset.
    const IS_MANIFESTATION_TOLERANT: bool;

    /// True if the device can send its current firmware to the host.
    const CAN_UPLOAD: bool;

    /// True if the device can receive a firmware from the host.
    const CAN_DOWNLOAD: bool;

    /// Time, in milliseconds, that the device will wait after receipt of the detach request. If
    /// this time elapses without a USB reset, then the device will terminate the reconfiguration
    /// phase and revert back to normal operation. This represents the maximum time that the device
    /// can wait (depending on its timers, etc.). The host may specify a shorter timeout in the
    /// detach request.
    const DETACH_TIMEOUT: u16;

    /// Maximum number of bytes the device can accept between per control-write transaction.
    const TRANSFER_SIZE: u16;

    /// Called by the USB stack when a reset is triggered by the host.
    fn on_reset(&mut self);

    /// Called by the USB stack when a detach request is received by the device. If `will_detach`
    /// is false, the device must initiate the detach-attach sequence now.
    fn on_detach_request(&mut self, timeout_ms: u16);
}

#[allow(non_snake_case)]
pub struct DFURuntimeClass<D: DeviceFirmwareUpgrade> {
    handler: D,
    interface_number: InterfaceNumber,
    now: Wrapping<u32>,
    state: State,
}

impl<H: DeviceFirmwareUpgrade> DFURuntimeClass<H> {
    pub fn new<B: UsbBus>(alloc: &UsbBusAllocator<B>, handler: H, now: u32) -> Self {
        Self {
            handler,
            interface_number: alloc.interface(),
            now: Wrapping(now),
            state: State::AppIdle,
        }
    }

    /// updates the state of the driver. Takes the number of nano-second since last update.
    pub fn poll(&mut self, elapsed_ms: u32) {
        self.now += Wrapping(elapsed_ms);
        if let State::AppDetach(when) = self.state {
            if self.now.0 > when {
                self.state = State::AppIdle;
            }
        }
    }
}

impl<H: DeviceFirmwareUpgrade, B: UsbBus> UsbClass<B> for DFURuntimeClass<H> {
    fn get_configuration_descriptors(&self, writer: &mut DescriptorWriter) -> Result<()> {
        writer.interface(
            self.interface_number,
            USB_CLASS_DFU,
            USB_SUB_CLASS_DFU,
            USB_PROTOCOL_DFU,
        )?;

        let attributes = {
            (if H::WILL_DETACH { 0b0000_1000 } else { 0 })
                | (if H::IS_MANIFESTATION_TOLERANT {
                    0b0000_0100
                } else {
                    0
                })
                | (if H::CAN_UPLOAD { 0b0000_0010 } else { 0 })
                | (if H::CAN_DOWNLOAD { 0b0000_0001 } else { 0 })
        };

        let mut descriptor = [attributes, 0, 0, 0, 0, 0, 0];
        descriptor[1..3].copy_from_slice(&H::DETACH_TIMEOUT.to_le_bytes());
        descriptor[3..5].copy_from_slice(&H::TRANSFER_SIZE.to_le_bytes());
        descriptor[5..7].copy_from_slice(&DFU_VERSION.to_le_bytes());
        writer.write(DFU_FUNCTIONAL, &descriptor)?;

        Ok(())
    }

    fn reset(&mut self) {
        if let State::AppDetach(_) = self.state {
            self.handler.on_reset();
        }
    }

    fn control_in(&mut self, xfer: ControlIn<B>) {
        let req = xfer.request();
        if !(req.request_type == control::RequestType::Class
            && req.recipient == control::Recipient::Interface
            && req.index == u8::from(self.interface_number) as u16)
        {
            return;
        }

        let _ = match req.request {
            Request::DFU_GETSTATE => xfer.accept_with(&[u8::from(self.state)]),
            Request::DFU_GETSTATUS => {
                xfer.accept_with(&[Status::Ok as u8, 1, self.state.into(), 0])
            }
            _ => xfer.reject(),
        };
    }

    fn control_out(&mut self, xfer: ControlOut<B>) {
        let req = xfer.request();
        if !(req.request_type == control::RequestType::Class
            && req.recipient == control::Recipient::Interface
            && req.index == u8::from(self.interface_number) as u16)
        {
            return;
        }

        if req.request == Request::DFU_DETACH {
            let timeout_ms = xfer.request().value;
            let end_timestamp = self.now + Wrapping(timeout_ms.into());

            self.state = State::AppDetach(end_timestamp.0);
            // propagate the event to the handler
            self.handler.on_detach_request(timeout_ms);

            let _ = xfer.accept();
        } else {
            let _ = xfer.reject();
        };
    }
}
