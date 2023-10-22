#[doc = "Register `enable_2_2` reader"]
pub type R = crate::R<ENABLE_2_2_SPEC>;
#[doc = "Register `enable_2_2` writer"]
pub type W = crate::W<ENABLE_2_2_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<ENABLE_2_2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
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
#[doc = "ENABLE Register for interrupt ids 95 to 64 for hart 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable_2_2::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_2_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENABLE_2_2_SPEC;
impl crate::RegisterSpec for ENABLE_2_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enable_2_2::R`](R) reader structure"]
impl crate::Readable for ENABLE_2_2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`enable_2_2::W`](W) writer structure"]
impl crate::Writable for ENABLE_2_2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
