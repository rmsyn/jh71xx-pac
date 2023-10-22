#[doc = "Register `clk_ddr_bus` reader"]
pub type R = crate::R<CLK_DDR_BUS_SPEC>;
#[doc = "Register `clk_ddr_bus` writer"]
pub type W = crate::W<CLK_DDR_BUS_SPEC>;
#[doc = "Field `clk_mux_sel` reader - Clock multiplexing selector: clk_osc_div2, clk_pll1_div4, clk_pll1_div8"]
pub type CLK_MUX_SEL_R = crate::FieldReader;
#[doc = "Field `clk_mux_sel` writer - Clock multiplexing selector: clk_osc_div2, clk_pll1_div4, clk_pll1_div8"]
pub type CLK_MUX_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 24:29 - Clock multiplexing selector: clk_osc_div2, clk_pll1_div4, clk_pll1_div8"]
    #[inline(always)]
    pub fn clk_mux_sel(&self) -> CLK_MUX_SEL_R {
        CLK_MUX_SEL_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:29 - Clock multiplexing selector: clk_osc_div2, clk_pll1_div4, clk_pll1_div8"]
    #[inline(always)]
    #[must_use]
    pub fn clk_mux_sel(&mut self) -> CLK_MUX_SEL_W<CLK_DDR_BUS_SPEC, 24> {
        CLK_MUX_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "clk_ddr_bus\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_ddr_bus::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_ddr_bus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_DDR_BUS_SPEC;
impl crate::RegisterSpec for CLK_DDR_BUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_ddr_bus::R`](R) reader structure"]
impl crate::Readable for CLK_DDR_BUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_ddr_bus::W`](W) writer structure"]
impl crate::Writable for CLK_DDR_BUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
