#[doc = "Register `PTCR0` writer"]
pub struct W(crate::W<PTCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<PTCR0_SPEC>> for W {
    fn from(writer: crate::W<PTCR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXTEN` writer - Receiver Transfer Enable"]
pub struct RXTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTEN_W<'a> {
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
#[doc = "Field `RXTDIS` writer - Receiver Transfer Disable"]
pub struct RXTDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTDIS_W<'a> {
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
#[doc = "Field `TXTEN` writer - Transmitter Transfer Enable"]
pub struct TXTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXTEN_W<'a> {
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
#[doc = "Field `TXTDIS` writer - Transmitter Transfer Disable"]
pub struct TXTDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXTDIS_W<'a> {
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
    #[doc = "Bit 0 - Receiver Transfer Enable"]
    #[inline(always)]
    pub fn rxten(&mut self) -> RXTEN_W {
        RXTEN_W { w: self }
    }
    #[doc = "Bit 1 - Receiver Transfer Disable"]
    #[inline(always)]
    pub fn rxtdis(&mut self) -> RXTDIS_W {
        RXTDIS_W { w: self }
    }
    #[doc = "Bit 8 - Transmitter Transfer Enable"]
    #[inline(always)]
    pub fn txten(&mut self) -> TXTEN_W {
        TXTEN_W { w: self }
    }
    #[doc = "Bit 9 - Transmitter Transfer Disable"]
    #[inline(always)]
    pub fn txtdis(&mut self) -> TXTDIS_W {
        TXTDIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transfer Control Register (pdc = 0)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptcr0](index.html) module"]
pub struct PTCR0_SPEC;
impl crate::RegisterSpec for PTCR0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ptcr0::W](W) writer structure"]
impl crate::Writable for PTCR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PTCR0 to value 0"]
impl crate::Resettable for PTCR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
