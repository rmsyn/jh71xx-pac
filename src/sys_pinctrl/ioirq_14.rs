#[doc = "Register `ioirq_14` reader"]
pub type R = crate::R<IOIRQ_14_SPEC>;
#[doc = "Register `ioirq_14` writer"]
pub type W = crate::W<IOIRQ_14_SPEC>;
#[doc = "Field `mis_1` reader - The masked GPIO IRQ status"]
pub type MIS_1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The masked GPIO IRQ status"]
    #[inline(always)]
    pub fn mis_1(&self) -> MIS_1_R {
        MIS_1_R::new(self.bits)
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
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 276: GPIO Masked Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq_14::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq_14::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOIRQ_14_SPEC;
impl crate::RegisterSpec for IOIRQ_14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioirq_14::R`](R) reader structure"]
impl crate::Readable for IOIRQ_14_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ioirq_14::W`](W) writer structure"]
impl crate::Writable for IOIRQ_14_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ioirq_14 to value 0"]
impl crate::Resettable for IOIRQ_14_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
