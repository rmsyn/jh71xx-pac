#[doc = "Register `pending_1` reader"]
pub type R = crate::R<PENDING_1_SPEC>;
#[doc = "Register `pending_1` writer"]
pub type W = crate::W<PENDING_1_SPEC>;
#[doc = "Field `pending` reader - "]
pub type PENDING_R = crate::FieldReader<u32>;
#[doc = "Field `pending` writer - "]
pub type PENDING_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn pending(&self) -> PENDING_R {
        PENDING_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn pending(&mut self) -> PENDING_W<PENDING_1_SPEC> {
        PENDING_W::new(self, 0)
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
#[doc = "PENDING Register for interrupt ids 63 to 32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pending_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pending_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PENDING_1_SPEC;
impl crate::RegisterSpec for PENDING_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pending_1::R`](R) reader structure"]
impl crate::Readable for PENDING_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pending_1::W`](W) writer structure"]
impl crate::Writable for PENDING_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pending_1 to value 0"]
impl crate::Resettable for PENDING_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
