#[doc = "Register `aon_iomux_cfgsaif_syscfg68` reader"]
pub type R = crate::R<AON_IOMUX_CFGSAIF_SYSCFG68_SPEC>;
#[doc = "Register `aon_iomux_cfgsaif_syscfg68` writer"]
pub type W = crate::W<AON_IOMUX_CFGSAIF_SYSCFG68_SPEC>;
#[doc = "Field `padcfg_pad_rstn_smt` reader - Active high Schmitt (SMT) trigger selector - 0: No hysteresis, 1: Schmitt trigger enabled"]
pub type PADCFG_PAD_RSTN_SMT_R = crate::BitReader;
#[doc = "Field `padcfg_pad_rstn_smt` writer - Active high Schmitt (SMT) trigger selector - 0: No hysteresis, 1: Schmitt trigger enabled"]
pub type PADCFG_PAD_RSTN_SMT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `padcfg_pad_rstn_pos` reader - Power-on-Start (POS) enabler - 1: Enable active pull-down for loss of core power, 0: Active pull-down capability disabled"]
pub type PADCFG_PAD_RSTN_POS_R = crate::BitReader;
#[doc = "Field `padcfg_pad_rstn_pos` writer - Power-on-Start (POS) enabler - 1: Enable active pull-down for loss of core power, 0: Active pull-down capability disabled"]
pub type PADCFG_PAD_RSTN_POS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Active high Schmitt (SMT) trigger selector - 0: No hysteresis, 1: Schmitt trigger enabled"]
    #[inline(always)]
    pub fn padcfg_pad_rstn_smt(&self) -> PADCFG_PAD_RSTN_SMT_R {
        PADCFG_PAD_RSTN_SMT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power-on-Start (POS) enabler - 1: Enable active pull-down for loss of core power, 0: Active pull-down capability disabled"]
    #[inline(always)]
    pub fn padcfg_pad_rstn_pos(&self) -> PADCFG_PAD_RSTN_POS_R {
        PADCFG_PAD_RSTN_POS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Active high Schmitt (SMT) trigger selector - 0: No hysteresis, 1: Schmitt trigger enabled"]
    #[inline(always)]
    #[must_use]
    pub fn padcfg_pad_rstn_smt(
        &mut self,
    ) -> PADCFG_PAD_RSTN_SMT_W<AON_IOMUX_CFGSAIF_SYSCFG68_SPEC> {
        PADCFG_PAD_RSTN_SMT_W::new(self, 0)
    }
    #[doc = "Bit 1 - Power-on-Start (POS) enabler - 1: Enable active pull-down for loss of core power, 0: Active pull-down capability disabled"]
    #[inline(always)]
    #[must_use]
    pub fn padcfg_pad_rstn_pos(
        &mut self,
    ) -> PADCFG_PAD_RSTN_POS_W<AON_IOMUX_CFGSAIF_SYSCFG68_SPEC> {
        PADCFG_PAD_RSTN_POS_W::new(self, 1)
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
#[doc = "AON IOMUX CFG SAIF SYSCFG 68\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg68::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg68::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AON_IOMUX_CFGSAIF_SYSCFG68_SPEC;
impl crate::RegisterSpec for AON_IOMUX_CFGSAIF_SYSCFG68_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_iomux_cfgsaif_syscfg68::R`](R) reader structure"]
impl crate::Readable for AON_IOMUX_CFGSAIF_SYSCFG68_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aon_iomux_cfgsaif_syscfg68::W`](W) writer structure"]
impl crate::Writable for AON_IOMUX_CFGSAIF_SYSCFG68_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
