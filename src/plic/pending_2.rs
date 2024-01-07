#[doc = "Register `pending_2` reader"]
pub type R = crate::R<PENDING_2_SPEC>;
#[doc = "Register `pending_2` writer"]
pub type W = crate::W<PENDING_2_SPEC>;
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
    pub fn pending(&mut self) -> PENDING_W<PENDING_2_SPEC> {
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
#[doc = "PENDING Register for interrupt ids 95 to 64\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pending_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pending_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PENDING_2_SPEC;
impl crate::RegisterSpec for PENDING_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pending_2::R`](R) reader structure"]
impl crate::Readable for PENDING_2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pending_2::W`](W) writer structure"]
impl crate::Writable for PENDING_2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pending_2 to value 0"]
impl crate::Resettable for PENDING_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
