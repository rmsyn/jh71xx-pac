#[doc = "Register `ioirq_5` reader"]
pub type R = crate::R<IOIRQ_5_SPEC>;
#[doc = "Register `ioirq_5` writer"]
pub type W = crate::W<IOIRQ_5_SPEC>;
#[doc = "Field `ris` reader - Status of the edge trigger, can be cleared by writing gpioic."]
pub type RIS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Status of the edge trigger, can be cleared by writing gpioic."]
    #[inline(always)]
    pub fn ris(&self) -> RIS_R {
        RIS_R::new((self.bits & 0x0f) as u8)
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
#[doc = "AON IOMUX CFG SAIF SYSCFG IOIRQ 36\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq_5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq_5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOIRQ_5_SPEC;
impl crate::RegisterSpec for IOIRQ_5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioirq_5::R`](R) reader structure"]
impl crate::Readable for IOIRQ_5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ioirq_5::W`](W) writer structure"]
impl crate::Writable for IOIRQ_5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ioirq_5 to value 0"]
impl crate::Resettable for IOIRQ_5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
