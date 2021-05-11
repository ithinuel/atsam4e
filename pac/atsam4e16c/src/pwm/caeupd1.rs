#[doc = "Register `CAEUPD1` writer"]
pub struct W(crate::W<CAEUPD1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAEUPD1_SPEC>;
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
impl core::convert::From<crate::W<CAEUPD1_SPEC>> for W {
    fn from(writer: crate::W<CAEUPD1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADEDGVUP` writer - Channel Additional Edge Value Update"]
pub struct ADEDGVUP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADEDGVUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
#[doc = "Channel Additional Edge Mode Update"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADEDGMUP_AW {
    #[doc = "0: The additional edge of the channel x output waveform occurs when CCNTx reaches ADEDGVUP and the counter of the channel x is incrementing."]
    INC = 0,
    #[doc = "1: The additional edge of the channel x output waveform occurs when CCNTx reaches ADEDGVUP and the counter of the channel x is incrementing."]
    DEC = 1,
    #[doc = "2: The additional edge of the channel x output waveform occurs when CCNTx reaches ADEDGVUP, whether the counter is incrementing or not."]
    BOTH = 2,
}
impl From<ADEDGMUP_AW> for u8 {
    #[inline(always)]
    fn from(variant: ADEDGMUP_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `ADEDGMUP` writer - Channel Additional Edge Mode Update"]
pub struct ADEDGMUP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADEDGMUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADEDGMUP_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The additional edge of the channel x output waveform occurs when CCNTx reaches ADEDGVUP and the counter of the channel x is incrementing."]
    #[inline(always)]
    pub fn inc(self) -> &'a mut W {
        self.variant(ADEDGMUP_AW::INC)
    }
    #[doc = "The additional edge of the channel x output waveform occurs when CCNTx reaches ADEDGVUP and the counter of the channel x is incrementing."]
    #[inline(always)]
    pub fn dec(self) -> &'a mut W {
        self.variant(ADEDGMUP_AW::DEC)
    }
    #[doc = "The additional edge of the channel x output waveform occurs when CCNTx reaches ADEDGVUP, whether the counter is incrementing or not."]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(ADEDGMUP_AW::BOTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:23 - Channel Additional Edge Value Update"]
    #[inline(always)]
    pub fn adedgvup(&mut self) -> ADEDGVUP_W {
        ADEDGVUP_W { w: self }
    }
    #[doc = "Bits 24:25 - Channel Additional Edge Mode Update"]
    #[inline(always)]
    pub fn adedgmup(&mut self) -> ADEDGMUP_W {
        ADEDGMUP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Channel Additional Edge Update Register (ch_num = 1)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [caeupd1](index.html) module"]
pub struct CAEUPD1_SPEC;
impl crate::RegisterSpec for CAEUPD1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [caeupd1::W](W) writer structure"]
impl crate::Writable for CAEUPD1_SPEC {
    type Writer = W;
}
