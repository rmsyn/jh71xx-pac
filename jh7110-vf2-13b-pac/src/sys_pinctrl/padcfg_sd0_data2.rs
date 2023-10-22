#[doc = "Register `padcfg_sd0_data2` reader"]
pub type R = crate::R<PADCFG_SD0_DATA2_SPEC>;
#[doc = "Register `padcfg_sd0_data2` writer"]
pub type W = crate::W<PADCFG_SD0_DATA2_SPEC>;
#[doc = "Field `ie` reader - Input Enable (IE) Controller - 1: Enable the receiver, 0: Disable the receiver"]
pub type IE_R = crate::BitReader;
#[doc = "Field `ie` writer - Input Enable (IE) Controller - 1: Enable the receiver, 0: Disable the receiver"]
pub type IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ds` reader - Output Drive Strength (DS) - 00: The rated drive strength is 2 mA, 01: The rated drive strength is 4 mA, 10: The rated drive strength is 8 mA, 11: The rated drive strength is 12 mA"]
pub type DS_R = crate::FieldReader;
#[doc = "Field `ds` writer - Output Drive Strength (DS) - 00: The rated drive strength is 2 mA, 01: The rated drive strength is 4 mA, 10: The rated drive strength is 8 mA, 11: The rated drive strength is 12 mA"]
pub type DS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `pu` reader - Pull-Up (PU) settings - 1: Yes, 0: No"]
pub type PU_R = crate::BitReader;
#[doc = "Field `pu` writer - Pull-Up (PU) settings - 1: Yes, 0: No"]
pub type PU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `pd` reader - Pull-Down (PD) settings - 1: Yes, 0: No"]
pub type PD_R = crate::BitReader;
#[doc = "Field `pd` writer - Pull-Down (PD) settings - 1: Yes, 0: No"]
pub type PD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `slew` reader - Slew Rate Control - 0: Slow (Half frequency), 1: Fast"]
pub type SLEW_R = crate::BitReader;
#[doc = "Field `slew` writer - Slew Rate Control - 0: Slow (Half frequency), 1: Fast"]
pub type SLEW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `smt` reader - Active high Schmitt (SMT) trigger selector - 0: No hysteresis, 1: Schmitt trigger ebabled"]
pub type SMT_R = crate::BitReader;
#[doc = "Field `smt` writer - Active high Schmitt (SMT) trigger selector - 0: No hysteresis, 1: Schmitt trigger ebabled"]
pub type SMT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `pos` reader - Power-on-Start (POS) enabler - 1: Enable active pull down for loss of core power, 0: Active pull-down capability disabled"]
pub type POS_R = crate::BitReader;
#[doc = "Field `pos` writer - Power-on-Start (POS) enabler - 1: Enable active pull down for loss of core power, 0: Active pull-down capability disabled"]
pub type POS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Input Enable (IE) Controller - 1: Enable the receiver, 0: Disable the receiver"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Output Drive Strength (DS) - 00: The rated drive strength is 2 mA, 01: The rated drive strength is 4 mA, 10: The rated drive strength is 8 mA, 11: The rated drive strength is 12 mA"]
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Pull-Up (PU) settings - 1: Yes, 0: No"]
    #[inline(always)]
    pub fn pu(&self) -> PU_R {
        PU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pull-Down (PD) settings - 1: Yes, 0: No"]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Slew Rate Control - 0: Slow (Half frequency), 1: Fast"]
    #[inline(always)]
    pub fn slew(&self) -> SLEW_R {
        SLEW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Active high Schmitt (SMT) trigger selector - 0: No hysteresis, 1: Schmitt trigger ebabled"]
    #[inline(always)]
    pub fn smt(&self) -> SMT_R {
        SMT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Power-on-Start (POS) enabler - 1: Enable active pull down for loss of core power, 0: Active pull-down capability disabled"]
    #[inline(always)]
    pub fn pos(&self) -> POS_R {
        POS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Input Enable (IE) Controller - 1: Enable the receiver, 0: Disable the receiver"]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IE_W<PADCFG_SD0_DATA2_SPEC, 0> {
        IE_W::new(self)
    }
    #[doc = "Bits 1:2 - Output Drive Strength (DS) - 00: The rated drive strength is 2 mA, 01: The rated drive strength is 4 mA, 10: The rated drive strength is 8 mA, 11: The rated drive strength is 12 mA"]
    #[inline(always)]
    #[must_use]
    pub fn ds(&mut self) -> DS_W<PADCFG_SD0_DATA2_SPEC, 1> {
        DS_W::new(self)
    }
    #[doc = "Bit 3 - Pull-Up (PU) settings - 1: Yes, 0: No"]
    #[inline(always)]
    #[must_use]
    pub fn pu(&mut self) -> PU_W<PADCFG_SD0_DATA2_SPEC, 3> {
        PU_W::new(self)
    }
    #[doc = "Bit 4 - Pull-Down (PD) settings - 1: Yes, 0: No"]
    #[inline(always)]
    #[must_use]
    pub fn pd(&mut self) -> PD_W<PADCFG_SD0_DATA2_SPEC, 4> {
        PD_W::new(self)
    }
    #[doc = "Bit 5 - Slew Rate Control - 0: Slow (Half frequency), 1: Fast"]
    #[inline(always)]
    #[must_use]
    pub fn slew(&mut self) -> SLEW_W<PADCFG_SD0_DATA2_SPEC, 5> {
        SLEW_W::new(self)
    }
    #[doc = "Bit 6 - Active high Schmitt (SMT) trigger selector - 0: No hysteresis, 1: Schmitt trigger ebabled"]
    #[inline(always)]
    #[must_use]
    pub fn smt(&mut self) -> SMT_W<PADCFG_SD0_DATA2_SPEC, 6> {
        SMT_W::new(self)
    }
    #[doc = "Bit 7 - Power-on-Start (POS) enabler - 1: Enable active pull down for loss of core power, 0: Active pull-down capability disabled"]
    #[inline(always)]
    #[must_use]
    pub fn pos(&mut self) -> POS_W<PADCFG_SD0_DATA2_SPEC, 7> {
        POS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 584: SD0_DATA2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_sd0_data2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_sd0_data2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PADCFG_SD0_DATA2_SPEC;
impl crate::RegisterSpec for PADCFG_SD0_DATA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padcfg_sd0_data2::R`](R) reader structure"]
impl crate::Readable for PADCFG_SD0_DATA2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`padcfg_sd0_data2::W`](W) writer structure"]
impl crate::Writable for PADCFG_SD0_DATA2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets padcfg_sd0_data2 to value 0x01"]
impl crate::Resettable for PADCFG_SD0_DATA2_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}