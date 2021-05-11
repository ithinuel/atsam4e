#[doc = "Register `DADDR2` reader"]
pub struct R(crate::R<DADDR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DADDR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DADDR2_SPEC>> for R {
    fn from(reader: crate::R<DADDR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DADDR2` writer"]
pub struct W(crate::W<DADDR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DADDR2_SPEC>;
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
impl core::convert::From<crate::W<DADDR2_SPEC>> for W {
    fn from(writer: crate::W<DADDR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DADDR` reader - Channel x Destination Address"]
pub struct DADDR_R(crate::FieldReader<u32, u32>);
impl DADDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        DADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DADDR` writer - Channel x Destination Address"]
pub struct DADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Channel x Destination Address"]
    #[inline(always)]
    pub fn daddr(&self) -> DADDR_R {
        DADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel x Destination Address"]
    #[inline(always)]
    pub fn daddr(&mut self) -> DADDR_W {
        DADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC Channel Destination Address Register (ch_num = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [daddr2](index.html) module"]
pub struct DADDR2_SPEC;
impl crate::RegisterSpec for DADDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [daddr2::R](R) reader structure"]
impl crate::Readable for DADDR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [daddr2::W](W) writer structure"]
impl crate::Writable for DADDR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DADDR2 to value 0"]
impl crate::Resettable for DADDR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
