#[doc = "Register `txflr` reader"]
pub type R = crate::R<TXFLR_SPEC>;
#[doc = "Register `txflr` writer"]
pub type W = crate::W<TXFLR_SPEC>;
#[doc = "Field `txflr` reader - txflr"]
pub type TXFLR_R = crate::FieldReader<u32>;
#[doc = "Field `txflr` writer - txflr"]
pub type TXFLR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - txflr"]
    #[inline(always)]
    pub fn txflr(&self) -> TXFLR_R {
        TXFLR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - txflr"]
    #[inline(always)]
    #[must_use]
    pub fn txflr(&mut self) -> TXFLR_W<TXFLR_SPEC> {
        TXFLR_W::new(self, 0)
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
#[doc = "DesignWare I2C TX Failure\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txflr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txflr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXFLR_SPEC;
impl crate::RegisterSpec for TXFLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txflr::R`](R) reader structure"]
impl crate::Readable for TXFLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txflr::W`](W) writer structure"]
impl crate::Writable for TXFLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets txflr to value 0"]
impl crate::Resettable for TXFLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
