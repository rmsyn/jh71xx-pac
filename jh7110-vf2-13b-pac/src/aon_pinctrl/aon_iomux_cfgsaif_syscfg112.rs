#[doc = "Register `aon_iomux_cfgsaif_syscfg112` reader"]
pub type R = crate::R<AON_IOMUX_CFGSAIF_SYSCFG112_SPEC>;
#[doc = "Register `aon_iomux_cfgsaif_syscfg112` writer"]
pub type W = crate::W<AON_IOMUX_CFGSAIF_SYSCFG112_SPEC>;
#[doc = "Field `padcfg_pad_gmac0_rxdv_syscon` reader - padcfg_pad_gmac0_rxdv_syscon"]
pub type PADCFG_PAD_GMAC0_RXDV_SYSCON_R = crate::FieldReader;
#[doc = "Field `padcfg_pad_gmac0_rxdv_syscon` writer - padcfg_pad_gmac0_rxdv_syscon"]
pub type PADCFG_PAD_GMAC0_RXDV_SYSCON_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - padcfg_pad_gmac0_rxdv_syscon"]
    #[inline(always)]
    pub fn padcfg_pad_gmac0_rxdv_syscon(&self) -> PADCFG_PAD_GMAC0_RXDV_SYSCON_R {
        PADCFG_PAD_GMAC0_RXDV_SYSCON_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - padcfg_pad_gmac0_rxdv_syscon"]
    #[inline(always)]
    #[must_use]
    pub fn padcfg_pad_gmac0_rxdv_syscon(
        &mut self,
    ) -> PADCFG_PAD_GMAC0_RXDV_SYSCON_W<AON_IOMUX_CFGSAIF_SYSCFG112_SPEC> {
        PADCFG_PAD_GMAC0_RXDV_SYSCON_W::new(self, 0)
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
#[doc = "AON IOMUX CFG SAIF SYSCFG 112\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg112::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg112::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AON_IOMUX_CFGSAIF_SYSCFG112_SPEC;
impl crate::RegisterSpec for AON_IOMUX_CFGSAIF_SYSCFG112_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_iomux_cfgsaif_syscfg112::R`](R) reader structure"]
impl crate::Readable for AON_IOMUX_CFGSAIF_SYSCFG112_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aon_iomux_cfgsaif_syscfg112::W`](W) writer structure"]
impl crate::Writable for AON_IOMUX_CFGSAIF_SYSCFG112_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
