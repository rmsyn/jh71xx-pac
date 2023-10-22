#[doc = "Register `claimplete_4` reader"]
pub type R = crate::R<CLAIMPLETE_4_SPEC>;
#[doc = "Register `claimplete_4` writer"]
pub type W = crate::W<CLAIMPLETE_4_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<CLAIMPLETE_4_SPEC> {
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
#[doc = "CLAIM and COMPLETE Register for hart 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`claimplete_4::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`claimplete_4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLAIMPLETE_4_SPEC;
impl crate::RegisterSpec for CLAIMPLETE_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`claimplete_4::R`](R) reader structure"]
impl crate::Readable for CLAIMPLETE_4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`claimplete_4::W`](W) writer structure"]
impl crate::Writable for CLAIMPLETE_4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}