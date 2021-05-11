#[doc = "Register `MFID5` reader"]
pub struct R(crate::R<MFID5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MFID5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<MFID5_SPEC>> for R {
    fn from(reader: crate::R<MFID5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MFID` reader - Family ID"]
pub struct MFID_R(crate::FieldReader<u32, u32>);
impl MFID_R {
    pub(crate) fn new(bits: u32) -> Self {
        MFID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MFID_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:28 - Family ID"]
    #[inline(always)]
    pub fn mfid(&self) -> MFID_R {
        MFID_R::new((self.bits & 0x1fff_ffff) as u32)
    }
}
#[doc = "Mailbox Family ID Register (MB = 5)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mfid5](index.html) module"]
pub struct MFID5_SPEC;
impl crate::RegisterSpec for MFID5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mfid5::R](R) reader structure"]
impl crate::Readable for MFID5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MFID5 to value 0"]
impl crate::Resettable for MFID5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
