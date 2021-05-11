#[doc = "Register `WPROT_MODE` reader"]
pub struct R(crate::R<WPROT_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WPROT_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<WPROT_MODE_SPEC>> for R {
    fn from(reader: crate::R<WPROT_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WPROT_MODE` writer"]
pub struct W(crate::W<WPROT_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WPROT_MODE_SPEC>;
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
impl core::convert::From<crate::W<WPROT_MODE_SPEC>> for W {
    fn from(writer: crate::W<WPROT_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WPROT` reader - Write protection bit"]
pub struct WPROT_R(crate::FieldReader<bool, bool>);
impl WPROT_R {
    pub(crate) fn new(bits: bool) -> Self {
        WPROT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WPROT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WPROT` writer - Write protection bit"]
pub struct WPROT_W<'a> {
    w: &'a mut W,
}
impl<'a> WPROT_W<'a> {
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
#[doc = "Field `SECURITY_CODE` reader - Write protection mode security code"]
pub struct SECURITY_CODE_R(crate::FieldReader<u32, u32>);
impl SECURITY_CODE_R {
    pub(crate) fn new(bits: u32) -> Self {
        SECURITY_CODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECURITY_CODE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECURITY_CODE` writer - Write protection mode security code"]
pub struct SECURITY_CODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SECURITY_CODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Write protection bit"]
    #[inline(always)]
    pub fn wprot(&self) -> WPROT_R {
        WPROT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:31 - Write protection mode security code"]
    #[inline(always)]
    pub fn security_code(&self) -> SECURITY_CODE_R {
        SECURITY_CODE_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 0 - Write protection bit"]
    #[inline(always)]
    pub fn wprot(&mut self) -> WPROT_W {
        WPROT_W { w: self }
    }
    #[doc = "Bits 8:31 - Write protection mode security code"]
    #[inline(always)]
    pub fn security_code(&mut self) -> SECURITY_CODE_W {
        SECURITY_CODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Protection Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wprot_mode](index.html) module"]
pub struct WPROT_MODE_SPEC;
impl crate::RegisterSpec for WPROT_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wprot_mode::R](R) reader structure"]
impl crate::Readable for WPROT_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wprot_mode::W](W) writer structure"]
impl crate::Writable for WPROT_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WPROT_MODE to value 0"]
impl crate::Resettable for WPROT_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
