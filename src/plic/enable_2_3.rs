#[doc = "Register `enable_2_3` reader"]
pub type R = crate::R<ENABLE_2_3_SPEC>;
#[doc = "Register `enable_2_3` writer"]
pub type W = crate::W<ENABLE_2_3_SPEC>;
#[doc = "Field `enable` reader - "]
pub type ENABLE_R = crate::FieldReader<u32>;
#[doc = "Field `enable` writer - "]
pub type ENABLE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<ENABLE_2_3_SPEC> {
        ENABLE_W::new(self, 0)
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
#[doc = "ENABLE Register for interrupt ids 95 to 64 for hart 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable_2_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_2_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENABLE_2_3_SPEC;
impl crate::RegisterSpec for ENABLE_2_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enable_2_3::R`](R) reader structure"]
impl crate::Readable for ENABLE_2_3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`enable_2_3::W`](W) writer structure"]
impl crate::Writable for ENABLE_2_3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets enable_2_3 to value 0"]
impl crate::Resettable for ENABLE_2_3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
