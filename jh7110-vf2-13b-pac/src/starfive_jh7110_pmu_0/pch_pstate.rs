#[doc = "Register `pch_pstate` reader"]
pub type R = crate::R<PCH_PSTATE_SPEC>;
#[doc = "Register `pch_pstate` writer"]
pub type W = crate::W<PCH_PSTATE_SPEC>;
#[doc = "Field `pch_pstate` reader - P-channel state set"]
pub type PCH_PSTATE_R = crate::FieldReader;
#[doc = "Field `pch_pstate` writer - P-channel state set"]
pub type PCH_PSTATE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - P-channel state set"]
    #[inline(always)]
    pub fn pch_pstate(&self) -> PCH_PSTATE_R {
        PCH_PSTATE_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - P-channel state set"]
    #[inline(always)]
    #[must_use]
    pub fn pch_pstate(&mut self) -> PCH_PSTATE_W<PCH_PSTATE_SPEC, 0> {
        PCH_PSTATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "P-channel PSTATE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pch_pstate::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pch_pstate::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCH_PSTATE_SPEC;
impl crate::RegisterSpec for PCH_PSTATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pch_pstate::R`](R) reader structure"]
impl crate::Readable for PCH_PSTATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pch_pstate::W`](W) writer structure"]
impl crate::Writable for PCH_PSTATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pch_pstate to value 0"]
impl crate::Resettable for PCH_PSTATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
