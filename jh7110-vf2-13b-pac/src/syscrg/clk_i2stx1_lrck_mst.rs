#[doc = "Register `clk_i2stx1_lrck_mst` reader"]
pub type R = crate::R<CLK_I2STX1_LRCK_MST_SPEC>;
#[doc = "Register `clk_i2stx1_lrck_mst` writer"]
pub type W = crate::W<CLK_I2STX1_LRCK_MST_SPEC>;
#[doc = "Field `clk_divcfg` reader - Clock divider coefficient: Max=64, Default=64, Min=64, Typical=64"]
pub type CLK_DIVCFG_R = crate::FieldReader<u32>;
#[doc = "Field `clk_divcfg` writer - Clock divider coefficient: Max=64, Default=64, Min=64, Typical=64"]
pub type CLK_DIVCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `clk_mux_sel` reader - Clock multiplexing selector: clk_i2stx_4ch0_bclk_mst_inv, clk_i2stx_4ch0_bclk_mst"]
pub type CLK_MUX_SEL_R = crate::FieldReader;
#[doc = "Field `clk_mux_sel` writer - Clock multiplexing selector: clk_i2stx_4ch0_bclk_mst_inv, clk_i2stx_4ch0_bclk_mst"]
pub type CLK_MUX_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=64, Default=64, Min=64, Typical=64"]
    #[inline(always)]
    pub fn clk_divcfg(&self) -> CLK_DIVCFG_R {
        CLK_DIVCFG_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:29 - Clock multiplexing selector: clk_i2stx_4ch0_bclk_mst_inv, clk_i2stx_4ch0_bclk_mst"]
    #[inline(always)]
    pub fn clk_mux_sel(&self) -> CLK_MUX_SEL_R {
        CLK_MUX_SEL_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=64, Default=64, Min=64, Typical=64"]
    #[inline(always)]
    #[must_use]
    pub fn clk_divcfg(&mut self) -> CLK_DIVCFG_W<CLK_I2STX1_LRCK_MST_SPEC> {
        CLK_DIVCFG_W::new(self, 0)
    }
    #[doc = "Bits 24:29 - Clock multiplexing selector: clk_i2stx_4ch0_bclk_mst_inv, clk_i2stx_4ch0_bclk_mst"]
    #[inline(always)]
    #[must_use]
    pub fn clk_mux_sel(&mut self) -> CLK_MUX_SEL_W<CLK_I2STX1_LRCK_MST_SPEC> {
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
#[doc = "Clock I2S TX 1 LRCK MST\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_i2stx1_lrck_mst::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_i2stx1_lrck_mst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_I2STX1_LRCK_MST_SPEC;
impl crate::RegisterSpec for CLK_I2STX1_LRCK_MST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_i2stx1_lrck_mst::R`](R) reader structure"]
impl crate::Readable for CLK_I2STX1_LRCK_MST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_i2stx1_lrck_mst::W`](W) writer structure"]
impl crate::Writable for CLK_I2STX1_LRCK_MST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
