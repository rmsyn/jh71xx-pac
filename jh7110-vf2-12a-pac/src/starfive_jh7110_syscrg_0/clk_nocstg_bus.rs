#[doc = "Register `clk_nocstg_bus` reader"]
pub type R = crate::R<CLK_NOCSTG_BUS_SPEC>;
#[doc = "Register `clk_nocstg_bus` writer"]
pub type W = crate::W<CLK_NOCSTG_BUS_SPEC>;
#[doc = "Field `clk_divcfg` reader - Clock divider coefficient: Max=3, Default=3, Min=3, Typical=3"]
pub type CLK_DIVCFG_R = crate::FieldReader<u32>;
#[doc = "Field `clk_divcfg` writer - Clock divider coefficient: Max=3, Default=3, Min=3, Typical=3"]
pub type CLK_DIVCFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
impl R {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=3, Default=3, Min=3, Typical=3"]
    #[inline(always)]
    pub fn clk_divcfg(&self) -> CLK_DIVCFG_R {
        CLK_DIVCFG_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=3, Default=3, Min=3, Typical=3"]
    #[inline(always)]
    #[must_use]
    pub fn clk_divcfg(&mut self) -> CLK_DIVCFG_W<CLK_NOCSTG_BUS_SPEC, 0> {
        CLK_DIVCFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Clock NOCSTG Bus\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_nocstg_bus::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_nocstg_bus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_NOCSTG_BUS_SPEC;
impl crate::RegisterSpec for CLK_NOCSTG_BUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_nocstg_bus::R`](R) reader structure"]
impl crate::Readable for CLK_NOCSTG_BUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_nocstg_bus::W`](W) writer structure"]
impl crate::Writable for CLK_NOCSTG_BUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
