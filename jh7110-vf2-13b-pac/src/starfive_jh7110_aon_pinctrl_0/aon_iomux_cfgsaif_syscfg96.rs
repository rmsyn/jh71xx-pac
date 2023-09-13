#[doc = "Register `aon_iomux_cfgsaif_syscfg96` reader"]
pub type R = crate::R<AON_IOMUX_CFGSAIF_SYSCFG96_SPEC>;
#[doc = "Register `aon_iomux_cfgsaif_syscfg96` writer"]
pub type W = crate::W<AON_IOMUX_CFGSAIF_SYSCFG96_SPEC>;
#[doc = "Field `padcfg_pad_gmac0_rxd0_syscon` reader - 0: GMAC0 IO voltage select 3.3V, 1: GMAC0 IO voltage select 2.5V, 2: GMAC0 IO voltage select 1.8V"]
pub type PADCFG_PAD_GMAC0_RXD0_SYSCON_R = crate::FieldReader;
#[doc = "Field `padcfg_pad_gmac0_rxd0_syscon` writer - 0: GMAC0 IO voltage select 3.3V, 1: GMAC0 IO voltage select 2.5V, 2: GMAC0 IO voltage select 1.8V"]
pub type PADCFG_PAD_GMAC0_RXD0_SYSCON_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - 0: GMAC0 IO voltage select 3.3V, 1: GMAC0 IO voltage select 2.5V, 2: GMAC0 IO voltage select 1.8V"]
    #[inline(always)]
    pub fn padcfg_pad_gmac0_rxd0_syscon(&self) -> PADCFG_PAD_GMAC0_RXD0_SYSCON_R {
        PADCFG_PAD_GMAC0_RXD0_SYSCON_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 0: GMAC0 IO voltage select 3.3V, 1: GMAC0 IO voltage select 2.5V, 2: GMAC0 IO voltage select 1.8V"]
    #[inline(always)]
    #[must_use]
    pub fn padcfg_pad_gmac0_rxd0_syscon(
        &mut self,
    ) -> PADCFG_PAD_GMAC0_RXD0_SYSCON_W<AON_IOMUX_CFGSAIF_SYSCFG96_SPEC, 0> {
        PADCFG_PAD_GMAC0_RXD0_SYSCON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AON IOMUX CFG SAIF SYSCFG 96\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg96::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg96::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AON_IOMUX_CFGSAIF_SYSCFG96_SPEC;
impl crate::RegisterSpec for AON_IOMUX_CFGSAIF_SYSCFG96_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_iomux_cfgsaif_syscfg96::R`](R) reader structure"]
impl crate::Readable for AON_IOMUX_CFGSAIF_SYSCFG96_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aon_iomux_cfgsaif_syscfg96::W`](W) writer structure"]
impl crate::Writable for AON_IOMUX_CFGSAIF_SYSCFG96_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
