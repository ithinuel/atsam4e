#[doc = "Writer for register IDR2"]
pub type W = crate::W<u32, super::IDR2>;
#[doc = "Write proxy for field `COVFS`"]
pub struct COVFS_W<'a> {
    w: &'a mut W,
}
impl<'a> COVFS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Write proxy for field `LOVRS`"]
pub struct LOVRS_W<'a> {
    w: &'a mut W,
}
impl<'a> LOVRS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Write proxy for field `CPAS`"]
pub struct CPAS_W<'a> {
    w: &'a mut W,
}
impl<'a> CPAS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Write proxy for field `CPBS`"]
pub struct CPBS_W<'a> {
    w: &'a mut W,
}
impl<'a> CPBS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Write proxy for field `CPCS`"]
pub struct CPCS_W<'a> {
    w: &'a mut W,
}
impl<'a> CPCS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Write proxy for field `LDRAS`"]
pub struct LDRAS_W<'a> {
    w: &'a mut W,
}
impl<'a> LDRAS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Write proxy for field `LDRBS`"]
pub struct LDRBS_W<'a> {
    w: &'a mut W,
}
impl<'a> LDRBS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Write proxy for field `ETRGS`"]
pub struct ETRGS_W<'a> {
    w: &'a mut W,
}
impl<'a> ETRGS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Write proxy for field `ENDRX`"]
pub struct ENDRX_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDRX_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Write proxy for field `RXBUFF`"]
pub struct RXBUFF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXBUFF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Counter Overflow"]
    #[inline(always)]
    pub fn covfs(&mut self) -> COVFS_W {
        COVFS_W { w: self }
    }
    #[doc = "Bit 1 - Load Overrun"]
    #[inline(always)]
    pub fn lovrs(&mut self) -> LOVRS_W {
        LOVRS_W { w: self }
    }
    #[doc = "Bit 2 - RA Compare"]
    #[inline(always)]
    pub fn cpas(&mut self) -> CPAS_W {
        CPAS_W { w: self }
    }
    #[doc = "Bit 3 - RB Compare"]
    #[inline(always)]
    pub fn cpbs(&mut self) -> CPBS_W {
        CPBS_W { w: self }
    }
    #[doc = "Bit 4 - RC Compare"]
    #[inline(always)]
    pub fn cpcs(&mut self) -> CPCS_W {
        CPCS_W { w: self }
    }
    #[doc = "Bit 5 - RA Loading"]
    #[inline(always)]
    pub fn ldras(&mut self) -> LDRAS_W {
        LDRAS_W { w: self }
    }
    #[doc = "Bit 6 - RB Loading"]
    #[inline(always)]
    pub fn ldrbs(&mut self) -> LDRBS_W {
        LDRBS_W { w: self }
    }
    #[doc = "Bit 7 - External Trigger"]
    #[inline(always)]
    pub fn etrgs(&mut self) -> ETRGS_W {
        ETRGS_W { w: self }
    }
    #[doc = "Bit 8 - End of Receiver Transfer"]
    #[inline(always)]
    pub fn endrx(&mut self) -> ENDRX_W {
        ENDRX_W { w: self }
    }
    #[doc = "Bit 9 - Reception Buffer Full"]
    #[inline(always)]
    pub fn rxbuff(&mut self) -> RXBUFF_W {
        RXBUFF_W { w: self }
    }
}
