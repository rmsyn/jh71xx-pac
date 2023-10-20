#[doc = "Register `pch_timeout` reader"]
pub type R = crate::R<PCH_TIMEOUT_SPEC>;
#[doc = "Register `pch_timeout` writer"]
pub type W = crate::W<PCH_TIMEOUT_SPEC>;
#[doc = "Field `pch_timeout` reader - P-channel waiting device acknowledge timeout."]
pub type PCH_TIMEOUT_R = crate::FieldReader;
#[doc = "Field `pch_timeout` writer - P-channel waiting device acknowledge timeout."]
pub type PCH_TIMEOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - P-channel waiting device acknowledge timeout."]
    #[inline(always)]
    pub fn pch_timeout(&self) -> PCH_TIMEOUT_R {
        PCH_TIMEOUT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - P-channel waiting device acknowledge timeout."]
    #[inline(always)]
    #[must_use]
    pub fn pch_timeout(&mut self) -> PCH_TIMEOUT_W<PCH_TIMEOUT_SPEC, 0> {
        PCH_TIMEOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "P-channel Timeout Threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pch_timeout::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pch_timeout::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCH_TIMEOUT_SPEC;
impl crate::RegisterSpec for PCH_TIMEOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pch_timeout::R`](R) reader structure"]
impl crate::Readable for PCH_TIMEOUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pch_timeout::W`](W) writer structure"]
impl crate::Writable for PCH_TIMEOUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pch_timeout to value 0"]
impl crate::Resettable for PCH_TIMEOUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
