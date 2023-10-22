#[doc = "Register `ioirq12` reader"]
pub type R = crate::R<IOIRQ12_SPEC>;
#[doc = "Register `ioirq12` writer"]
pub type W = crate::W<IOIRQ12_SPEC>;
#[doc = "Field `gpioris1` reader - Status of the edge trigger. The register can be cleared by writing gpio ic"]
pub type GPIORIS1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Status of the edge trigger. The register can be cleared by writing gpio ic"]
    #[inline(always)]
    pub fn gpioris1(&self) -> GPIORIS1_R {
        GPIORIS1_R::new(self.bits)
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
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 67: GPIO Register Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOIRQ12_SPEC;
impl crate::RegisterSpec for IOIRQ12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioirq12::R`](R) reader structure"]
impl crate::Readable for IOIRQ12_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ioirq12::W`](W) writer structure"]
impl crate::Writable for IOIRQ12_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ioirq12 to value 0"]
impl crate::Resettable for IOIRQ12_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
