#[doc = "Register `clk_gmac0_gtxclk` reader"]
pub type R = crate::R<CLK_GMAC0_GTXCLK_SPEC>;
#[doc = "Register `clk_gmac0_gtxclk` writer"]
pub type W = crate::W<CLK_GMAC0_GTXCLK_SPEC>;
#[doc = "Field `dly_chain_sel` reader - Selector delay chain stage number, totally 32 stages, -50 ps each stage. The register value indicates the delay chain stage number. For example, diy_chain_sel=1 means to delay 1 stage."]
pub type DLY_CHAIN_SEL_R = crate::FieldReader<u32>;
#[doc = "Field `dly_chain_sel` writer - Selector delay chain stage number, totally 32 stages, -50 ps each stage. The register value indicates the delay chain stage number. For example, diy_chain_sel=1 means to delay 1 stage."]
pub type DLY_CHAIN_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
impl R {
    #[doc = "Bits 0:23 - Selector delay chain stage number, totally 32 stages, -50 ps each stage. The register value indicates the delay chain stage number. For example, diy_chain_sel=1 means to delay 1 stage."]
    #[inline(always)]
    pub fn dly_chain_sel(&self) -> DLY_CHAIN_SEL_R {
        DLY_CHAIN_SEL_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Selector delay chain stage number, totally 32 stages, -50 ps each stage. The register value indicates the delay chain stage number. For example, diy_chain_sel=1 means to delay 1 stage."]
    #[inline(always)]
    #[must_use]
    pub fn dly_chain_sel(&mut self) -> DLY_CHAIN_SEL_W<CLK_GMAC0_GTXCLK_SPEC, 0> {
        DLY_CHAIN_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Clock GMAC 0 GTXC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_gmac0_gtxclk::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gmac0_gtxclk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_GMAC0_GTXCLK_SPEC;
impl crate::RegisterSpec for CLK_GMAC0_GTXCLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_gmac0_gtxclk::R`](R) reader structure"]
impl crate::Readable for CLK_GMAC0_GTXCLK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_gmac0_gtxclk::W`](W) writer structure"]
impl crate::Writable for CLK_GMAC0_GTXCLK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
