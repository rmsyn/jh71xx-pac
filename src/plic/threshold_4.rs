#[doc = "Register `threshold_4` reader"]
pub type R = crate::R<THRESHOLD_4_SPEC>;
#[doc = "Register `threshold_4` writer"]
pub type W = crate::W<THRESHOLD_4_SPEC>;
#[doc = "Field `priority` reader - "]
pub type PRIORITY_R = crate::FieldReader<u32>;
#[doc = "Field `priority` writer - "]
pub type PRIORITY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn priority(&self) -> PRIORITY_R {
        PRIORITY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn priority(&mut self) -> PRIORITY_W<THRESHOLD_4_SPEC> {
        PRIORITY_W::new(self, 0)
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
#[doc = "PRIORITY THRESHOLD Register for hart 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`threshold_4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`threshold_4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct THRESHOLD_4_SPEC;
impl crate::RegisterSpec for THRESHOLD_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`threshold_4::R`](R) reader structure"]
impl crate::Readable for THRESHOLD_4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`threshold_4::W`](W) writer structure"]
impl crate::Writable for THRESHOLD_4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets threshold_4 to value 0"]
impl crate::Resettable for THRESHOLD_4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
