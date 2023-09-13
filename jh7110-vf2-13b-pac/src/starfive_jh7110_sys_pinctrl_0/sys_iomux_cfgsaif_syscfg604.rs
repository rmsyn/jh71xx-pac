#[doc = "Register `sys_iomux_cfgsaif_syscfg604` reader"]
pub type R = crate::R<SYS_IOMUX_CFGSAIF_SYSCFG604_SPEC>;
#[doc = "Register `sys_iomux_cfgsaif_syscfg604` writer"]
pub type W = crate::W<SYS_IOMUX_CFGSAIF_SYSCFG604_SPEC>;
#[doc = "Field `padcfg_pad_gmac1_rxd2_syscon` reader - padcfg_pad_gmac1_rxd2_syscon"]
pub type PADCFG_PAD_GMAC1_RXD2_SYSCON_R = crate::FieldReader;
#[doc = "Field `padcfg_pad_gmac1_rxd2_syscon` writer - padcfg_pad_gmac1_rxd2_syscon"]
pub type PADCFG_PAD_GMAC1_RXD2_SYSCON_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - padcfg_pad_gmac1_rxd2_syscon"]
    #[inline(always)]
    pub fn padcfg_pad_gmac1_rxd2_syscon(&self) -> PADCFG_PAD_GMAC1_RXD2_SYSCON_R {
        PADCFG_PAD_GMAC1_RXD2_SYSCON_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - padcfg_pad_gmac1_rxd2_syscon"]
    #[inline(always)]
    #[must_use]
    pub fn padcfg_pad_gmac1_rxd2_syscon(
        &mut self,
    ) -> PADCFG_PAD_GMAC1_RXD2_SYSCON_W<SYS_IOMUX_CFGSAIF_SYSCFG604_SPEC, 0> {
        PADCFG_PAD_GMAC1_RXD2_SYSCON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG 604\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_iomux_cfgsaif_syscfg604::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_iomux_cfgsaif_syscfg604::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_IOMUX_CFGSAIF_SYSCFG604_SPEC;
impl crate::RegisterSpec for SYS_IOMUX_CFGSAIF_SYSCFG604_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_iomux_cfgsaif_syscfg604::R`](R) reader structure"]
impl crate::Readable for SYS_IOMUX_CFGSAIF_SYSCFG604_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_iomux_cfgsaif_syscfg604::W`](W) writer structure"]
impl crate::Writable for SYS_IOMUX_CFGSAIF_SYSCFG604_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
