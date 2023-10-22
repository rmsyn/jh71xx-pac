#[doc = "Register `threshold_1` reader"]
pub type R = crate::R<THRESHOLD_1_SPEC>;
#[doc = "Register `threshold_1` writer"]
pub type W = crate::W<THRESHOLD_1_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<THRESHOLD_1_SPEC> {
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
#[doc = "PRIORITY THRESHOLD Register for hart 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`threshold_1::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`threshold_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct THRESHOLD_1_SPEC;
impl crate::RegisterSpec for THRESHOLD_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`threshold_1::R`](R) reader structure"]
impl crate::Readable for THRESHOLD_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`threshold_1::W`](W) writer structure"]
impl crate::Writable for THRESHOLD_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}