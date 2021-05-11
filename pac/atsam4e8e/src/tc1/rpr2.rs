#[doc = "Register `RPR2` reader"]
pub struct R(crate::R<RPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RPR2_SPEC>> for R {
    fn from(reader: crate::R<RPR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RPR2` writer"]
pub struct W(crate::W<RPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RPR2_SPEC>;
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
impl core::convert::From<crate::W<RPR2_SPEC>> for W {
    fn from(writer: crate::W<RPR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXPTR` reader - Receive Pointer Register"]
pub struct RXPTR_R(crate::FieldReader<u32, u32>);
impl RXPTR_R {
    pub(crate) fn new(bits: u32) -> Self {
        RXPTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXPTR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXPTR` writer - Receive Pointer Register"]
pub struct RXPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Receive Pointer Register"]
    #[inline(always)]
    pub fn rxptr(&self) -> RXPTR_R {
        RXPTR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive Pointer Register"]
    #[inline(always)]
    pub fn rxptr(&mut self) -> RXPTR_W {
        RXPTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Pointer Register (pdc = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpr2](index.html) module"]
pub struct RPR2_SPEC;
impl crate::RegisterSpec for RPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rpr2::R](R) reader structure"]
impl crate::Readable for RPR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rpr2::W](W) writer structure"]
impl crate::Writable for RPR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RPR2 to value 0"]
impl crate::Resettable for RPR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
