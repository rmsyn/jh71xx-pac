#[doc = "Register `clk_i2s_bclk` reader"]
pub type R = crate::R<CLK_I2S_BCLK_SPEC>;
#[doc = "Register `clk_i2s_bclk` writer"]
pub type W = crate::W<CLK_I2S_BCLK_SPEC>;
#[doc = "Field `clk_mux_sel` reader - Clock multiplexing selector: clk_i2srx_3ch_bclk_mst, clk_i2srx_3ch_bclk_ext"]
pub type CLK_MUX_SEL_R = crate::FieldReader;
#[doc = "Field `clk_mux_sel` writer - Clock multiplexing selector: clk_i2srx_3ch_bclk_mst, clk_i2srx_3ch_bclk_ext"]
pub type CLK_MUX_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 24:29 - Clock multiplexing selector: clk_i2srx_3ch_bclk_mst, clk_i2srx_3ch_bclk_ext"]
    #[inline(always)]
    pub fn clk_mux_sel(&self) -> CLK_MUX_SEL_R {
        CLK_MUX_SEL_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:29 - Clock multiplexing selector: clk_i2srx_3ch_bclk_mst, clk_i2srx_3ch_bclk_ext"]
    #[inline(always)]
    #[must_use]
    pub fn clk_mux_sel(&mut self) -> CLK_MUX_SEL_W<CLK_I2S_BCLK_SPEC, 24> {
        CLK_MUX_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Clock I2S BCLK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_i2s_bclk::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_i2s_bclk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_I2S_BCLK_SPEC;
impl crate::RegisterSpec for CLK_I2S_BCLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_i2s_bclk::R`](R) reader structure"]
impl crate::Readable for CLK_I2S_BCLK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_i2s_bclk::W`](W) writer structure"]
impl crate::Writable for CLK_I2S_BCLK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
