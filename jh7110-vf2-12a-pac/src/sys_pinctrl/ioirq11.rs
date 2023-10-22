#[doc = "Register `ioirq11` reader"]
pub type R = crate::R<IOIRQ11_SPEC>;
#[doc = "Register `ioirq11` writer"]
pub type W = crate::W<IOIRQ11_SPEC>;
#[doc = "Field `gpioris0` reader - Status of the edge trigger. The register can be cleared by writing gpio ic"]
pub type GPIORIS0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Status of the edge trigger. The register can be cleared by writing gpio ic"]
    #[inline(always)]
    pub fn gpioris0(&self) -> GPIORIS0_R {
        GPIORIS0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 66: GPIO Register Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOIRQ11_SPEC;
impl crate::RegisterSpec for IOIRQ11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioirq11::R`](R) reader structure"]
impl crate::Readable for IOIRQ11_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ioirq11::W`](W) writer structure"]
impl crate::Writable for IOIRQ11_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ioirq11 to value 0"]
impl crate::Resettable for IOIRQ11_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
