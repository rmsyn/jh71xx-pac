#[doc = "Register `claimplete_0` reader"]
pub type R = crate::R<CLAIMPLETE_0_SPEC>;
#[doc = "Register `claimplete_0` writer"]
pub type W = crate::W<CLAIMPLETE_0_SPEC>;
#[doc = "Field `claimplete` reader - "]
pub type CLAIMPLETE_R = crate::FieldReader<u32>;
#[doc = "Field `claimplete` writer - "]
pub type CLAIMPLETE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn claimplete(&self) -> CLAIMPLETE_R {
        CLAIMPLETE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn claimplete(&mut self) -> CLAIMPLETE_W<CLAIMPLETE_0_SPEC> {
        CLAIMPLETE_W::new(self, 0)
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
#[doc = "CLAIM and COMPLETE Register for hart 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`claimplete_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`claimplete_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLAIMPLETE_0_SPEC;
impl crate::RegisterSpec for CLAIMPLETE_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`claimplete_0::R`](R) reader structure"]
impl crate::Readable for CLAIMPLETE_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`claimplete_0::W`](W) writer structure"]
impl crate::Writable for CLAIMPLETE_0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets claimplete_0 to value 0"]
impl crate::Resettable for CLAIMPLETE_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
