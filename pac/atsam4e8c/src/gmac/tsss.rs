#[doc = "Register `TSSS` reader"]
pub struct R(crate::R<TSSS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSSS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TSSS_SPEC>> for R {
    fn from(reader: crate::R<TSSS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSSS` writer"]
pub struct W(crate::W<TSSS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSSS_SPEC>;
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
impl core::convert::From<crate::W<TSSS_SPEC>> for W {
    fn from(writer: crate::W<TSSS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VTS` reader - Value of Timer Seconds Register Capture"]
pub struct VTS_R(crate::FieldReader<u32, u32>);
impl VTS_R {
    pub(crate) fn new(bits: u32) -> Self {
        VTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VTS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VTS` writer - Value of Timer Seconds Register Capture"]
pub struct VTS_W<'a> {
    w: &'a mut W,
}
impl<'a> VTS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Value of Timer Seconds Register Capture"]
    #[inline(always)]
    pub fn vts(&self) -> VTS_R {
        VTS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value of Timer Seconds Register Capture"]
    #[inline(always)]
    pub fn vts(&mut self) -> VTS_W {
        VTS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "1588 Timer Sync Strobe Seconds Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsss](index.html) module"]
pub struct TSSS_SPEC;
impl crate::RegisterSpec for TSSS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsss::R](R) reader structure"]
impl crate::Readable for TSSS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsss::W](W) writer structure"]
impl crate::Writable for TSSS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSSS to value 0"]
impl crate::Resettable for TSSS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
