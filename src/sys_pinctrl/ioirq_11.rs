#[doc = "Register `ioirq_11` reader"]
pub type R = crate::R<IOIRQ_11_SPEC>;
#[doc = "Register `ioirq_11` writer"]
pub type W = crate::W<IOIRQ_11_SPEC>;
#[doc = "Field `ris_0` reader - Status of the edge trigger. The register can be cleared by writing gpio ic"]
pub type RIS_0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Status of the edge trigger. The register can be cleared by writing gpio ic"]
    #[inline(always)]
    pub fn ris_0(&self) -> RIS_0_R {
        RIS_0_R::new(self.bits)
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
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 264: GPIO Register Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq_11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq_11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOIRQ_11_SPEC;
impl crate::RegisterSpec for IOIRQ_11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioirq_11::R`](R) reader structure"]
impl crate::Readable for IOIRQ_11_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ioirq_11::W`](W) writer structure"]
impl crate::Writable for IOIRQ_11_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ioirq_11 to value 0"]
impl crate::Resettable for IOIRQ_11_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
