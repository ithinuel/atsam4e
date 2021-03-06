#[doc = "Reader of register PRBS10"]
pub type R = crate::R<u32, super::PRBS10>;
#[doc = "Writer for register PRBS10"]
pub type W = crate::W<u32, super::PRBS10>;
#[doc = "Register PRBS10 `reset()`'s with value 0x3333_3333"]
impl crate::ResetValue for super::PRBS10 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3333_3333
    }
}
#[doc = "Reader of field `M8PR`"]
pub type M8PR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `M8PR`"]
pub struct M8PR_W<'a> {
    w: &'a mut W,
}
impl<'a> M8PR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `M9PR`"]
pub type M9PR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `M9PR`"]
pub struct M9PR_W<'a> {
    w: &'a mut W,
}
impl<'a> M9PR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `M10PR`"]
pub type M10PR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `M10PR`"]
pub struct M10PR_W<'a> {
    w: &'a mut W,
}
impl<'a> M10PR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `M11PR`"]
pub type M11PR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `M11PR`"]
pub struct M11PR_W<'a> {
    w: &'a mut W,
}
impl<'a> M11PR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `M12PR`"]
pub type M12PR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `M12PR`"]
pub struct M12PR_W<'a> {
    w: &'a mut W,
}
impl<'a> M12PR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `M13PR`"]
pub type M13PR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `M13PR`"]
pub struct M13PR_W<'a> {
    w: &'a mut W,
}
impl<'a> M13PR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `M14PR`"]
pub type M14PR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `M14PR`"]
pub struct M14PR_W<'a> {
    w: &'a mut W,
}
impl<'a> M14PR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `M15PR`"]
pub type M15PR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `M15PR`"]
pub struct M15PR_W<'a> {
    w: &'a mut W,
}
impl<'a> M15PR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Master 8 Priority"]
    #[inline(always)]
    pub fn m8pr(&self) -> M8PR_R {
        M8PR_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Master 9 Priority"]
    #[inline(always)]
    pub fn m9pr(&self) -> M9PR_R {
        M9PR_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Master 10 Priority"]
    #[inline(always)]
    pub fn m10pr(&self) -> M10PR_R {
        M10PR_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Master 11 Priority"]
    #[inline(always)]
    pub fn m11pr(&self) -> M11PR_R {
        M11PR_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Master 12 Priority"]
    #[inline(always)]
    pub fn m12pr(&self) -> M12PR_R {
        M12PR_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Master 13 Priority"]
    #[inline(always)]
    pub fn m13pr(&self) -> M13PR_R {
        M13PR_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Master 14 Priority"]
    #[inline(always)]
    pub fn m14pr(&self) -> M14PR_R {
        M14PR_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Master 15 Priority"]
    #[inline(always)]
    pub fn m15pr(&self) -> M15PR_R {
        M15PR_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Master 8 Priority"]
    #[inline(always)]
    pub fn m8pr(&mut self) -> M8PR_W {
        M8PR_W { w: self }
    }
    #[doc = "Bits 4:5 - Master 9 Priority"]
    #[inline(always)]
    pub fn m9pr(&mut self) -> M9PR_W {
        M9PR_W { w: self }
    }
    #[doc = "Bits 8:9 - Master 10 Priority"]
    #[inline(always)]
    pub fn m10pr(&mut self) -> M10PR_W {
        M10PR_W { w: self }
    }
    #[doc = "Bits 12:13 - Master 11 Priority"]
    #[inline(always)]
    pub fn m11pr(&mut self) -> M11PR_W {
        M11PR_W { w: self }
    }
    #[doc = "Bits 16:17 - Master 12 Priority"]
    #[inline(always)]
    pub fn m12pr(&mut self) -> M12PR_W {
        M12PR_W { w: self }
    }
    #[doc = "Bits 20:21 - Master 13 Priority"]
    #[inline(always)]
    pub fn m13pr(&mut self) -> M13PR_W {
        M13PR_W { w: self }
    }
    #[doc = "Bits 24:25 - Master 14 Priority"]
    #[inline(always)]
    pub fn m14pr(&mut self) -> M14PR_W {
        M14PR_W { w: self }
    }
    #[doc = "Bits 28:29 - Master 15 Priority"]
    #[inline(always)]
    pub fn m15pr(&mut self) -> M15PR_W {
        M15PR_W { w: self }
    }
}
