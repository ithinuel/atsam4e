#[doc = "Reader of register AR"]
pub type R = crate::R<u32, super::AR>;
#[doc = "Writer for register AR"]
pub type W = crate::W<u32, super::AR>;
#[doc = "Register AR `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::AR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `ALMV`"]
pub type ALMV_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ALMV`"]
pub struct ALMV_W<'a> {
    w: &'a mut W,
}
impl<'a> ALMV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Alarm Value"]
    #[inline(always)]
    pub fn almv(&self) -> ALMV_R {
        ALMV_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Alarm Value"]
    #[inline(always)]
    pub fn almv(&mut self) -> ALMV_W {
        ALMV_W { w: self }
    }
}
