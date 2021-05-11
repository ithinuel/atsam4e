#[doc = "Register `CYCLE0` reader"]
pub struct R(crate::R<CYCLE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CYCLE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CYCLE0_SPEC>> for R {
    fn from(reader: crate::R<CYCLE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CYCLE0` writer"]
pub struct W(crate::W<CYCLE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CYCLE0_SPEC>;
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
impl core::convert::From<crate::W<CYCLE0_SPEC>> for W {
    fn from(writer: crate::W<CYCLE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NWE_CYCLE` reader - Total Write Cycle Length"]
pub struct NWE_CYCLE_R(crate::FieldReader<u16, u16>);
impl NWE_CYCLE_R {
    pub(crate) fn new(bits: u16) -> Self {
        NWE_CYCLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NWE_CYCLE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NWE_CYCLE` writer - Total Write Cycle Length"]
pub struct NWE_CYCLE_W<'a> {
    w: &'a mut W,
}
impl<'a> NWE_CYCLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
#[doc = "Field `NRD_CYCLE` reader - Total Read Cycle Length"]
pub struct NRD_CYCLE_R(crate::FieldReader<u16, u16>);
impl NRD_CYCLE_R {
    pub(crate) fn new(bits: u16) -> Self {
        NRD_CYCLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NRD_CYCLE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NRD_CYCLE` writer - Total Read Cycle Length"]
pub struct NRD_CYCLE_W<'a> {
    w: &'a mut W,
}
impl<'a> NRD_CYCLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | (((value as u32) & 0x01ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Total Write Cycle Length"]
    #[inline(always)]
    pub fn nwe_cycle(&self) -> NWE_CYCLE_R {
        NWE_CYCLE_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - Total Read Cycle Length"]
    #[inline(always)]
    pub fn nrd_cycle(&self) -> NRD_CYCLE_R {
        NRD_CYCLE_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Total Write Cycle Length"]
    #[inline(always)]
    pub fn nwe_cycle(&mut self) -> NWE_CYCLE_W {
        NWE_CYCLE_W { w: self }
    }
    #[doc = "Bits 16:24 - Total Read Cycle Length"]
    #[inline(always)]
    pub fn nrd_cycle(&mut self) -> NRD_CYCLE_W {
        NRD_CYCLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SMC Cycle Register (CS_number = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cycle0](index.html) module"]
pub struct CYCLE0_SPEC;
impl crate::RegisterSpec for CYCLE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cycle0::R](R) reader structure"]
impl crate::Readable for CYCLE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cycle0::W](W) writer structure"]
impl crate::Writable for CYCLE0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CYCLE0 to value 0x0003_0003"]
impl crate::Resettable for CYCLE0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0003_0003
    }
}
