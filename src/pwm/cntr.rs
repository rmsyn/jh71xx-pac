#[doc = "Register `cntr` reader"]
pub type R = crate::R<CNTR_SPEC>;
#[doc = "Register `cntr` writer"]
pub type W = crate::W<CNTR_SPEC>;
#[doc = "Field `cntr` reader - PWM PTC counter"]
pub type CNTR_R = crate::FieldReader<u32>;
#[doc = "Field `cntr` writer - PWM PTC counter"]
pub type CNTR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PWM PTC counter"]
    #[inline(always)]
    pub fn cntr(&self) -> CNTR_R {
        CNTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PWM PTC counter"]
    #[inline(always)]
    #[must_use]
    pub fn cntr(&mut self) -> CNTR_W<CNTR_SPEC> {
        CNTR_W::new(self, 0)
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
#[doc = "PTC counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntr::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNTR_SPEC;
impl crate::RegisterSpec for CNTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cntr::R`](R) reader structure"]
impl crate::Readable for CNTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cntr::W`](W) writer structure"]
impl crate::Writable for CNTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
