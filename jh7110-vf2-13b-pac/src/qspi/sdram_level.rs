#[doc = "Register `sdram_level` reader"]
pub type R = crate::R<SDRAM_LEVEL_SPEC>;
#[doc = "Register `sdram_level` writer"]
pub type W = crate::W<SDRAM_LEVEL_SPEC>;
#[doc = "Field `rd` reader - SDRAM Read Level"]
pub type RD_R = crate::FieldReader<u16>;
#[doc = "Field `wr` reader - SDRAM Write Level"]
pub type WR_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - SDRAM Read Level"]
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - SDRAM Write Level"]
    #[inline(always)]
    pub fn wr(&self) -> WR_R {
        WR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Cadence QSPI SDRAM Level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdram_level::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdram_level::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDRAM_LEVEL_SPEC;
impl crate::RegisterSpec for SDRAM_LEVEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdram_level::R`](R) reader structure"]
impl crate::Readable for SDRAM_LEVEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sdram_level::W`](W) writer structure"]
impl crate::Writable for SDRAM_LEVEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sdram_level to value 0"]
impl crate::Resettable for SDRAM_LEVEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
