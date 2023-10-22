#[doc = "Register `ioirq14` reader"]
pub type R = crate::R<IOIRQ14_SPEC>;
#[doc = "Register `ioirq14` writer"]
pub type W = crate::W<IOIRQ14_SPEC>;
#[doc = "Field `gpiomis1` reader - The masked GPIO IRQ status"]
pub type GPIOMIS1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The masked GPIO IRQ status"]
    #[inline(always)]
    pub fn gpiomis1(&self) -> GPIOMIS1_R {
        GPIOMIS1_R::new(self.bits)
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
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 69: GPIO Masked Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq14::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq14::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOIRQ14_SPEC;
impl crate::RegisterSpec for IOIRQ14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioirq14::R`](R) reader structure"]
impl crate::Readable for IOIRQ14_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ioirq14::W`](W) writer structure"]
impl crate::Writable for IOIRQ14_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ioirq14 to value 0"]
impl crate::Resettable for IOIRQ14_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
