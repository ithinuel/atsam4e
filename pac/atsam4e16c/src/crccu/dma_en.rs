#[doc = "Register `DMA_EN` writer"]
pub struct W(crate::W<DMA_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_EN_SPEC>;
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
impl core::convert::From<crate::W<DMA_EN_SPEC>> for W {
    fn from(writer: crate::W<DMA_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAEN` writer - DMA Enable Register"]
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
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
impl W {
    #[doc = "Bit 0 - DMA Enable Register"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRCCU DMA Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_en](index.html) module"]
pub struct DMA_EN_SPEC;
impl crate::RegisterSpec for DMA_EN_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dma_en::W](W) writer structure"]
impl crate::Writable for DMA_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_EN to value 0"]
impl crate::Resettable for DMA_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
