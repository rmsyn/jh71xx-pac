#[doc = "Register `threshold_0` reader"]
pub type R = crate::R<THRESHOLD_0_SPEC>;
#[doc = "Register `threshold_0` writer"]
pub type W = crate::W<THRESHOLD_0_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<THRESHOLD_0_SPEC> {
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
#[doc = "PRIORITY THRESHOLD Register for hart 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`threshold_0::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`threshold_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct THRESHOLD_0_SPEC;
impl crate::RegisterSpec for THRESHOLD_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`threshold_0::R`](R) reader structure"]
impl crate::Readable for THRESHOLD_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`threshold_0::W`](W) writer structure"]
impl crate::Writable for THRESHOLD_0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
