#[doc = "Register `clk_i2s_bclk_neg` reader"]
pub type R = crate::R<CLK_I2S_BCLK_NEG_SPEC>;
#[doc = "Register `clk_i2s_bclk_neg` writer"]
pub type W = crate::W<CLK_I2S_BCLK_NEG_SPEC>;
#[doc = "Field `clk_polarity` reader - 1: Clock inverter, 0: Clock buffer"]
pub type CLK_POLARITY_R = crate::BitReader;
#[doc = "Field `clk_polarity` writer - 1: Clock inverter, 0: Clock buffer"]
pub type CLK_POLARITY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 30 - 1: Clock inverter, 0: Clock buffer"]
    #[inline(always)]
    pub fn clk_polarity(&self) -> CLK_POLARITY_R {
        CLK_POLARITY_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - 1: Clock inverter, 0: Clock buffer"]
    #[inline(always)]
    #[must_use]
    pub fn clk_polarity(&mut self) -> CLK_POLARITY_W<CLK_I2S_BCLK_NEG_SPEC, 30> {
        CLK_POLARITY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Clock I2S BCLK Negative\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_i2s_bclk_neg::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_i2s_bclk_neg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_I2S_BCLK_NEG_SPEC;
impl crate::RegisterSpec for CLK_I2S_BCLK_NEG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_i2s_bclk_neg::R`](R) reader structure"]
impl crate::Readable for CLK_I2S_BCLK_NEG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_i2s_bclk_neg::W`](W) writer structure"]
impl crate::Writable for CLK_I2S_BCLK_NEG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
