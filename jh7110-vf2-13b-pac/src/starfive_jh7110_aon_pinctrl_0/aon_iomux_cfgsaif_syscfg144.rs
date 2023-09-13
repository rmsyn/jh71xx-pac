#[doc = "Register `aon_iomux_cfgsaif_syscfg144` reader"]
pub type R = crate::R<AON_IOMUX_CFGSAIF_SYSCFG144_SPEC>;
#[doc = "Register `aon_iomux_cfgsaif_syscfg144` writer"]
pub type W = crate::W<AON_IOMUX_CFGSAIF_SYSCFG144_SPEC>;
#[doc = "Field `pad_gmac0_rxc_func_sel` reader - Function selector of GMAC0_RXC: * Function 0: u0_aon_crg_clk_gmac0_rgmii_rx, * Function 1: u0_aon_crg_clk_gmac0_rmii_ref, * Function 2: None, * Function 3: None"]
pub type PAD_GMAC0_RXC_FUNC_SEL_R = crate::FieldReader;
#[doc = "Field `pad_gmac0_rxc_func_sel` writer - Function selector of GMAC0_RXC: * Function 0: u0_aon_crg_clk_gmac0_rgmii_rx, * Function 1: u0_aon_crg_clk_gmac0_rmii_ref, * Function 2: None, * Function 3: None"]
pub type PAD_GMAC0_RXC_FUNC_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Function selector of GMAC0_RXC: * Function 0: u0_aon_crg_clk_gmac0_rgmii_rx, * Function 1: u0_aon_crg_clk_gmac0_rmii_ref, * Function 2: None, * Function 3: None"]
    #[inline(always)]
    pub fn pad_gmac0_rxc_func_sel(&self) -> PAD_GMAC0_RXC_FUNC_SEL_R {
        PAD_GMAC0_RXC_FUNC_SEL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Function selector of GMAC0_RXC: * Function 0: u0_aon_crg_clk_gmac0_rgmii_rx, * Function 1: u0_aon_crg_clk_gmac0_rmii_ref, * Function 2: None, * Function 3: None"]
    #[inline(always)]
    #[must_use]
    pub fn pad_gmac0_rxc_func_sel(
        &mut self,
    ) -> PAD_GMAC0_RXC_FUNC_SEL_W<AON_IOMUX_CFGSAIF_SYSCFG144_SPEC, 0> {
        PAD_GMAC0_RXC_FUNC_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AON IOMUX CFG SAIF SYSCFG 144\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg144::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg144::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AON_IOMUX_CFGSAIF_SYSCFG144_SPEC;
impl crate::RegisterSpec for AON_IOMUX_CFGSAIF_SYSCFG144_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_iomux_cfgsaif_syscfg144::R`](R) reader structure"]
impl crate::Readable for AON_IOMUX_CFGSAIF_SYSCFG144_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aon_iomux_cfgsaif_syscfg144::W`](W) writer structure"]
impl crate::Writable for AON_IOMUX_CFGSAIF_SYSCFG144_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
