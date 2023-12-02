#[doc = "Register `ssp_icr` reader"]
pub type R = crate::R<SSP_ICR_SPEC>;
#[doc = "Register `ssp_icr` writer"]
pub type W = crate::W<SSP_ICR_SPEC>;
#[doc = "Field `roric` reader - Clears the SSPRORINTR interrupt"]
pub type RORIC_R = crate::BitReader;
#[doc = "Field `roric` writer - Clears the SSPRORINTR interrupt"]
pub type RORIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rtic` reader - Clears the SSPRTINTR interrupt"]
pub type RTIC_R = crate::BitReader;
#[doc = "Field `rtic` writer - Clears the SSPRTINTR interrupt"]
pub type RTIC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clears the SSPRORINTR interrupt"]
    #[inline(always)]
    pub fn roric(&self) -> RORIC_R {
        RORIC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clears the SSPRTINTR interrupt"]
    #[inline(always)]
    pub fn rtic(&self) -> RTIC_R {
        RTIC_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clears the SSPRORINTR interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn roric(&mut self) -> RORIC_W<SSP_ICR_SPEC> {
        RORIC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clears the SSPRTINTR interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rtic(&mut self) -> RTIC_W<SSP_ICR_SPEC> {
        RTIC_W::new(self, 1)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "The SSPICR register is the interrupt clear register and is write-only. On a write of 1, the corresponding interrupt is cleared. A write of 0 has no effect.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp_icr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssp_icr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSP_ICR_SPEC;
impl crate::RegisterSpec for SSP_ICR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ssp_icr::R`](R) reader structure"]
impl crate::Readable for SSP_ICR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ssp_icr::W`](W) writer structure"]
impl crate::Writable for SSP_ICR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ssp_icr to value 0"]
impl crate::Resettable for SSP_ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
