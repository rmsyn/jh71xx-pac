#[doc = "Register `rand_4` reader"]
pub type R = crate::R<RAND_4_SPEC>;
#[doc = "Register `rand_4` writer"]
pub type W = crate::W<RAND_4_SPEC>;
#[doc = "Field `rand` reader - Random number bits"]
pub type RAND_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Random number bits"]
    #[inline(always)]
    pub fn rand(&self) -> RAND_R {
        RAND_R::new(self.bits)
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
#[doc = "TRNG RAND 4 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rand_4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rand_4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAND_4_SPEC;
impl crate::RegisterSpec for RAND_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rand_4::R`](R) reader structure"]
impl crate::Readable for RAND_4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rand_4::W`](W) writer structure"]
impl crate::Writable for RAND_4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rand_4 to value 0"]
impl crate::Resettable for RAND_4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
