#[doc = "Register `pch_bypass` reader"]
pub type R = crate::R<PCH_BYPASS_SPEC>;
#[doc = "Register `pch_bypass` writer"]
pub type W = crate::W<PCH_BYPASS_SPEC>;
#[doc = "Field `pch_bypass` reader - Bypass P-channel. 0: enable p-channel, 1: bypass p-channel"]
pub type PCH_BYPASS_R = crate::BitReader;
#[doc = "Field `pch_bypass` writer - Bypass P-channel. 0: enable p-channel, 1: bypass p-channel"]
pub type PCH_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bypass P-channel. 0: enable p-channel, 1: bypass p-channel"]
    #[inline(always)]
    pub fn pch_bypass(&self) -> PCH_BYPASS_R {
        PCH_BYPASS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bypass P-channel. 0: enable p-channel, 1: bypass p-channel"]
    #[inline(always)]
    #[must_use]
    pub fn pch_bypass(&mut self) -> PCH_BYPASS_W<PCH_BYPASS_SPEC> {
        PCH_BYPASS_W::new(self, 0)
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
#[doc = "P-channel Bypass\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pch_bypass::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pch_bypass::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCH_BYPASS_SPEC;
impl crate::RegisterSpec for PCH_BYPASS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pch_bypass::R`](R) reader structure"]
impl crate::Readable for PCH_BYPASS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pch_bypass::W`](W) writer structure"]
impl crate::Writable for PCH_BYPASS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pch_bypass to value 0"]
impl crate::Resettable for PCH_BYPASS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
