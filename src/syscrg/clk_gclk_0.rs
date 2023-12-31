#[doc = "Register `clk_gclk_0` reader"]
pub type R = crate::R<CLK_GCLK_0_SPEC>;
#[doc = "Register `clk_gclk_0` writer"]
pub type W = crate::W<CLK_GCLK_0_SPEC>;
#[doc = "Field `clk_divcfg` reader - Clock divider coefficient: Max=62, Default=20, Min=16, Typical=20"]
pub type CLK_DIVCFG_R = crate::FieldReader<u32>;
#[doc = "Field `clk_divcfg` writer - Clock divider coefficient: Max=62, Default=20, Min=16, Typical=20"]
pub type CLK_DIVCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `clk_icg` reader - 1: Clock enable, 0: Clock disable"]
pub type CLK_ICG_R = crate::BitReader;
#[doc = "Field `clk_icg` writer - 1: Clock enable, 0: Clock disable"]
pub type CLK_ICG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=62, Default=20, Min=16, Typical=20"]
    #[inline(always)]
    pub fn clk_divcfg(&self) -> CLK_DIVCFG_R {
        CLK_DIVCFG_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 31 - 1: Clock enable, 0: Clock disable"]
    #[inline(always)]
    pub fn clk_icg(&self) -> CLK_ICG_R {
        CLK_ICG_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=62, Default=20, Min=16, Typical=20"]
    #[inline(always)]
    #[must_use]
    pub fn clk_divcfg(&mut self) -> CLK_DIVCFG_W<CLK_GCLK_0_SPEC> {
        CLK_DIVCFG_W::new(self, 0)
    }
    #[doc = "Bit 31 - 1: Clock enable, 0: Clock disable"]
    #[inline(always)]
    #[must_use]
    pub fn clk_icg(&mut self) -> CLK_ICG_W<CLK_GCLK_0_SPEC> {
        CLK_ICG_W::new(self, 31)
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
#[doc = "Clock GCLK 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_gclk_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gclk_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_GCLK_0_SPEC;
impl crate::RegisterSpec for CLK_GCLK_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_gclk_0::R`](R) reader structure"]
impl crate::Readable for CLK_GCLK_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_gclk_0::W`](W) writer structure"]
impl crate::Writable for CLK_GCLK_0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets clk_gclk_0 to value 0"]
impl crate::Resettable for CLK_GCLK_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
