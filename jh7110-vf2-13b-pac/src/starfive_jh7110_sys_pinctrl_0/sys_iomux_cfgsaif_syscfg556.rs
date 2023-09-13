#[doc = "Register `sys_iomux_cfgsaif_syscfg556` reader"]
pub type R = crate::R<SYS_IOMUX_CFGSAIF_SYSCFG556_SPEC>;
#[doc = "Register `sys_iomux_cfgsaif_syscfg556` writer"]
pub type W = crate::W<SYS_IOMUX_CFGSAIF_SYSCFG556_SPEC>;
#[doc = "Field `padcfg_pad_sd0_data1_ie` reader - Input Enable (IE) Controller - 1: Enable the receiver, 0: Disable the receiver"]
pub type PADCFG_PAD_SD0_DATA1_IE_R = crate::BitReader;
#[doc = "Field `padcfg_pad_sd0_data1_ie` writer - Input Enable (IE) Controller - 1: Enable the receiver, 0: Disable the receiver"]
pub type PADCFG_PAD_SD0_DATA1_IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `padcfg_pad_sd0_data1_ds` reader - Output Drive Strength (DS) - 00: The rated drive strength is 2 mA, 01: The rated drive strength is 4 mA, 10: The rated drive strength is 8 mA, 11: The rated drive strength is 12 mA"]
pub type PADCFG_PAD_SD0_DATA1_DS_R = crate::FieldReader;
#[doc = "Field `padcfg_pad_sd0_data1_ds` writer - Output Drive Strength (DS) - 00: The rated drive strength is 2 mA, 01: The rated drive strength is 4 mA, 10: The rated drive strength is 8 mA, 11: The rated drive strength is 12 mA"]
pub type PADCFG_PAD_SD0_DATA1_DS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `padcfg_pad_sd0_data1_pu` reader - Pull-Up (PU) settings - 1: Yes, 0: No"]
pub type PADCFG_PAD_SD0_DATA1_PU_R = crate::BitReader;
#[doc = "Field `padcfg_pad_sd0_data1_pu` writer - Pull-Up (PU) settings - 1: Yes, 0: No"]
pub type PADCFG_PAD_SD0_DATA1_PU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `padcfg_pad_sd0_data1_pd` reader - Pull-Down (PD) settings - 1: Yes, 0: No"]
pub type PADCFG_PAD_SD0_DATA1_PD_R = crate::BitReader;
#[doc = "Field `padcfg_pad_sd0_data1_pd` writer - Pull-Down (PD) settings - 1: Yes, 0: No"]
pub type PADCFG_PAD_SD0_DATA1_PD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `padcfg_pad_sd0_data1_slew` reader - Slew Rate Control - 0: Slow (Half frequency), 1: Fast"]
pub type PADCFG_PAD_SD0_DATA1_SLEW_R = crate::BitReader;
#[doc = "Field `padcfg_pad_sd0_data1_slew` writer - Slew Rate Control - 0: Slow (Half frequency), 1: Fast"]
pub type PADCFG_PAD_SD0_DATA1_SLEW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `padcfg_pad_sd0_data1_smt` reader - Active high Schmitt (SMT) trigger selector - 0: No hysteresis, 1: Schmitt trigger ebabled"]
pub type PADCFG_PAD_SD0_DATA1_SMT_R = crate::BitReader;
#[doc = "Field `padcfg_pad_sd0_data1_smt` writer - Active high Schmitt (SMT) trigger selector - 0: No hysteresis, 1: Schmitt trigger ebabled"]
pub type PADCFG_PAD_SD0_DATA1_SMT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `padcfg_pad_sd0_data1_pos` reader - Power-on-Start (POS) enabler - 1: Enable active pull down for loss of core power, 0: Active pull-down capability disabled"]
pub type PADCFG_PAD_SD0_DATA1_POS_R = crate::BitReader;
#[doc = "Field `padcfg_pad_sd0_data1_pos` writer - Power-on-Start (POS) enabler - 1: Enable active pull down for loss of core power, 0: Active pull-down capability disabled"]
pub type PADCFG_PAD_SD0_DATA1_POS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Input Enable (IE) Controller - 1: Enable the receiver, 0: Disable the receiver"]
    #[inline(always)]
    pub fn padcfg_pad_sd0_data1_ie(&self) -> PADCFG_PAD_SD0_DATA1_IE_R {
        PADCFG_PAD_SD0_DATA1_IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Output Drive Strength (DS) - 00: The rated drive strength is 2 mA, 01: The rated drive strength is 4 mA, 10: The rated drive strength is 8 mA, 11: The rated drive strength is 12 mA"]
    #[inline(always)]
    pub fn padcfg_pad_sd0_data1_ds(&self) -> PADCFG_PAD_SD0_DATA1_DS_R {
        PADCFG_PAD_SD0_DATA1_DS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Pull-Up (PU) settings - 1: Yes, 0: No"]
    #[inline(always)]
    pub fn padcfg_pad_sd0_data1_pu(&self) -> PADCFG_PAD_SD0_DATA1_PU_R {
        PADCFG_PAD_SD0_DATA1_PU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pull-Down (PD) settings - 1: Yes, 0: No"]
    #[inline(always)]
    pub fn padcfg_pad_sd0_data1_pd(&self) -> PADCFG_PAD_SD0_DATA1_PD_R {
        PADCFG_PAD_SD0_DATA1_PD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Slew Rate Control - 0: Slow (Half frequency), 1: Fast"]
    #[inline(always)]
    pub fn padcfg_pad_sd0_data1_slew(&self) -> PADCFG_PAD_SD0_DATA1_SLEW_R {
        PADCFG_PAD_SD0_DATA1_SLEW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Active high Schmitt (SMT) trigger selector - 0: No hysteresis, 1: Schmitt trigger ebabled"]
    #[inline(always)]
    pub fn padcfg_pad_sd0_data1_smt(&self) -> PADCFG_PAD_SD0_DATA1_SMT_R {
        PADCFG_PAD_SD0_DATA1_SMT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Power-on-Start (POS) enabler - 1: Enable active pull down for loss of core power, 0: Active pull-down capability disabled"]
    #[inline(always)]
    pub fn padcfg_pad_sd0_data1_pos(&self) -> PADCFG_PAD_SD0_DATA1_POS_R {
        PADCFG_PAD_SD0_DATA1_POS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Input Enable (IE) Controller - 1: Enable the receiver, 0: Disable the receiver"]
    #[inline(always)]
    #[must_use]
    pub fn padcfg_pad_sd0_data1_ie(
        &mut self,
    ) -> PADCFG_PAD_SD0_DATA1_IE_W<SYS_IOMUX_CFGSAIF_SYSCFG556_SPEC, 0> {
        PADCFG_PAD_SD0_DATA1_IE_W::new(self)
    }
    #[doc = "Bits 1:2 - Output Drive Strength (DS) - 00: The rated drive strength is 2 mA, 01: The rated drive strength is 4 mA, 10: The rated drive strength is 8 mA, 11: The rated drive strength is 12 mA"]
    #[inline(always)]
    #[must_use]
    pub fn padcfg_pad_sd0_data1_ds(
        &mut self,
    ) -> PADCFG_PAD_SD0_DATA1_DS_W<SYS_IOMUX_CFGSAIF_SYSCFG556_SPEC, 1> {
        PADCFG_PAD_SD0_DATA1_DS_W::new(self)
    }
    #[doc = "Bit 3 - Pull-Up (PU) settings - 1: Yes, 0: No"]
    #[inline(always)]
    #[must_use]
    pub fn padcfg_pad_sd0_data1_pu(
        &mut self,
    ) -> PADCFG_PAD_SD0_DATA1_PU_W<SYS_IOMUX_CFGSAIF_SYSCFG556_SPEC, 3> {
        PADCFG_PAD_SD0_DATA1_PU_W::new(self)
    }
    #[doc = "Bit 4 - Pull-Down (PD) settings - 1: Yes, 0: No"]
    #[inline(always)]
    #[must_use]
    pub fn padcfg_pad_sd0_data1_pd(
        &mut self,
    ) -> PADCFG_PAD_SD0_DATA1_PD_W<SYS_IOMUX_CFGSAIF_SYSCFG556_SPEC, 4> {
        PADCFG_PAD_SD0_DATA1_PD_W::new(self)
    }
    #[doc = "Bit 5 - Slew Rate Control - 0: Slow (Half frequency), 1: Fast"]
    #[inline(always)]
    #[must_use]
    pub fn padcfg_pad_sd0_data1_slew(
        &mut self,
    ) -> PADCFG_PAD_SD0_DATA1_SLEW_W<SYS_IOMUX_CFGSAIF_SYSCFG556_SPEC, 5> {
        PADCFG_PAD_SD0_DATA1_SLEW_W::new(self)
    }
    #[doc = "Bit 6 - Active high Schmitt (SMT) trigger selector - 0: No hysteresis, 1: Schmitt trigger ebabled"]
    #[inline(always)]
    #[must_use]
    pub fn padcfg_pad_sd0_data1_smt(
        &mut self,
    ) -> PADCFG_PAD_SD0_DATA1_SMT_W<SYS_IOMUX_CFGSAIF_SYSCFG556_SPEC, 6> {
        PADCFG_PAD_SD0_DATA1_SMT_W::new(self)
    }
    #[doc = "Bit 7 - Power-on-Start (POS) enabler - 1: Enable active pull down for loss of core power, 0: Active pull-down capability disabled"]
    #[inline(always)]
    #[must_use]
    pub fn padcfg_pad_sd0_data1_pos(
        &mut self,
    ) -> PADCFG_PAD_SD0_DATA1_POS_W<SYS_IOMUX_CFGSAIF_SYSCFG556_SPEC, 7> {
        PADCFG_PAD_SD0_DATA1_POS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG 556\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_iomux_cfgsaif_syscfg556::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_iomux_cfgsaif_syscfg556::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_IOMUX_CFGSAIF_SYSCFG556_SPEC;
impl crate::RegisterSpec for SYS_IOMUX_CFGSAIF_SYSCFG556_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_iomux_cfgsaif_syscfg556::R`](R) reader structure"]
impl crate::Readable for SYS_IOMUX_CFGSAIF_SYSCFG556_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_iomux_cfgsaif_syscfg556::W`](W) writer structure"]
impl crate::Writable for SYS_IOMUX_CFGSAIF_SYSCFG556_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
