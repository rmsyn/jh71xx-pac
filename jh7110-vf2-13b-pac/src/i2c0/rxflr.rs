#[doc = "Register `rxflr` reader"]
pub type R = crate::R<RXFLR_SPEC>;
#[doc = "Register `rxflr` writer"]
pub type W = crate::W<RXFLR_SPEC>;
#[doc = "Field `rxflr` reader - rxflr"]
pub type RXFLR_R = crate::FieldReader<u32>;
#[doc = "Field `rxflr` writer - rxflr"]
pub type RXFLR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - rxflr"]
    #[inline(always)]
    pub fn rxflr(&self) -> RXFLR_R {
        RXFLR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - rxflr"]
    #[inline(always)]
    #[must_use]
    pub fn rxflr(&mut self) -> RXFLR_W<RXFLR_SPEC, 0> {
        RXFLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DesignWare I2C RX Failure\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxflr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxflr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXFLR_SPEC;
impl crate::RegisterSpec for RXFLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxflr::R`](R) reader structure"]
impl crate::Readable for RXFLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxflr::W`](W) writer structure"]
impl crate::Writable for RXFLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rxflr to value 0"]
impl crate::Resettable for RXFLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
