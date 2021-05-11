#[doc = "Register `ISR` reader"]
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ISR_SPEC>> for R {
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EOC0` reader - End of Conversion 0"]
pub struct EOC0_R(crate::FieldReader<bool, bool>);
impl EOC0_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOC0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOC0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOC1` reader - End of Conversion 1"]
pub struct EOC1_R(crate::FieldReader<bool, bool>);
impl EOC1_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOC1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOC2` reader - End of Conversion 2"]
pub struct EOC2_R(crate::FieldReader<bool, bool>);
impl EOC2_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOC2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOC3` reader - End of Conversion 3"]
pub struct EOC3_R(crate::FieldReader<bool, bool>);
impl EOC3_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOC3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOC3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOC4` reader - End of Conversion 4"]
pub struct EOC4_R(crate::FieldReader<bool, bool>);
impl EOC4_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOC4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOC4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOC5` reader - End of Conversion 5"]
pub struct EOC5_R(crate::FieldReader<bool, bool>);
impl EOC5_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOC5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOC5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOC6` reader - End of Conversion 6"]
pub struct EOC6_R(crate::FieldReader<bool, bool>);
impl EOC6_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOC6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOC6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOC7` reader - End of Conversion 7"]
pub struct EOC7_R(crate::FieldReader<bool, bool>);
impl EOC7_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOC7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOC7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOC8` reader - End of Conversion 8"]
pub struct EOC8_R(crate::FieldReader<bool, bool>);
impl EOC8_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOC8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOC8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOC9` reader - End of Conversion 9"]
pub struct EOC9_R(crate::FieldReader<bool, bool>);
impl EOC9_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOC9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOC9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOC10` reader - End of Conversion 10"]
pub struct EOC10_R(crate::FieldReader<bool, bool>);
impl EOC10_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOC10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOC10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOC11` reader - End of Conversion 11"]
pub struct EOC11_R(crate::FieldReader<bool, bool>);
impl EOC11_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOC11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOC11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOC12` reader - End of Conversion 12"]
pub struct EOC12_R(crate::FieldReader<bool, bool>);
impl EOC12_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOC12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOC12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOC13` reader - End of Conversion 13"]
pub struct EOC13_R(crate::FieldReader<bool, bool>);
impl EOC13_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOC13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOC13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOC14` reader - End of Conversion 14"]
pub struct EOC14_R(crate::FieldReader<bool, bool>);
impl EOC14_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOC14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOC14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOC15` reader - End of Conversion 15"]
pub struct EOC15_R(crate::FieldReader<bool, bool>);
impl EOC15_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOC15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOC15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOC16` reader - End of Conversion 16"]
pub struct EOC16_R(crate::FieldReader<bool, bool>);
impl EOC16_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOC16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOC16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOC17` reader - End of Conversion 17"]
pub struct EOC17_R(crate::FieldReader<bool, bool>);
impl EOC17_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOC17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOC17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOC18` reader - End of Conversion 18"]
pub struct EOC18_R(crate::FieldReader<bool, bool>);
impl EOC18_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOC18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOC18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOC19` reader - End of Conversion 19"]
pub struct EOC19_R(crate::FieldReader<bool, bool>);
impl EOC19_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOC19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOC19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOC20` reader - End of Conversion 20"]
pub struct EOC20_R(crate::FieldReader<bool, bool>);
impl EOC20_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOC20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOC20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOC21` reader - End of Conversion 21"]
pub struct EOC21_R(crate::FieldReader<bool, bool>);
impl EOC21_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOC21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOC21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOC22` reader - End of Conversion 22"]
pub struct EOC22_R(crate::FieldReader<bool, bool>);
impl EOC22_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOC22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOC22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOC23` reader - End of Conversion 23"]
pub struct EOC23_R(crate::FieldReader<bool, bool>);
impl EOC23_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOC23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOC23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRDY` reader - Data Ready"]
pub struct DRDY_R(crate::FieldReader<bool, bool>);
impl DRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        DRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GOVRE` reader - General Overrun Error"]
pub struct GOVRE_R(crate::FieldReader<bool, bool>);
impl GOVRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        GOVRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GOVRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPE` reader - Comparison Error"]
pub struct COMPE_R(crate::FieldReader<bool, bool>);
impl COMPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENDRX` reader - End of RX Buffer"]
pub struct ENDRX_R(crate::FieldReader<bool, bool>);
impl ENDRX_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENDRX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENDRX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXBUFF` reader - RX Buffer Full"]
pub struct RXBUFF_R(crate::FieldReader<bool, bool>);
impl RXBUFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXBUFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXBUFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEMPCHG` reader - Temperature Change"]
pub struct TEMPCHG_R(crate::FieldReader<bool, bool>);
impl TEMPCHG_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEMPCHG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEMPCHG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOCAL` reader - End of Calibration Sequence"]
pub struct EOCAL_R(crate::FieldReader<bool, bool>);
impl EOCAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOCAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOCAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - End of Conversion 0"]
    #[inline(always)]
    pub fn eoc0(&self) -> EOC0_R {
        EOC0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - End of Conversion 1"]
    #[inline(always)]
    pub fn eoc1(&self) -> EOC1_R {
        EOC1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - End of Conversion 2"]
    #[inline(always)]
    pub fn eoc2(&self) -> EOC2_R {
        EOC2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - End of Conversion 3"]
    #[inline(always)]
    pub fn eoc3(&self) -> EOC3_R {
        EOC3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - End of Conversion 4"]
    #[inline(always)]
    pub fn eoc4(&self) -> EOC4_R {
        EOC4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - End of Conversion 5"]
    #[inline(always)]
    pub fn eoc5(&self) -> EOC5_R {
        EOC5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - End of Conversion 6"]
    #[inline(always)]
    pub fn eoc6(&self) -> EOC6_R {
        EOC6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - End of Conversion 7"]
    #[inline(always)]
    pub fn eoc7(&self) -> EOC7_R {
        EOC7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - End of Conversion 8"]
    #[inline(always)]
    pub fn eoc8(&self) -> EOC8_R {
        EOC8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - End of Conversion 9"]
    #[inline(always)]
    pub fn eoc9(&self) -> EOC9_R {
        EOC9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - End of Conversion 10"]
    #[inline(always)]
    pub fn eoc10(&self) -> EOC10_R {
        EOC10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - End of Conversion 11"]
    #[inline(always)]
    pub fn eoc11(&self) -> EOC11_R {
        EOC11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - End of Conversion 12"]
    #[inline(always)]
    pub fn eoc12(&self) -> EOC12_R {
        EOC12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - End of Conversion 13"]
    #[inline(always)]
    pub fn eoc13(&self) -> EOC13_R {
        EOC13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - End of Conversion 14"]
    #[inline(always)]
    pub fn eoc14(&self) -> EOC14_R {
        EOC14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - End of Conversion 15"]
    #[inline(always)]
    pub fn eoc15(&self) -> EOC15_R {
        EOC15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - End of Conversion 16"]
    #[inline(always)]
    pub fn eoc16(&self) -> EOC16_R {
        EOC16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - End of Conversion 17"]
    #[inline(always)]
    pub fn eoc17(&self) -> EOC17_R {
        EOC17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - End of Conversion 18"]
    #[inline(always)]
    pub fn eoc18(&self) -> EOC18_R {
        EOC18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - End of Conversion 19"]
    #[inline(always)]
    pub fn eoc19(&self) -> EOC19_R {
        EOC19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - End of Conversion 20"]
    #[inline(always)]
    pub fn eoc20(&self) -> EOC20_R {
        EOC20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - End of Conversion 21"]
    #[inline(always)]
    pub fn eoc21(&self) -> EOC21_R {
        EOC21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - End of Conversion 22"]
    #[inline(always)]
    pub fn eoc22(&self) -> EOC22_R {
        EOC22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - End of Conversion 23"]
    #[inline(always)]
    pub fn eoc23(&self) -> EOC23_R {
        EOC23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Data Ready"]
    #[inline(always)]
    pub fn drdy(&self) -> DRDY_R {
        DRDY_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - General Overrun Error"]
    #[inline(always)]
    pub fn govre(&self) -> GOVRE_R {
        GOVRE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Comparison Error"]
    #[inline(always)]
    pub fn compe(&self) -> COMPE_R {
        COMPE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - End of RX Buffer"]
    #[inline(always)]
    pub fn endrx(&self) -> ENDRX_R {
        ENDRX_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - RX Buffer Full"]
    #[inline(always)]
    pub fn rxbuff(&self) -> RXBUFF_R {
        RXBUFF_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Temperature Change"]
    #[inline(always)]
    pub fn tempchg(&self) -> TEMPCHG_R {
        TEMPCHG_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - End of Calibration Sequence"]
    #[inline(always)]
    pub fn eocal(&self) -> EOCAL_R {
        EOCAL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr::R](R) reader structure"]
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
