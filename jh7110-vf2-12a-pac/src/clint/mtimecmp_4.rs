#[doc = "Register `mtimecmp_4` reader"]
pub type R = crate::R<MTIMECMP_4_SPEC>;
#[doc = "Register `mtimecmp_4` writer"]
pub type W = crate::W<MTIMECMP_4_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<MTIMECMP_4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u64) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MTIMECMP Register for hart 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtimecmp_4::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtimecmp_4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTIMECMP_4_SPEC;
impl crate::RegisterSpec for MTIMECMP_4_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [`mtimecmp_4::R`](R) reader structure"]
impl crate::Readable for MTIMECMP_4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mtimecmp_4::W`](W) writer structure"]
impl crate::Writable for MTIMECMP_4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
