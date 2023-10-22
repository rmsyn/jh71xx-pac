#[doc = "Register `aon_iomux_cfgsaif_syscfg60` reader"]
pub type R = crate::R<AON_IOMUX_CFGSAIF_SYSCFG60_SPEC>;
#[doc = "Register `aon_iomux_cfgsaif_syscfg60` writer"]
pub type W = crate::W<AON_IOMUX_CFGSAIF_SYSCFG60_SPEC>;
#[doc = "Field `padcfg_pad_rgpio2_ie` reader - Input Enable (IE) Controller - 1: Enable the receiver, 0: Disable the receiver"]
pub type PADCFG_PAD_RGPIO2_IE_R = crate::BitReader;
#[doc = "Field `padcfg_pad_rgpio2_ie` writer - Input Enable (IE) Controller - 1: Enable the receiver, 0: Disable the receiver"]
pub type PADCFG_PAD_RGPIO2_IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `padcfg_pad_rgpio2_ds` reader - Output Drive Strength (DS) - 00: The rated drive strength is 2 mA, 01: The rated drive strength is 4 mA, 10: The rated drive strength is 8 mA, 11: The rated drive strength is 12 mA"]
pub type PADCFG_PAD_RGPIO2_DS_R = crate::FieldReader;
#[doc = "Field `padcfg_pad_rgpio2_ds` writer - Output Drive Strength (DS) - 00: The rated drive strength is 2 mA, 01: The rated drive strength is 4 mA, 10: The rated drive strength is 8 mA, 11: The rated drive strength is 12 mA"]
pub type PADCFG_PAD_RGPIO2_DS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `padcfg_pad_rgpio2_pu` reader - Pull-Up (PU) settings - 1: Yes, 0: No"]
pub type PADCFG_PAD_RGPIO2_PU_R = crate::BitReader;
#[doc = "Field `padcfg_pad_rgpio2_pu` writer - Pull-Up (PU) settings - 1: Yes, 0: No"]
pub type PADCFG_PAD_RGPIO2_PU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `padcfg_pad_rgpio2_pd` reader - Pull-Down (PD) settings - 1: Yes, 0: No"]
pub type PADCFG_PAD_RGPIO2_PD_R = crate::BitReader;
#[doc = "Field `padcfg_pad_rgpio2_pd` writer - Pull-Down (PD) settings - 1: Yes, 0: No"]
pub type PADCFG_PAD_RGPIO2_PD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `padcfg_pad_rgpio2_slew` reader - Slew Rate Control - 0: Slow (Half frequency), 1: Fast"]
pub type PADCFG_PAD_RGPIO2_SLEW_R = crate::BitReader;
#[doc = "Field `padcfg_pad_rgpio2_slew` writer - Slew Rate Control - 0: Slow (Half frequency), 1: Fast"]
pub type PADCFG_PAD_RGPIO2_SLEW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `padcfg_pad_rgpio2_smt` reader - Active high Schmitt (SMT) trigger selector - 0: No hysteresis, 1: Schmitt trigger ebabled"]
pub type PADCFG_PAD_RGPIO2_SMT_R = crate::BitReader;
#[doc = "Field `padcfg_pad_rgpio2_smt` writer - Active high Schmitt (SMT) trigger selector - 0: No hysteresis, 1: Schmitt trigger ebabled"]
pub type PADCFG_PAD_RGPIO2_SMT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `padcfg_pad_rgpio2_pos` reader - Power-on-Start (POS) enabler - 1: Enable active pull down for loss of core power, 0: Active pull-down capability disabled"]
pub type PADCFG_PAD_RGPIO2_POS_R = crate::BitReader;
#[doc = "Field `padcfg_pad_rgpio2_pos` writer - Power-on-Start (POS) enabler - 1: Enable active pull down for loss of core power, 0: Active pull-down capability disabled"]
pub type PADCFG_PAD_RGPIO2_POS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Input Enable (IE) Controller - 1: Enable the receiver, 0: Disable the receiver"]
    #[inline(always)]
    pub fn padcfg_pad_rgpio2_ie(&self) -> PADCFG_PAD_RGPIO2_IE_R {
        PADCFG_PAD_RGPIO2_IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Output Drive Strength (DS) - 00: The rated drive strength is 2 mA, 01: The rated drive strength is 4 mA, 10: The rated drive strength is 8 mA, 11: The rated drive strength is 12 mA"]
    #[inline(always)]
    pub fn padcfg_pad_rgpio2_ds(&self) -> PADCFG_PAD_RGPIO2_DS_R {
        PADCFG_PAD_RGPIO2_DS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Pull-Up (PU) settings - 1: Yes, 0: No"]
    #[inline(always)]
    pub fn padcfg_pad_rgpio2_pu(&self) -> PADCFG_PAD_RGPIO2_PU_R {
        PADCFG_PAD_RGPIO2_PU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pull-Down (PD) settings - 1: Yes, 0: No"]
    #[inline(always)]
    pub fn padcfg_pad_rgpio2_pd(&self) -> PADCFG_PAD_RGPIO2_PD_R {
        PADCFG_PAD_RGPIO2_PD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Slew Rate Control - 0: Slow (Half frequency), 1: Fast"]
    #[inline(always)]
    pub fn padcfg_pad_rgpio2_slew(&self) -> PADCFG_PAD_RGPIO2_SLEW_R {
        PADCFG_PAD_RGPIO2_SLEW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Active high Schmitt (SMT) trigger selector - 0: No hysteresis, 1: Schmitt trigger ebabled"]
    #[inline(always)]
    pub fn padcfg_pad_rgpio2_smt(&self) -> PADCFG_PAD_RGPIO2_SMT_R {
        PADCFG_PAD_RGPIO2_SMT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Power-on-Start (POS) enabler - 1: Enable active pull down for loss of core power, 0: Active pull-down capability disabled"]
    #[inline(always)]
    pub fn padcfg_pad_rgpio2_pos(&self) -> PADCFG_PAD_RGPIO2_POS_R {
        PADCFG_PAD_RGPIO2_POS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Input Enable (IE) Controller - 1: Enable the receiver, 0: Disable the receiver"]
    #[inline(always)]
    #[must_use]
    pub fn padcfg_pad_rgpio2_ie(
        &mut self,
    ) -> PADCFG_PAD_RGPIO2_IE_W<AON_IOMUX_CFGSAIF_SYSCFG60_SPEC, 0> {
        PADCFG_PAD_RGPIO2_IE_W::new(self)
    }
    #[doc = "Bits 1:2 - Output Drive Strength (DS) - 00: The rated drive strength is 2 mA, 01: The rated drive strength is 4 mA, 10: The rated drive strength is 8 mA, 11: The rated drive strength is 12 mA"]
    #[inline(always)]
    #[must_use]
    pub fn padcfg_pad_rgpio2_ds(
        &mut self,
    ) -> PADCFG_PAD_RGPIO2_DS_W<AON_IOMUX_CFGSAIF_SYSCFG60_SPEC, 1> {
        PADCFG_PAD_RGPIO2_DS_W::new(self)
    }
    #[doc = "Bit 3 - Pull-Up (PU) settings - 1: Yes, 0: No"]
    #[inline(always)]
    #[must_use]
    pub fn padcfg_pad_rgpio2_pu(
        &mut self,
    ) -> PADCFG_PAD_RGPIO2_PU_W<AON_IOMUX_CFGSAIF_SYSCFG60_SPEC, 3> {
        PADCFG_PAD_RGPIO2_PU_W::new(self)
    }
    #[doc = "Bit 4 - Pull-Down (PD) settings - 1: Yes, 0: No"]
    #[inline(always)]
    #[must_use]
    pub fn padcfg_pad_rgpio2_pd(
        &mut self,
    ) -> PADCFG_PAD_RGPIO2_PD_W<AON_IOMUX_CFGSAIF_SYSCFG60_SPEC, 4> {
        PADCFG_PAD_RGPIO2_PD_W::new(self)
    }
    #[doc = "Bit 5 - Slew Rate Control - 0: Slow (Half frequency), 1: Fast"]
    #[inline(always)]
    #[must_use]
    pub fn padcfg_pad_rgpio2_slew(
        &mut self,
    ) -> PADCFG_PAD_RGPIO2_SLEW_W<AON_IOMUX_CFGSAIF_SYSCFG60_SPEC, 5> {
        PADCFG_PAD_RGPIO2_SLEW_W::new(self)
    }
    #[doc = "Bit 6 - Active high Schmitt (SMT) trigger selector - 0: No hysteresis, 1: Schmitt trigger ebabled"]
    #[inline(always)]
    #[must_use]
    pub fn padcfg_pad_rgpio2_smt(
        &mut self,
    ) -> PADCFG_PAD_RGPIO2_SMT_W<AON_IOMUX_CFGSAIF_SYSCFG60_SPEC, 6> {
        PADCFG_PAD_RGPIO2_SMT_W::new(self)
    }
    #[doc = "Bit 7 - Power-on-Start (POS) enabler - 1: Enable active pull down for loss of core power, 0: Active pull-down capability disabled"]
    #[inline(always)]
    #[must_use]
    pub fn padcfg_pad_rgpio2_pos(
        &mut self,
    ) -> PADCFG_PAD_RGPIO2_POS_W<AON_IOMUX_CFGSAIF_SYSCFG60_SPEC, 7> {
        PADCFG_PAD_RGPIO2_POS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AON IOMUX CFG SAIF SYSCFG 60\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg60::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg60::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AON_IOMUX_CFGSAIF_SYSCFG60_SPEC;
impl crate::RegisterSpec for AON_IOMUX_CFGSAIF_SYSCFG60_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_iomux_cfgsaif_syscfg60::R`](R) reader structure"]
impl crate::Readable for AON_IOMUX_CFGSAIF_SYSCFG60_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aon_iomux_cfgsaif_syscfg60::W`](W) writer structure"]
impl crate::Writable for AON_IOMUX_CFGSAIF_SYSCFG60_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}