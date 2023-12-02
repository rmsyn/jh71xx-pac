#[doc = "Register `priority_101` reader"]
pub type R = crate::R<PRIORITY_101_SPEC>;
#[doc = "Register `priority_101` writer"]
pub type W = crate::W<PRIORITY_101_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<PRIORITY_101_SPEC> {
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
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PRIORITY Register for interrupt id 101\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_101::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_101::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRIORITY_101_SPEC;
impl crate::RegisterSpec for PRIORITY_101_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`priority_101::R`](R) reader structure"]
impl crate::Readable for PRIORITY_101_SPEC {}
#[doc = "`write(|w| ..)` method takes [`priority_101::W`](W) writer structure"]
impl crate::Writable for PRIORITY_101_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
