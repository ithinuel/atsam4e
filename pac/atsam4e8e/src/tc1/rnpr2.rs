#[doc = "Register `RNPR2` reader"]
pub struct R(crate::R<RNPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RNPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RNPR2_SPEC>> for R {
    fn from(reader: crate::R<RNPR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RNPR2` writer"]
pub struct W(crate::W<RNPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RNPR2_SPEC>;
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
impl core::convert::From<crate::W<RNPR2_SPEC>> for W {
    fn from(writer: crate::W<RNPR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXNPTR` reader - Receive Next Pointer"]
pub struct RXNPTR_R(crate::FieldReader<u32, u32>);
impl RXNPTR_R {
    pub(crate) fn new(bits: u32) -> Self {
        RXNPTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXNPTR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXNPTR` writer - Receive Next Pointer"]
pub struct RXNPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXNPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Receive Next Pointer"]
    #[inline(always)]
    pub fn rxnptr(&self) -> RXNPTR_R {
        RXNPTR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive Next Pointer"]
    #[inline(always)]
    pub fn rxnptr(&mut self) -> RXNPTR_W {
        RXNPTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Next Pointer Register (pdc = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rnpr2](index.html) module"]
pub struct RNPR2_SPEC;
impl crate::RegisterSpec for RNPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rnpr2::R](R) reader structure"]
impl crate::Readable for RNPR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rnpr2::W](W) writer structure"]
impl crate::Writable for RNPR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RNPR2 to value 0"]
impl crate::Resettable for RNPR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
