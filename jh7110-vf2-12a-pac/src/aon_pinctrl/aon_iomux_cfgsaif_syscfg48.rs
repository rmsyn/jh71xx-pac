#[doc = "Register `aon_iomux_cfgsaif_syscfg48` reader"]
pub type R = crate::R<AON_IOMUX_CFGSAIF_SYSCFG48_SPEC>;
#[doc = "Register `aon_iomux_cfgsaif_syscfg48` writer"]
pub type W = crate::W<AON_IOMUX_CFGSAIF_SYSCFG48_SPEC>;
#[doc = "Field `padcfg_pad_testen_pos` reader - Power-on-Start (POS) enabler - 1: Enable active pull down for loss of core power, 0: Active pull-down capability disabled"]
pub type PADCFG_PAD_TESTEN_POS_R = crate::BitReader;
#[doc = "Field `padcfg_pad_testen_pos` writer - Power-on-Start (POS) enabler - 1: Enable active pull down for loss of core power, 0: Active pull-down capability disabled"]
pub type PADCFG_PAD_TESTEN_POS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Power-on-Start (POS) enabler - 1: Enable active pull down for loss of core power, 0: Active pull-down capability disabled"]
    #[inline(always)]
    pub fn padcfg_pad_testen_pos(&self) -> PADCFG_PAD_TESTEN_POS_R {
        PADCFG_PAD_TESTEN_POS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power-on-Start (POS) enabler - 1: Enable active pull down for loss of core power, 0: Active pull-down capability disabled"]
    #[inline(always)]
    #[must_use]
    pub fn padcfg_pad_testen_pos(
        &mut self,
    ) -> PADCFG_PAD_TESTEN_POS_W<AON_IOMUX_CFGSAIF_SYSCFG48_SPEC> {
        PADCFG_PAD_TESTEN_POS_W::new(self, 0)
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
#[doc = "AON IOMUX CFG SAIF SYSCFG 48\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg48::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg48::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AON_IOMUX_CFGSAIF_SYSCFG48_SPEC;
impl crate::RegisterSpec for AON_IOMUX_CFGSAIF_SYSCFG48_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_iomux_cfgsaif_syscfg48::R`](R) reader structure"]
impl crate::Readable for AON_IOMUX_CFGSAIF_SYSCFG48_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aon_iomux_cfgsaif_syscfg48::W`](W) writer structure"]
impl crate::Writable for AON_IOMUX_CFGSAIF_SYSCFG48_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
