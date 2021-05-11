#[doc = "Register `CCNT1` reader"]
pub struct R(crate::R<CCNT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCNT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CCNT1_SPEC>> for R {
    fn from(reader: crate::R<CCNT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CNT` reader - Channel Counter Register"]
pub struct CNT_R(crate::FieldReader<u32, u32>);
impl CNT_R {
    pub(crate) fn new(bits: u32) -> Self {
        CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:23 - Channel Counter Register"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
#[doc = "PWM Channel Counter Register (ch_num = 1)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccnt1](index.html) module"]
pub struct CCNT1_SPEC;
impl crate::RegisterSpec for CCNT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccnt1::R](R) reader structure"]
impl crate::Readable for CCNT1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CCNT1 to value 0"]
impl crate::Resettable for CCNT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
