#[doc = "Register `enable_1_3` reader"]
pub type R = crate::R<ENABLE_1_3_SPEC>;
#[doc = "Register `enable_1_3` writer"]
pub type W = crate::W<ENABLE_1_3_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<ENABLE_1_3_SPEC> {
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
#[doc = "ENABLE Register for interrupt ids 63 to 32 for hart 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable_1_3::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_1_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENABLE_1_3_SPEC;
impl crate::RegisterSpec for ENABLE_1_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enable_1_3::R`](R) reader structure"]
impl crate::Readable for ENABLE_1_3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`enable_1_3::W`](W) writer structure"]
impl crate::Writable for ENABLE_1_3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
