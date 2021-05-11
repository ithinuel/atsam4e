#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - General Purpose Backup Register"]
    pub gpbr: [crate::Reg<gpbr::GPBR_SPEC>; 20],
}
#[doc = "GPBR register accessor: an alias for `Reg<GPBR_SPEC>`"]
pub type GPBR = crate::Reg<gpbr::GPBR_SPEC>;
#[doc = "General Purpose Backup Register"]
pub mod gpbr;
