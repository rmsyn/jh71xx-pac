#[doc = "Register `aon_iomux_cfgsaif_syscfg52` reader"]
pub type R = crate::R<AON_IOMUX_CFGSAIF_SYSCFG52_SPEC>;
#[doc = "Register `aon_iomux_cfgsaif_syscfg52` writer"]
pub type W = crate::W<AON_IOMUX_CFGSAIF_SYSCFG52_SPEC>;
#[doc = "Field `padcfg_pad_rgpio0_ie` reader - Input Enable (IE) Controller - 1: Enable the receiver, 0: Disable the receiver"]
pub type PADCFG_PAD_RGPIO0_IE_R = crate::BitReader;
#[doc = "Field `padcfg_pad_rgpio0_ie` writer - Input Enable (IE) Controller - 1: Enable the receiver, 0: Disable the receiver"]
pub type PADCFG_PAD_RGPIO0_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `padcfg_pad_rgpio0_ds` reader - Output Drive Strength (DS) - 00: The rated drive strength is 2 mA, 01: The rated drive strength is 4 mA, 10: The rated drive strength is 8 mA, 11: The rated drive strength is 12 mA"]
pub type PADCFG_PAD_RGPIO0_DS_R = crate::FieldReader;
#[doc = "Field `padcfg_pad_rgpio0_ds` writer - Output Drive Strength (DS) - 00: The rated drive strength is 2 mA, 01: The rated drive strength is 4 mA, 10: The rated drive strength is 8 mA, 11: The rated drive strength is 12 mA"]
pub type PADCFG_PAD_RGPIO0_DS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `padcfg_pad_rgpio0_pu` reader - Pull-Up (PU) settings - 1: Yes, 0: No"]
pub type PADCFG_PAD_RGPIO0_PU_R = crate::BitReader;
#[doc = "Field `padcfg_pad_rgpio0_pu` writer - Pull-Up (PU) settings - 1: Yes, 0: No"]
pub type PADCFG_PAD_RGPIO0_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `padcfg_pad_rgpio0_pd` reader - Pull-Down (PD) settings - 1: Yes, 0: No"]
pub type PADCFG_PAD_RGPIO0_PD_R = crate::BitReader;
#[doc = "Field `padcfg_pad_rgpio0_pd` writer - Pull-Down (PD) settings - 1: Yes, 0: No"]
pub type PADCFG_PAD_RGPIO0_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `padcfg_pad_rgpio0_slew` reader - Slew Rate Control - 0: Slow (Half frequency), 1: Fast"]
pub type PADCFG_PAD_RGPIO0_SLEW_R = crate::BitReader;
#[doc = "Field `padcfg_pad_rgpio0_slew` writer - Slew Rate Control - 0: Slow (Half frequency), 1: Fast"]
pub type PADCFG_PAD_RGPIO0_SLEW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `padcfg_pad_rgpio0_smt` reader - Active high Schmitt (SMT) trigger selector - 0: No hysteresis, 1: Schmitt trigger ebabled"]
pub type PADCFG_PAD_RGPIO0_SMT_R = crate::BitReader;
#[doc = "Field `padcfg_pad_rgpio0_smt` writer - Active high Schmitt (SMT) trigger selector - 0: No hysteresis, 1: Schmitt trigger ebabled"]
pub type PADCFG_PAD_RGPIO0_SMT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `padcfg_pad_rgpio0_pos` reader - Power-on-Start (POS) enabler - 1: Enable active pull down for loss of core power, 0: Active pull-down capability disabled"]
pub type PADCFG_PAD_RGPIO0_POS_R = crate::BitReader;
#[doc = "Field `padcfg_pad_rgpio0_pos` writer - Power-on-Start (POS) enabler - 1: Enable active pull down for loss of core power, 0: Active pull-down capability disabled"]
pub type PADCFG_PAD_RGPIO0_POS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Input Enable (IE) Controller - 1: Enable the receiver, 0: Disable the receiver"]
    #[inline(always)]
    pub fn padcfg_pad_rgpio0_ie(&self) -> PADCFG_PAD_RGPIO0_IE_R {
        PADCFG_PAD_RGPIO0_IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Output Drive Strength (DS) - 00: The rated drive strength is 2 mA, 01: The rated drive strength is 4 mA, 10: The rated drive strength is 8 mA, 11: The rated drive strength is 12 mA"]
    #[inline(always)]
    pub fn padcfg_pad_rgpio0_ds(&self) -> PADCFG_PAD_RGPIO0_DS_R {
        PADCFG_PAD_RGPIO0_DS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Pull-Up (PU) settings - 1: Yes, 0: No"]
    #[inline(always)]
    pub fn padcfg_pad_rgpio0_pu(&self) -> PADCFG_PAD_RGPIO0_PU_R {
        PADCFG_PAD_RGPIO0_PU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pull-Down (PD) settings - 1: Yes, 0: No"]
    #[inline(always)]
    pub fn padcfg_pad_rgpio0_pd(&self) -> PADCFG_PAD_RGPIO0_PD_R {
        PADCFG_PAD_RGPIO0_PD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Slew Rate Control - 0: Slow (Half frequency), 1: Fast"]
    #[inline(always)]
    pub fn padcfg_pad_rgpio0_slew(&self) -> PADCFG_PAD_RGPIO0_SLEW_R {
        PADCFG_PAD_RGPIO0_SLEW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Active high Schmitt (SMT) trigger selector - 0: No hysteresis, 1: Schmitt trigger ebabled"]
    #[inline(always)]
    pub fn padcfg_pad_rgpio0_smt(&self) -> PADCFG_PAD_RGPIO0_SMT_R {
        PADCFG_PAD_RGPIO0_SMT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Power-on-Start (POS) enabler - 1: Enable active pull down for loss of core power, 0: Active pull-down capability disabled"]
    #[inline(always)]
    pub fn padcfg_pad_rgpio0_pos(&self) -> PADCFG_PAD_RGPIO0_POS_R {
        PADCFG_PAD_RGPIO0_POS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Input Enable (IE) Controller - 1: Enable the receiver, 0: Disable the receiver"]
    #[inline(always)]
    #[must_use]
    pub fn padcfg_pad_rgpio0_ie(
        &mut self,
    ) -> PADCFG_PAD_RGPIO0_IE_W<AON_IOMUX_CFGSAIF_SYSCFG52_SPEC> {
        PADCFG_PAD_RGPIO0_IE_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Output Drive Strength (DS) - 00: The rated drive strength is 2 mA, 01: The rated drive strength is 4 mA, 10: The rated drive strength is 8 mA, 11: The rated drive strength is 12 mA"]
    #[inline(always)]
    #[must_use]
    pub fn padcfg_pad_rgpio0_ds(
        &mut self,
    ) -> PADCFG_PAD_RGPIO0_DS_W<AON_IOMUX_CFGSAIF_SYSCFG52_SPEC> {
        PADCFG_PAD_RGPIO0_DS_W::new(self, 1)
    }
    #[doc = "Bit 3 - Pull-Up (PU) settings - 1: Yes, 0: No"]
    #[inline(always)]
    #[must_use]
    pub fn padcfg_pad_rgpio0_pu(
        &mut self,
    ) -> PADCFG_PAD_RGPIO0_PU_W<AON_IOMUX_CFGSAIF_SYSCFG52_SPEC> {
        PADCFG_PAD_RGPIO0_PU_W::new(self, 3)
    }
    #[doc = "Bit 4 - Pull-Down (PD) settings - 1: Yes, 0: No"]
    #[inline(always)]
    #[must_use]
    pub fn padcfg_pad_rgpio0_pd(
        &mut self,
    ) -> PADCFG_PAD_RGPIO0_PD_W<AON_IOMUX_CFGSAIF_SYSCFG52_SPEC> {
        PADCFG_PAD_RGPIO0_PD_W::new(self, 4)
    }
    #[doc = "Bit 5 - Slew Rate Control - 0: Slow (Half frequency), 1: Fast"]
    #[inline(always)]
    #[must_use]
    pub fn padcfg_pad_rgpio0_slew(
        &mut self,
    ) -> PADCFG_PAD_RGPIO0_SLEW_W<AON_IOMUX_CFGSAIF_SYSCFG52_SPEC> {
        PADCFG_PAD_RGPIO0_SLEW_W::new(self, 5)
    }
    #[doc = "Bit 6 - Active high Schmitt (SMT) trigger selector - 0: No hysteresis, 1: Schmitt trigger ebabled"]
    #[inline(always)]
    #[must_use]
    pub fn padcfg_pad_rgpio0_smt(
        &mut self,
    ) -> PADCFG_PAD_RGPIO0_SMT_W<AON_IOMUX_CFGSAIF_SYSCFG52_SPEC> {
        PADCFG_PAD_RGPIO0_SMT_W::new(self, 6)
    }
    #[doc = "Bit 7 - Power-on-Start (POS) enabler - 1: Enable active pull down for loss of core power, 0: Active pull-down capability disabled"]
    #[inline(always)]
    #[must_use]
    pub fn padcfg_pad_rgpio0_pos(
        &mut self,
    ) -> PADCFG_PAD_RGPIO0_POS_W<AON_IOMUX_CFGSAIF_SYSCFG52_SPEC> {
        PADCFG_PAD_RGPIO0_POS_W::new(self, 7)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AON IOMUX CFG SAIF SYSCFG 52\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg52::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg52::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AON_IOMUX_CFGSAIF_SYSCFG52_SPEC;
impl crate::RegisterSpec for AON_IOMUX_CFGSAIF_SYSCFG52_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_iomux_cfgsaif_syscfg52::R`](R) reader structure"]
impl crate::Readable for AON_IOMUX_CFGSAIF_SYSCFG52_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aon_iomux_cfgsaif_syscfg52::W`](W) writer structure"]
impl crate::Writable for AON_IOMUX_CFGSAIF_SYSCFG52_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
