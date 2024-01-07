#[doc = "Register `gmac0_rxc_func_sel` reader"]
pub type R = crate::R<GMAC0_RXC_FUNC_SEL_SPEC>;
#[doc = "Register `gmac0_rxc_func_sel` writer"]
pub type W = crate::W<GMAC0_RXC_FUNC_SEL_SPEC>;
#[doc = "Field `value` reader - Function selector of GMAC0_RXC: * Function 0: u0_aon_crg_clk_gmac0_rgmii_rx, * Function 1: u0_aon_crg_clk_gmac0_rmii_ref, * Function 2: None, * Function 3: None"]
pub type VALUE_R = crate::FieldReader;
#[doc = "Field `value` writer - Function selector of GMAC0_RXC: * Function 0: u0_aon_crg_clk_gmac0_rgmii_rx, * Function 1: u0_aon_crg_clk_gmac0_rmii_ref, * Function 2: None, * Function 3: None"]
pub type VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Function selector of GMAC0_RXC: * Function 0: u0_aon_crg_clk_gmac0_rgmii_rx, * Function 1: u0_aon_crg_clk_gmac0_rmii_ref, * Function 2: None, * Function 3: None"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Function selector of GMAC0_RXC: * Function 0: u0_aon_crg_clk_gmac0_rgmii_rx, * Function 1: u0_aon_crg_clk_gmac0_rmii_ref, * Function 2: None, * Function 3: None"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<GMAC0_RXC_FUNC_SEL_SPEC> {
        VALUE_W::new(self, 0)
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
#[doc = "AON IOMUX CFG SAIF SYSCFG 144\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac0_rxc_func_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac0_rxc_func_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GMAC0_RXC_FUNC_SEL_SPEC;
impl crate::RegisterSpec for GMAC0_RXC_FUNC_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac0_rxc_func_sel::R`](R) reader structure"]
impl crate::Readable for GMAC0_RXC_FUNC_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gmac0_rxc_func_sel::W`](W) writer structure"]
impl crate::Writable for GMAC0_RXC_FUNC_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gmac0_rxc_func_sel to value 0"]
impl crate::Resettable for GMAC0_RXC_FUNC_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
