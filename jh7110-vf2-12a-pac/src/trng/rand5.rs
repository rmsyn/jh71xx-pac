#[doc = "Register `rand5` reader"]
pub type R = crate::R<RAND5_SPEC>;
#[doc = "Register `rand5` writer"]
pub type W = crate::W<RAND5_SPEC>;
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
#[doc = "TRNG RAND 5 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rand5::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rand5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAND5_SPEC;
impl crate::RegisterSpec for RAND5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rand5::R`](R) reader structure"]
impl crate::Readable for RAND5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rand5::W`](W) writer structure"]
impl crate::Writable for RAND5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
