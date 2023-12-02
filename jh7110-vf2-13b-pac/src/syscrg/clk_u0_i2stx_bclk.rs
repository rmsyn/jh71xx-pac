#[doc = "Register `clk_u0_i2stx_bclk` reader"]
pub type R = crate::R<CLK_U0_I2STX_BCLK_SPEC>;
#[doc = "Register `clk_u0_i2stx_bclk` writer"]
pub type W = crate::W<CLK_U0_I2STX_BCLK_SPEC>;
#[doc = "Field `clk_mux_sel` reader - Clock multiplexing selector: clk_i2stx_4ch0_bclk_mst, clk_i2stx_bclk_ext"]
pub type CLK_MUX_SEL_R = crate::FieldReader;
#[doc = "Field `clk_mux_sel` writer - Clock multiplexing selector: clk_i2stx_4ch0_bclk_mst, clk_i2stx_bclk_ext"]
pub type CLK_MUX_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 24:29 - Clock multiplexing selector: clk_i2stx_4ch0_bclk_mst, clk_i2stx_bclk_ext"]
    #[inline(always)]
    pub fn clk_mux_sel(&self) -> CLK_MUX_SEL_R {
        CLK_MUX_SEL_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:29 - Clock multiplexing selector: clk_i2stx_4ch0_bclk_mst, clk_i2stx_bclk_ext"]
    #[inline(always)]
    #[must_use]
    pub fn clk_mux_sel(&mut self) -> CLK_MUX_SEL_W<CLK_U0_I2STX_BCLK_SPEC> {
        CLK_MUX_SEL_W::new(self, 24)
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
#[doc = "U0 Clock I2S TX BCLK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_i2stx_bclk::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_i2stx_bclk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_U0_I2STX_BCLK_SPEC;
impl crate::RegisterSpec for CLK_U0_I2STX_BCLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_u0_i2stx_bclk::R`](R) reader structure"]
impl crate::Readable for CLK_U0_I2STX_BCLK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_u0_i2stx_bclk::W`](W) writer structure"]
impl crate::Writable for CLK_U0_I2STX_BCLK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
