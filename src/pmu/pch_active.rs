#[doc = "Register `pch_active` reader"]
pub type R = crate::R<PCH_ACTIVE_SPEC>;
#[doc = "Register `pch_active` writer"]
pub type W = crate::W<PCH_ACTIVE_SPEC>;
#[doc = "Field `pch_active` reader - P-channel PACTIVE status."]
pub type PCH_ACTIVE_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:10 - P-channel PACTIVE status."]
    #[inline(always)]
    pub fn pch_active(&self) -> PCH_ACTIVE_R {
        PCH_ACTIVE_R::new((self.bits & 0x07ff) as u16)
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
#[doc = "P-channel PACTIVE Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pch_active::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pch_active::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCH_ACTIVE_SPEC;
impl crate::RegisterSpec for PCH_ACTIVE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pch_active::R`](R) reader structure"]
impl crate::Readable for PCH_ACTIVE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pch_active::W`](W) writer structure"]
impl crate::Writable for PCH_ACTIVE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pch_active to value 0"]
impl crate::Resettable for PCH_ACTIVE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
