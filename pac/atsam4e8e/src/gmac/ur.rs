#[doc = "Register `UR` reader"]
pub struct R(crate::R<UR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<UR_SPEC>> for R {
    fn from(reader: crate::R<UR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UR` writer"]
pub struct W(crate::W<UR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UR_SPEC>;
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
impl core::convert::From<crate::W<UR_SPEC>> for W {
    fn from(writer: crate::W<UR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RMIIMII` reader - "]
pub struct RMIIMII_R(crate::FieldReader<bool, bool>);
impl RMIIMII_R {
    pub(crate) fn new(bits: bool) -> Self {
        RMIIMII_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RMIIMII_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RMIIMII` writer - "]
pub struct RMIIMII_W<'a> {
    w: &'a mut W,
}
impl<'a> RMIIMII_W<'a> {
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
#[doc = "Field `HDFC` reader - Half Duplex Flow Control"]
pub struct HDFC_R(crate::FieldReader<bool, bool>);
impl HDFC_R {
    pub(crate) fn new(bits: bool) -> Self {
        HDFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HDFC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HDFC` writer - Half Duplex Flow Control"]
pub struct HDFC_W<'a> {
    w: &'a mut W,
}
impl<'a> HDFC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `BPDG` reader - BPDG Bypass Deglitchers"]
pub struct BPDG_R(crate::FieldReader<bool, bool>);
impl BPDG_R {
    pub(crate) fn new(bits: bool) -> Self {
        BPDG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BPDG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BPDG` writer - BPDG Bypass Deglitchers"]
pub struct BPDG_W<'a> {
    w: &'a mut W,
}
impl<'a> BPDG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rmiimii(&self) -> RMIIMII_R {
        RMIIMII_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 6 - Half Duplex Flow Control"]
    #[inline(always)]
    pub fn hdfc(&self) -> HDFC_R {
        HDFC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - BPDG Bypass Deglitchers"]
    #[inline(always)]
    pub fn bpdg(&self) -> BPDG_R {
        BPDG_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rmiimii(&mut self) -> RMIIMII_W {
        RMIIMII_W { w: self }
    }
    #[doc = "Bit 6 - Half Duplex Flow Control"]
    #[inline(always)]
    pub fn hdfc(&mut self) -> HDFC_W {
        HDFC_W { w: self }
    }
    #[doc = "Bit 7 - BPDG Bypass Deglitchers"]
    #[inline(always)]
    pub fn bpdg(&mut self) -> BPDG_W {
        BPDG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "User Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur](index.html) module"]
pub struct UR_SPEC;
impl crate::RegisterSpec for UR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ur::R](R) reader structure"]
impl crate::Readable for UR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ur::W](W) writer structure"]
impl crate::Writable for UR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UR to value 0"]
impl crate::Resettable for UR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
