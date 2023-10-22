#[doc = "Register `rand6` reader"]
pub type R = crate::R<RAND6_SPEC>;
#[doc = "Register `rand6` writer"]
pub type W = crate::W<RAND6_SPEC>;
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "TRNG RAND 6 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rand6::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rand6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAND6_SPEC;
impl crate::RegisterSpec for RAND6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rand6::R`](R) reader structure"]
impl crate::Readable for RAND6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rand6::W`](W) writer structure"]
impl crate::Writable for RAND6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}