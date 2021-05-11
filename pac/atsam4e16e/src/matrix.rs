#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Master Configuration Register"]
    pub mcfg: [crate::Reg<mcfg::MCFG_SPEC>; 16],
    #[doc = "0x40 - Slave Configuration Register"]
    pub scfg: [crate::Reg<scfg::SCFG_SPEC>; 16],
    #[doc = "0x80 - Priority Register A for Slave 0"]
    pub pras0: crate::Reg<pras0::PRAS0_SPEC>,
    #[doc = "0x84 - Priority Register B for Slave 0"]
    pub prbs0: crate::Reg<prbs0::PRBS0_SPEC>,
    #[doc = "0x88 - Priority Register A for Slave 1"]
    pub pras1: crate::Reg<pras1::PRAS1_SPEC>,
    #[doc = "0x8c - Priority Register B for Slave 1"]
    pub prbs1: crate::Reg<prbs1::PRBS1_SPEC>,
    #[doc = "0x90 - Priority Register A for Slave 2"]
    pub pras2: crate::Reg<pras2::PRAS2_SPEC>,
    #[doc = "0x94 - Priority Register B for Slave 2"]
    pub prbs2: crate::Reg<prbs2::PRBS2_SPEC>,
    #[doc = "0x98 - Priority Register A for Slave 3"]
    pub pras3: crate::Reg<pras3::PRAS3_SPEC>,
    #[doc = "0x9c - Priority Register B for Slave 3"]
    pub prbs3: crate::Reg<prbs3::PRBS3_SPEC>,
    #[doc = "0xa0 - Priority Register A for Slave 4"]
    pub pras4: crate::Reg<pras4::PRAS4_SPEC>,
    #[doc = "0xa4 - Priority Register B for Slave 4"]
    pub prbs4: crate::Reg<prbs4::PRBS4_SPEC>,
    #[doc = "0xa8 - Priority Register A for Slave 5"]
    pub pras5: crate::Reg<pras5::PRAS5_SPEC>,
    #[doc = "0xac - Priority Register B for Slave 5"]
    pub prbs5: crate::Reg<prbs5::PRBS5_SPEC>,
    #[doc = "0xb0 - Priority Register A for Slave 6"]
    pub pras6: crate::Reg<pras6::PRAS6_SPEC>,
    #[doc = "0xb4 - Priority Register B for Slave 6"]
    pub prbs6: crate::Reg<prbs6::PRBS6_SPEC>,
    #[doc = "0xb8 - Priority Register A for Slave 7"]
    pub pras7: crate::Reg<pras7::PRAS7_SPEC>,
    #[doc = "0xbc - Priority Register B for Slave 7"]
    pub prbs7: crate::Reg<prbs7::PRBS7_SPEC>,
    #[doc = "0xc0 - Priority Register A for Slave 8"]
    pub pras8: crate::Reg<pras8::PRAS8_SPEC>,
    #[doc = "0xc4 - Priority Register B for Slave 8"]
    pub prbs8: crate::Reg<prbs8::PRBS8_SPEC>,
    #[doc = "0xc8 - Priority Register A for Slave 9"]
    pub pras9: crate::Reg<pras9::PRAS9_SPEC>,
    #[doc = "0xcc - Priority Register B for Slave 9"]
    pub prbs9: crate::Reg<prbs9::PRBS9_SPEC>,
    #[doc = "0xd0 - Priority Register A for Slave 10"]
    pub pras10: crate::Reg<pras10::PRAS10_SPEC>,
    #[doc = "0xd4 - Priority Register B for Slave 10"]
    pub prbs10: crate::Reg<prbs10::PRBS10_SPEC>,
    #[doc = "0xd8 - Priority Register A for Slave 11"]
    pub pras11: crate::Reg<pras11::PRAS11_SPEC>,
    #[doc = "0xdc - Priority Register B for Slave 11"]
    pub prbs11: crate::Reg<prbs11::PRBS11_SPEC>,
    #[doc = "0xe0 - Priority Register A for Slave 12"]
    pub pras12: crate::Reg<pras12::PRAS12_SPEC>,
    #[doc = "0xe4 - Priority Register B for Slave 12"]
    pub prbs12: crate::Reg<prbs12::PRBS12_SPEC>,
    #[doc = "0xe8 - Priority Register A for Slave 13"]
    pub pras13: crate::Reg<pras13::PRAS13_SPEC>,
    #[doc = "0xec - Priority Register B for Slave 13"]
    pub prbs13: crate::Reg<prbs13::PRBS13_SPEC>,
    #[doc = "0xf0 - Priority Register A for Slave 14"]
    pub pras14: crate::Reg<pras14::PRAS14_SPEC>,
    #[doc = "0xf4 - Priority Register B for Slave 14"]
    pub prbs14: crate::Reg<prbs14::PRBS14_SPEC>,
    #[doc = "0xf8 - Priority Register A for Slave 15"]
    pub pras15: crate::Reg<pras15::PRAS15_SPEC>,
    #[doc = "0xfc - Priority Register B for Slave 15"]
    pub prbs15: crate::Reg<prbs15::PRBS15_SPEC>,
    #[doc = "0x100 - Master Remap Control Register"]
    pub mrcr: crate::Reg<mrcr::MRCR_SPEC>,
    _reserved35: [u8; 12usize],
    #[doc = "0x110 - Special Function Register"]
    pub sfr: [crate::Reg<sfr::SFR_SPEC>; 16],
    _reserved36: [u8; 148usize],
    #[doc = "0x1e4 - Write Protect Mode Register"]
    pub wpmr: crate::Reg<wpmr::WPMR_SPEC>,
    #[doc = "0x1e8 - Write Protect Status Register"]
    pub wpsr: crate::Reg<wpsr::WPSR_SPEC>,
}
#[doc = "MCFG register accessor: an alias for `Reg<MCFG_SPEC>`"]
pub type MCFG = crate::Reg<mcfg::MCFG_SPEC>;
#[doc = "Master Configuration Register"]
pub mod mcfg;
#[doc = "SCFG register accessor: an alias for `Reg<SCFG_SPEC>`"]
pub type SCFG = crate::Reg<scfg::SCFG_SPEC>;
#[doc = "Slave Configuration Register"]
pub mod scfg;
#[doc = "PRAS0 register accessor: an alias for `Reg<PRAS0_SPEC>`"]
pub type PRAS0 = crate::Reg<pras0::PRAS0_SPEC>;
#[doc = "Priority Register A for Slave 0"]
pub mod pras0;
#[doc = "PRBS0 register accessor: an alias for `Reg<PRBS0_SPEC>`"]
pub type PRBS0 = crate::Reg<prbs0::PRBS0_SPEC>;
#[doc = "Priority Register B for Slave 0"]
pub mod prbs0;
#[doc = "PRAS1 register accessor: an alias for `Reg<PRAS1_SPEC>`"]
pub type PRAS1 = crate::Reg<pras1::PRAS1_SPEC>;
#[doc = "Priority Register A for Slave 1"]
pub mod pras1;
#[doc = "PRBS1 register accessor: an alias for `Reg<PRBS1_SPEC>`"]
pub type PRBS1 = crate::Reg<prbs1::PRBS1_SPEC>;
#[doc = "Priority Register B for Slave 1"]
pub mod prbs1;
#[doc = "PRAS2 register accessor: an alias for `Reg<PRAS2_SPEC>`"]
pub type PRAS2 = crate::Reg<pras2::PRAS2_SPEC>;
#[doc = "Priority Register A for Slave 2"]
pub mod pras2;
#[doc = "PRBS2 register accessor: an alias for `Reg<PRBS2_SPEC>`"]
pub type PRBS2 = crate::Reg<prbs2::PRBS2_SPEC>;
#[doc = "Priority Register B for Slave 2"]
pub mod prbs2;
#[doc = "PRAS3 register accessor: an alias for `Reg<PRAS3_SPEC>`"]
pub type PRAS3 = crate::Reg<pras3::PRAS3_SPEC>;
#[doc = "Priority Register A for Slave 3"]
pub mod pras3;
#[doc = "PRBS3 register accessor: an alias for `Reg<PRBS3_SPEC>`"]
pub type PRBS3 = crate::Reg<prbs3::PRBS3_SPEC>;
#[doc = "Priority Register B for Slave 3"]
pub mod prbs3;
#[doc = "PRAS4 register accessor: an alias for `Reg<PRAS4_SPEC>`"]
pub type PRAS4 = crate::Reg<pras4::PRAS4_SPEC>;
#[doc = "Priority Register A for Slave 4"]
pub mod pras4;
#[doc = "PRBS4 register accessor: an alias for `Reg<PRBS4_SPEC>`"]
pub type PRBS4 = crate::Reg<prbs4::PRBS4_SPEC>;
#[doc = "Priority Register B for Slave 4"]
pub mod prbs4;
#[doc = "PRAS5 register accessor: an alias for `Reg<PRAS5_SPEC>`"]
pub type PRAS5 = crate::Reg<pras5::PRAS5_SPEC>;
#[doc = "Priority Register A for Slave 5"]
pub mod pras5;
#[doc = "PRBS5 register accessor: an alias for `Reg<PRBS5_SPEC>`"]
pub type PRBS5 = crate::Reg<prbs5::PRBS5_SPEC>;
#[doc = "Priority Register B for Slave 5"]
pub mod prbs5;
#[doc = "PRAS6 register accessor: an alias for `Reg<PRAS6_SPEC>`"]
pub type PRAS6 = crate::Reg<pras6::PRAS6_SPEC>;
#[doc = "Priority Register A for Slave 6"]
pub mod pras6;
#[doc = "PRBS6 register accessor: an alias for `Reg<PRBS6_SPEC>`"]
pub type PRBS6 = crate::Reg<prbs6::PRBS6_SPEC>;
#[doc = "Priority Register B for Slave 6"]
pub mod prbs6;
#[doc = "PRAS7 register accessor: an alias for `Reg<PRAS7_SPEC>`"]
pub type PRAS7 = crate::Reg<pras7::PRAS7_SPEC>;
#[doc = "Priority Register A for Slave 7"]
pub mod pras7;
#[doc = "PRBS7 register accessor: an alias for `Reg<PRBS7_SPEC>`"]
pub type PRBS7 = crate::Reg<prbs7::PRBS7_SPEC>;
#[doc = "Priority Register B for Slave 7"]
pub mod prbs7;
#[doc = "PRAS8 register accessor: an alias for `Reg<PRAS8_SPEC>`"]
pub type PRAS8 = crate::Reg<pras8::PRAS8_SPEC>;
#[doc = "Priority Register A for Slave 8"]
pub mod pras8;
#[doc = "PRBS8 register accessor: an alias for `Reg<PRBS8_SPEC>`"]
pub type PRBS8 = crate::Reg<prbs8::PRBS8_SPEC>;
#[doc = "Priority Register B for Slave 8"]
pub mod prbs8;
#[doc = "PRAS9 register accessor: an alias for `Reg<PRAS9_SPEC>`"]
pub type PRAS9 = crate::Reg<pras9::PRAS9_SPEC>;
#[doc = "Priority Register A for Slave 9"]
pub mod pras9;
#[doc = "PRBS9 register accessor: an alias for `Reg<PRBS9_SPEC>`"]
pub type PRBS9 = crate::Reg<prbs9::PRBS9_SPEC>;
#[doc = "Priority Register B for Slave 9"]
pub mod prbs9;
#[doc = "PRAS10 register accessor: an alias for `Reg<PRAS10_SPEC>`"]
pub type PRAS10 = crate::Reg<pras10::PRAS10_SPEC>;
#[doc = "Priority Register A for Slave 10"]
pub mod pras10;
#[doc = "PRBS10 register accessor: an alias for `Reg<PRBS10_SPEC>`"]
pub type PRBS10 = crate::Reg<prbs10::PRBS10_SPEC>;
#[doc = "Priority Register B for Slave 10"]
pub mod prbs10;
#[doc = "PRAS11 register accessor: an alias for `Reg<PRAS11_SPEC>`"]
pub type PRAS11 = crate::Reg<pras11::PRAS11_SPEC>;
#[doc = "Priority Register A for Slave 11"]
pub mod pras11;
#[doc = "PRBS11 register accessor: an alias for `Reg<PRBS11_SPEC>`"]
pub type PRBS11 = crate::Reg<prbs11::PRBS11_SPEC>;
#[doc = "Priority Register B for Slave 11"]
pub mod prbs11;
#[doc = "PRAS12 register accessor: an alias for `Reg<PRAS12_SPEC>`"]
pub type PRAS12 = crate::Reg<pras12::PRAS12_SPEC>;
#[doc = "Priority Register A for Slave 12"]
pub mod pras12;
#[doc = "PRBS12 register accessor: an alias for `Reg<PRBS12_SPEC>`"]
pub type PRBS12 = crate::Reg<prbs12::PRBS12_SPEC>;
#[doc = "Priority Register B for Slave 12"]
pub mod prbs12;
#[doc = "PRAS13 register accessor: an alias for `Reg<PRAS13_SPEC>`"]
pub type PRAS13 = crate::Reg<pras13::PRAS13_SPEC>;
#[doc = "Priority Register A for Slave 13"]
pub mod pras13;
#[doc = "PRBS13 register accessor: an alias for `Reg<PRBS13_SPEC>`"]
pub type PRBS13 = crate::Reg<prbs13::PRBS13_SPEC>;
#[doc = "Priority Register B for Slave 13"]
pub mod prbs13;
#[doc = "PRAS14 register accessor: an alias for `Reg<PRAS14_SPEC>`"]
pub type PRAS14 = crate::Reg<pras14::PRAS14_SPEC>;
#[doc = "Priority Register A for Slave 14"]
pub mod pras14;
#[doc = "PRBS14 register accessor: an alias for `Reg<PRBS14_SPEC>`"]
pub type PRBS14 = crate::Reg<prbs14::PRBS14_SPEC>;
#[doc = "Priority Register B for Slave 14"]
pub mod prbs14;
#[doc = "PRAS15 register accessor: an alias for `Reg<PRAS15_SPEC>`"]
pub type PRAS15 = crate::Reg<pras15::PRAS15_SPEC>;
#[doc = "Priority Register A for Slave 15"]
pub mod pras15;
#[doc = "PRBS15 register accessor: an alias for `Reg<PRBS15_SPEC>`"]
pub type PRBS15 = crate::Reg<prbs15::PRBS15_SPEC>;
#[doc = "Priority Register B for Slave 15"]
pub mod prbs15;
#[doc = "MRCR register accessor: an alias for `Reg<MRCR_SPEC>`"]
pub type MRCR = crate::Reg<mrcr::MRCR_SPEC>;
#[doc = "Master Remap Control Register"]
pub mod mrcr;
#[doc = "SFR register accessor: an alias for `Reg<SFR_SPEC>`"]
pub type SFR = crate::Reg<sfr::SFR_SPEC>;
#[doc = "Special Function Register"]
pub mod sfr;
#[doc = "WPMR register accessor: an alias for `Reg<WPMR_SPEC>`"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "Write Protect Mode Register"]
pub mod wpmr;
#[doc = "WPSR register accessor: an alias for `Reg<WPSR_SPEC>`"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "Write Protect Status Register"]
pub mod wpsr;
