#[doc = "Reader of register IMR0"]
pub type R = crate::R<u32, super::IMR0>;
#[doc = "Reader of field `COVFS`"]
pub type COVFS_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOVRS`"]
pub type LOVRS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CPAS`"]
pub type CPAS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CPBS`"]
pub type CPBS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CPCS`"]
pub type CPCS_R = crate::R<bool, bool>;
#[doc = "Reader of field `LDRAS`"]
pub type LDRAS_R = crate::R<bool, bool>;
#[doc = "Reader of field `LDRBS`"]
pub type LDRBS_R = crate::R<bool, bool>;
#[doc = "Reader of field `ETRGS`"]
pub type ETRGS_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENDRX`"]
pub type ENDRX_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXBUFF`"]
pub type RXBUFF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Counter Overflow"]
    #[inline(always)]
    pub fn covfs(&self) -> COVFS_R {
        COVFS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Load Overrun"]
    #[inline(always)]
    pub fn lovrs(&self) -> LOVRS_R {
        LOVRS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RA Compare"]
    #[inline(always)]
    pub fn cpas(&self) -> CPAS_R {
        CPAS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RB Compare"]
    #[inline(always)]
    pub fn cpbs(&self) -> CPBS_R {
        CPBS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RC Compare"]
    #[inline(always)]
    pub fn cpcs(&self) -> CPCS_R {
        CPCS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RA Loading"]
    #[inline(always)]
    pub fn ldras(&self) -> LDRAS_R {
        LDRAS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RB Loading"]
    #[inline(always)]
    pub fn ldrbs(&self) -> LDRBS_R {
        LDRBS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - External Trigger"]
    #[inline(always)]
    pub fn etrgs(&self) -> ETRGS_R {
        ETRGS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - End of Receiver Transfer"]
    #[inline(always)]
    pub fn endrx(&self) -> ENDRX_R {
        ENDRX_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Reception Buffer Full"]
    #[inline(always)]
    pub fn rxbuff(&self) -> RXBUFF_R {
        RXBUFF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
