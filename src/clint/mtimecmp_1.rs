#[doc = "Register `mtimecmp_1` reader"]
pub type R = crate::R<MTIMECMP_1_SPEC>;
#[doc = "Register `mtimecmp_1` writer"]
pub type W = crate::W<MTIMECMP_1_SPEC>;
#[doc = "Field `cycles` reader - "]
pub type CYCLES_R = crate::FieldReader<u64>;
#[doc = "Field `cycles` writer - "]
pub type CYCLES_W<'a, REG> = crate::FieldWriter<'a, REG, 64, u64>;
impl R {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn cycles(&self) -> CYCLES_R {
        CYCLES_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    #[must_use]
    pub fn cycles(&mut self) -> CYCLES_W<MTIMECMP_1_SPEC> {
        CYCLES_W::new(self, 0)
    }
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
#[doc = "MTIMECMP Register for hart 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtimecmp_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtimecmp_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTIMECMP_1_SPEC;
impl crate::RegisterSpec for MTIMECMP_1_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [`mtimecmp_1::R`](R) reader structure"]
impl crate::Readable for MTIMECMP_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mtimecmp_1::W`](W) writer structure"]
impl crate::Writable for MTIMECMP_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mtimecmp_1 to value 0"]
impl crate::Resettable for MTIMECMP_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
