#[doc = "Register `CAE3` reader"]
pub struct R(crate::R<CAE3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAE3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CAE3_SPEC>> for R {
    fn from(reader: crate::R<CAE3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAE3` writer"]
pub struct W(crate::W<CAE3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAE3_SPEC>;
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
impl core::convert::From<crate::W<CAE3_SPEC>> for W {
    fn from(writer: crate::W<CAE3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADEDGV` reader - Channel Additional Edge Value"]
pub struct ADEDGV_R(crate::FieldReader<u32, u32>);
impl ADEDGV_R {
    pub(crate) fn new(bits: u32) -> Self {
        ADEDGV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADEDGV_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADEDGV` writer - Channel Additional Edge Value"]
pub struct ADEDGV_W<'a> {
    w: &'a mut W,
}
impl<'a> ADEDGV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
#[doc = "Channel Additional Edge Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADEDGM_A {
    #[doc = "0: The additional edge of the channel x output waveform occurs when CCNTx reaches ADEDGV and the counter of the channel x is incrementing."]
    INC = 0,
    #[doc = "1: The additional edge of the channel x output waveform occurs when CCNTx reaches ADEDGV and the counter of the channel x is incrementing."]
    DEC = 1,
    #[doc = "2: The additional edge of the channel x output waveform occurs when CCNTx reaches ADEDGV, whether the counter is incrementing or not."]
    BOTH = 2,
}
impl From<ADEDGM_A> for u8 {
    #[inline(always)]
    fn from(variant: ADEDGM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADEDGM` reader - Channel Additional Edge Mode"]
pub struct ADEDGM_R(crate::FieldReader<u8, ADEDGM_A>);
impl ADEDGM_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADEDGM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ADEDGM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ADEDGM_A::INC),
            1 => Val(ADEDGM_A::DEC),
            2 => Val(ADEDGM_A::BOTH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `INC`"]
    #[inline(always)]
    pub fn is_inc(&self) -> bool {
        **self == ADEDGM_A::INC
    }
    #[doc = "Checks if the value of the field is `DEC`"]
    #[inline(always)]
    pub fn is_dec(&self) -> bool {
        **self == ADEDGM_A::DEC
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        **self == ADEDGM_A::BOTH
    }
}
impl core::ops::Deref for ADEDGM_R {
    type Target = crate::FieldReader<u8, ADEDGM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADEDGM` writer - Channel Additional Edge Mode"]
pub struct ADEDGM_W<'a> {
    w: &'a mut W,
}
impl<'a> ADEDGM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADEDGM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The additional edge of the channel x output waveform occurs when CCNTx reaches ADEDGV and the counter of the channel x is incrementing."]
    #[inline(always)]
    pub fn inc(self) -> &'a mut W {
        self.variant(ADEDGM_A::INC)
    }
    #[doc = "The additional edge of the channel x output waveform occurs when CCNTx reaches ADEDGV and the counter of the channel x is incrementing."]
    #[inline(always)]
    pub fn dec(self) -> &'a mut W {
        self.variant(ADEDGM_A::DEC)
    }
    #[doc = "The additional edge of the channel x output waveform occurs when CCNTx reaches ADEDGV, whether the counter is incrementing or not."]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(ADEDGM_A::BOTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Channel Additional Edge Value"]
    #[inline(always)]
    pub fn adedgv(&self) -> ADEDGV_R {
        ADEDGV_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 24:25 - Channel Additional Edge Mode"]
    #[inline(always)]
    pub fn adedgm(&self) -> ADEDGM_R {
        ADEDGM_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - Channel Additional Edge Value"]
    #[inline(always)]
    pub fn adedgv(&mut self) -> ADEDGV_W {
        ADEDGV_W { w: self }
    }
    #[doc = "Bits 24:25 - Channel Additional Edge Mode"]
    #[inline(always)]
    pub fn adedgm(&mut self) -> ADEDGM_W {
        ADEDGM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Channel Additional Edge Register (ch_num = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cae3](index.html) module"]
pub struct CAE3_SPEC;
impl crate::RegisterSpec for CAE3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cae3::R](R) reader structure"]
impl crate::Readable for CAE3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cae3::W](W) writer structure"]
impl crate::Writable for CAE3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAE3 to value 0"]
impl crate::Resettable for CAE3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
