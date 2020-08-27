//! Clock generation:
//! - slow clocks:
//!     - internal 32KHz
//!     - external up to 32768Hz
//! - main clock:
//!     - internal 4/8/12MHz (depends on the target ?)
//!     - external 3 to 20MHz
//!
//! Master Clock:
//! - slow clock
//! - main clock
//! - plla (derived from main clock)
//!
//! Programmable Clock x
//! - slow clock
//! - main clock
//! - plla (derived from main clock)
//! - Master Clock
//!
//! USB Clock:
//! - PLLA Clock

use crate::pac::pmc::*;
use crate::pac::PMC;
use crate::time::{Hertz, U32Ext};

/// 46.5 PLLA Characteristics
const PLL_MAX_FREQUENCY: Hertz = Hertz(240_000_000);

pub trait PmcExt {
    fn constrain(self) -> Pmc;
}

impl PmcExt for PMC {
    fn constrain(self) -> Pmc {
        Pmc {
            main_ck: MainClock::Internal(ckgr_mor::MOSCRCF_A::_4_MHZ),
            master_clock: 4.mhz().into(),
            programmable_clock_1: None,
            programmable_clock_2: None,
            programmable_clock_3: None,
            use_usb: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MainClock {
    Internal(ckgr_mor::MOSCRCF_A),
    External(Hertz),
}

#[derive(Debug, Clone, Copy)]
struct PllConfig {
    div: u8,
    mul: u16,
    master_pres: Option<pmc_mckr::PRES_A>,
    usb_pres: Option<u8>,
    pck1_pres: Option<pmc_pck::PRES_A>,
    pck2_pres: Option<pmc_pck::PRES_A>,
    pck3_pres: Option<pmc_pck::PRES_A>,
}

pub struct Pmc {
    main_ck: MainClock,
    master_clock: Hertz,
    programmable_clock_1: Option<Hertz>,
    programmable_clock_2: Option<Hertz>,
    programmable_clock_3: Option<Hertz>,
    use_usb: bool,
}

#[inline]
fn setup_flash(clk: Hertz) {
    unsafe {
        let eefc = &*crate::pac::EFC::ptr();
        // TODO: this should probably be checking the core voltage
        eefc.fmr.modify(|_, w| {
            w.fws().bits(match clk.0 {
                0..=21_000_000 => 0,
                21_000_001..=43_000_000 => 1,
                43_000_001..=64_000_000 => 2,
                64_000_001..=86_000_000 => 3,
                86_000_001..=107_000_000 => 4,
                107_000_001..=129_000_000 => 5,
                _ => panic!("master clock is too high"),
            });
            w.cloe().set_bit();
            w
        })
    }
}

impl Pmc {
    #[inline]
    pub fn main_clock(mut self, clock: MainClock) -> Self {
        self.main_ck = clock;
        self
    }
    #[inline]
    pub fn master_clock<F: Into<Hertz>>(mut self, freq: F) -> Self {
        self.master_clock = freq.into();
        self
    }
    #[inline]
    pub fn use_usb(mut self) -> Self {
        self.use_usb = true;
        self
    }

    #[inline]
    fn pll_configuration(&self, main_freq: Hertz) -> Option<PllConfig> {
        use crate::math::*;

        // compute required pll freq
        let mut pll_freq = None;
        if self.master_clock > main_freq {
            pll_freq = Some(self.master_clock.0);
        }
        if self.use_usb {
            pll_freq = Some(pll_freq.map_or(48_000_000, |v| lcm(v, 48_000_000)));
        }
        if let Some(Hertz(pck)) = self.programmable_clock_1 {
            pll_freq = Some(pll_freq.map_or(pck, |v| lcm(v, pck)));
        }
        if let Some(Hertz(pck)) = self.programmable_clock_2 {
            pll_freq = Some(pll_freq.map_or(pck, |v| lcm(v, pck)));
        }
        if let Some(Hertz(pck)) = self.programmable_clock_3 {
            pll_freq = Some(pll_freq.map_or(pck, |v| lcm(v, pck)));
        }

        let pll_freq = pll_freq?;

        // verify we can achieve the expected frequuencies with the sub prescaler
        assert!(pll_freq <= PLL_MAX_FREQUENCY.0);

        let master_pres = if self.master_clock > main_freq {
            Some(match pll_freq / self.master_clock.0 {
                1 => pmc_mckr::PRES_A::CLK_1,
                2 => pmc_mckr::PRES_A::CLK_2,
                3 => pmc_mckr::PRES_A::CLK_3,
                4 => pmc_mckr::PRES_A::CLK_4,
                8 => pmc_mckr::PRES_A::CLK_8,
                16 => pmc_mckr::PRES_A::CLK_16,
                32 => pmc_mckr::PRES_A::CLK_32,
                64 => pmc_mckr::PRES_A::CLK_64,
                _ => panic!("Invalid master_clock prescaler"),
            })
        } else {
            None
        };

        let usb_pres = if self.use_usb {
            let usb_prescaler = pll_freq / 48_000_000;
            assert!((1..=16).contains(&usb_prescaler));
            Some(usb_prescaler as u8)
        } else {
            None
        };

        let check_pck_pres = |v: Hertz| match pll_freq / v.0 {
            1 => pmc_pck::PRES_A::CLK_1,
            2 => pmc_pck::PRES_A::CLK_2,
            4 => pmc_pck::PRES_A::CLK_4,
            8 => pmc_pck::PRES_A::CLK_8,
            16 => pmc_pck::PRES_A::CLK_16,
            32 => pmc_pck::PRES_A::CLK_32,
            64 => pmc_pck::PRES_A::CLK_64,
            _ => panic!("Invalid programmable clock prescaler"),
        };
        let pck1_pres = self.programmable_clock_1.map(check_pck_pres);
        let pck2_pres = self.programmable_clock_2.map(check_pck_pres);
        let pck3_pres = self.programmable_clock_3.map(check_pck_pres);

        // compute mul / div
        let gcd = gcd(pll_freq, main_freq.0);
        let div = main_freq.0 / gcd;
        let mul = (pll_freq / gcd) - 1;

        assert!(div <= 255 && 1 < mul && mul <= 80);
        // plladiv pllamul plladiv2
        Some(PllConfig {
            div: div as u8,
            mul: mul as u16,
            master_pres,
            usb_pres,
            pck1_pres,
            pck2_pres,
            pck3_pres,
        })
    }

    #[inline]
    pub fn freeze(self) -> Clocks {
        let pmc = unsafe { &*PMC::ptr() };

        // TODO: Configure and select slow clock
        /* if let SlowClock::External = self.slow_ck {
            let supc = unsafe { &*crate::pac::SUPC::ptr() };
            supc.cr
                .write_with_zero(|w| w.xtalsel().set_bit().key().passwd());
        }*/
        let slow_ck_freq: Hertz = 32.khz().into();

        // Configure and select Main Clock
        let main_ck_freq = match self.main_ck {
            MainClock::Internal(clk) => {
                pmc.ckgr_mor
                    .modify(|_, w| w.moscrcf().variant(clk).key().passwd());

                match clk {
                    ckgr_mor::MOSCRCF_A::_4_MHZ => 4,
                    ckgr_mor::MOSCRCF_A::_8_MHZ => 8,
                    ckgr_mor::MOSCRCF_A::_12_MHZ => 12,
                }
                .mhz()
                .into()
            }
            MainClock::External(freq) => {
                // enable external xtal: CKGR_MOR.MOSCXTEN
                pmc.ckgr_mor
                    .modify(|_, w| w.moscxten().set_bit().key().passwd());
                // wait for xtal ready PMC_SR.MOSCXTS
                while pmc.pmc_sr.read().moscxts().bit_is_clear() {}
                // switch clock CKGR_MOR.MOSCSEL
                pmc.ckgr_mor
                    .modify(|_, w| w.moscsel().set_bit().key().passwd());
                // wait for xtal selected PMC_SR.MOSCSELS
                while pmc.pmc_sr.read().moscsels().bit_is_clear() {}

                // check clock freq
                #[cfg(feature = "check-main-frequency")]
                {
                    while pmc.ckgr_mcfr.read().mainfrdy().bit_is_clear() {}
                    let mainf = u32::from(pmc.ckgr_mcfr.read().mainf().bits());
                    assert!((mainf * slow_ck_freq.0) / 16 == freq.0);
                }
                freq
            }
        };

        let pll_config = self.pll_configuration(main_ck_freq);

        // configure PLL & select clock(s)
        if let Some(pll_config) = pll_config {
            if let Some(usb_pres) = pll_config.usb_pres {
                // usb_pres range has been checked in pll_configuration
                pmc.pmc_usb
                    .modify(|_, w| unsafe { w.usbdiv().bits(usb_pres) });
            }

            pmc.ckgr_pllar.modify(|_, w| unsafe {
                w.diva().bits(pll_config.div);
                w.mula().bits(pll_config.mul);
                w.pllacount().bits(0x3f); // maximum value the field can hold
                w.one().set_bit();
                w
            });
            while pmc.pmc_sr.read().locka().bit_is_clear() {}
        }

        // select master clock
        if let Some(master_pres) = pll_config.and_then(|config| config.master_pres) {
            pmc.pmc_mckr.modify(|_, w| w.pres().variant(master_pres));
            while pmc.pmc_sr.read().mckrdy().bit_is_clear() {}

            // configure the flash
            setup_flash(self.master_clock);

            pmc.pmc_mckr
                .modify(|_, w| w.css().variant(pmc_mckr::CSS_A::PLLA_CLK));
            while pmc.pmc_sr.read().mckrdy().bit_is_clear() {}
        } else {
            let pres = match main_ck_freq.0 / self.master_clock.0 {
                1 => pmc_mckr::PRES_A::CLK_1,
                2 => pmc_mckr::PRES_A::CLK_2,
                3 => pmc_mckr::PRES_A::CLK_3,
                4 => pmc_mckr::PRES_A::CLK_4,
                8 => pmc_mckr::PRES_A::CLK_8,
                16 => pmc_mckr::PRES_A::CLK_16,
                32 => pmc_mckr::PRES_A::CLK_32,
                64 => pmc_mckr::PRES_A::CLK_64,
                _ => panic!("Invalid master clock prescaler"),
            };
            pmc.pmc_mckr
                .modify(|_, w| w.css().variant(pmc_mckr::CSS_A::PLLA_CLK));
            while pmc.pmc_sr.read().mckrdy().bit_is_clear() {}

            // configure the flash
            setup_flash(self.master_clock);

            pmc.pmc_mckr.modify(|_, w| w.pres().variant(pres));
            while pmc.pmc_sr.read().mckrdy().bit_is_clear() {}
        }

        // configure programmable clocks
        // TODO: Implement that

        Clocks {
            slow_clock: slow_ck_freq,
            master_clock: self.master_clock,
            programmable_clock_1: None,
            programmable_clock_2: None,
            programmable_clock_3: None,
            usb_clock_enabled: self.use_usb,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Clocks {
    pub slow_clock: Hertz,
    pub master_clock: Hertz,
    pub programmable_clock_1: Option<Hertz>,
    pub programmable_clock_2: Option<Hertz>,
    pub programmable_clock_3: Option<Hertz>,
    pub usb_clock_enabled: bool,
}
