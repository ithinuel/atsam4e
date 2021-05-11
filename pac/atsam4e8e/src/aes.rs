#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - Mode Register"]
    pub mr: crate::Reg<mr::MR_SPEC>,
    _reserved2: [u8; 8usize],
    #[doc = "0x10 - Interrupt Enable Register"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x14 - Interrupt Disable Register"]
    pub idr: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x18 - Interrupt Mask Register"]
    pub imr: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0x1c - Interrupt Status Register"]
    pub isr: crate::Reg<isr::ISR_SPEC>,
    #[doc = "0x20 - Key Word Register"]
    pub keywr: [crate::Reg<keywr::KEYWR_SPEC>; 8],
    #[doc = "0x40 - Input Data Register"]
    pub idatar: [crate::Reg<idatar::IDATAR_SPEC>; 4],
    #[doc = "0x50 - Output Data Register"]
    pub odatar: [crate::Reg<odatar::ODATAR_SPEC>; 4],
    #[doc = "0x60 - Initialization Vector Register"]
    pub ivr: [crate::Reg<ivr::IVR_SPEC>; 4],
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "MR register accessor: an alias for `Reg<MR_SPEC>`"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "Mode Register"]
pub mod mr;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "ISR register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "KEYWR register accessor: an alias for `Reg<KEYWR_SPEC>`"]
pub type KEYWR = crate::Reg<keywr::KEYWR_SPEC>;
#[doc = "Key Word Register"]
pub mod keywr;
#[doc = "IDATAR register accessor: an alias for `Reg<IDATAR_SPEC>`"]
pub type IDATAR = crate::Reg<idatar::IDATAR_SPEC>;
#[doc = "Input Data Register"]
pub mod idatar;
#[doc = "ODATAR register accessor: an alias for `Reg<ODATAR_SPEC>`"]
pub type ODATAR = crate::Reg<odatar::ODATAR_SPEC>;
#[doc = "Output Data Register"]
pub mod odatar;
#[doc = "IVR register accessor: an alias for `Reg<IVR_SPEC>`"]
pub type IVR = crate::Reg<ivr::IVR_SPEC>;
#[doc = "Initialization Vector Register"]
pub mod ivr;
