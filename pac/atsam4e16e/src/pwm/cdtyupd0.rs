#[doc = "Register `CDTYUPD0` writer"]
pub struct W(crate::W<CDTYUPD0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CDTYUPD0_SPEC>;
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
impl core::convert::From<crate::W<CDTYUPD0_SPEC>> for W {
    fn from(writer: crate::W<CDTYUPD0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CDTYUPD` writer - Channel Duty-Cycle Update"]
pub struct CDTYUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> CDTYUPD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:23 - Channel Duty-Cycle Update"]
    #[inline(always)]
    pub fn cdtyupd(&mut self) -> CDTYUPD_W {
        CDTYUPD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Channel Duty Cycle Update Register (ch_num = 0)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdtyupd0](index.html) module"]
pub struct CDTYUPD0_SPEC;
impl crate::RegisterSpec for CDTYUPD0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cdtyupd0::W](W) writer structure"]
impl crate::Writable for CDTYUPD0_SPEC {
    type Writer = W;
}
