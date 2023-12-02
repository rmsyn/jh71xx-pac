#[doc = "Register `clk_pll1_div8` reader"]
pub type R = crate::R<CLK_PLL1_DIV8_SPEC>;
#[doc = "Register `clk_pll1_div8` writer"]
pub type W = crate::W<CLK_PLL1_DIV8_SPEC>;
#[doc = "Field `clk_divcfg` reader - Clock divider coefficient: Max=2, Default=2, Min=2, Typical=2"]
pub type CLK_DIVCFG_R = crate::FieldReader<u32>;
#[doc = "Field `clk_divcfg` writer - Clock divider coefficient: Max=2, Default=2, Min=2, Typical=2"]
pub type CLK_DIVCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=2, Default=2, Min=2, Typical=2"]
    #[inline(always)]
    pub fn clk_divcfg(&self) -> CLK_DIVCFG_R {
        CLK_DIVCFG_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=2, Default=2, Min=2, Typical=2"]
    #[inline(always)]
    #[must_use]
    pub fn clk_divcfg(&mut self) -> CLK_DIVCFG_W<CLK_PLL1_DIV8_SPEC> {
        CLK_DIVCFG_W::new(self, 0)
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
#[doc = "clk_pll1_div8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_pll1_div8::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_pll1_div8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_PLL1_DIV8_SPEC;
impl crate::RegisterSpec for CLK_PLL1_DIV8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_pll1_div8::R`](R) reader structure"]
impl crate::Readable for CLK_PLL1_DIV8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_pll1_div8::W`](W) writer structure"]
impl crate::Writable for CLK_PLL1_DIV8_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
