#[doc = "Register `lp_timeout` reader"]
pub type R = crate::R<LP_TIMEOUT_SPEC>;
#[doc = "Register `lp_timeout` writer"]
pub type W = crate::W<LP_TIMEOUT_SPEC>;
#[doc = "Field `lp_timeout` reader - LP Cell Control signal waiting carries acknowledge timeout."]
pub type LP_TIMEOUT_R = crate::FieldReader;
#[doc = "Field `lp_timeout` writer - LP Cell Control signal waiting carries acknowledge timeout."]
pub type LP_TIMEOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - LP Cell Control signal waiting carries acknowledge timeout."]
    #[inline(always)]
    pub fn lp_timeout(&self) -> LP_TIMEOUT_R {
        LP_TIMEOUT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - LP Cell Control signal waiting carries acknowledge timeout."]
    #[inline(always)]
    #[must_use]
    pub fn lp_timeout(&mut self) -> LP_TIMEOUT_W<LP_TIMEOUT_SPEC> {
        LP_TIMEOUT_W::new(self, 0)
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
#[doc = "LP Cell Control Timeout Threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_timeout::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_timeout::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_TIMEOUT_SPEC;
impl crate::RegisterSpec for LP_TIMEOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_timeout::R`](R) reader structure"]
impl crate::Readable for LP_TIMEOUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_timeout::W`](W) writer structure"]
impl crate::Writable for LP_TIMEOUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lp_timeout to value 0"]
impl crate::Resettable for LP_TIMEOUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
