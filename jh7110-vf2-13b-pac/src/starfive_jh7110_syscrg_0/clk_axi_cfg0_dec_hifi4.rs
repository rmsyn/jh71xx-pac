#[doc = "Register `clk_axi_cfg0_dec_hifi4` reader"]
pub type R = crate::R<CLK_AXI_CFG0_DEC_HIFI4_SPEC>;
#[doc = "Register `clk_axi_cfg0_dec_hifi4` writer"]
pub type W = crate::W<CLK_AXI_CFG0_DEC_HIFI4_SPEC>;
#[doc = "Field `clk_icg` reader - 1: Clock enable, 0: Clock disable"]
pub type CLK_ICG_R = crate::BitReader;
#[doc = "Field `clk_icg` writer - 1: Clock enable, 0: Clock disable"]
pub type CLK_ICG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 31 - 1: Clock enable, 0: Clock disable"]
    #[inline(always)]
    pub fn clk_icg(&self) -> CLK_ICG_R {
        CLK_ICG_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - 1: Clock enable, 0: Clock disable"]
    #[inline(always)]
    #[must_use]
    pub fn clk_icg(&mut self) -> CLK_ICG_W<CLK_AXI_CFG0_DEC_HIFI4_SPEC, 31> {
        CLK_ICG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Clock AXI Config 0 DEC HIFI4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_axi_cfg0_dec_hifi4::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_axi_cfg0_dec_hifi4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_AXI_CFG0_DEC_HIFI4_SPEC;
impl crate::RegisterSpec for CLK_AXI_CFG0_DEC_HIFI4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_axi_cfg0_dec_hifi4::R`](R) reader structure"]
impl crate::Readable for CLK_AXI_CFG0_DEC_HIFI4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_axi_cfg0_dec_hifi4::W`](W) writer structure"]
impl crate::Writable for CLK_AXI_CFG0_DEC_HIFI4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
