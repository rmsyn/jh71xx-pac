#[doc = "Register `ioirq15` reader"]
pub type R = crate::R<IOIRQ15_SPEC>;
#[doc = "Register `ioirq15` writer"]
pub type W = crate::W<IOIRQ15_SPEC>;
#[doc = "Field `gpio_in_sync2_0` reader - Status of the gpio_in after synchronization"]
pub type GPIO_IN_SYNC2_0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Status of the gpio_in after synchronization"]
    #[inline(always)]
    pub fn gpio_in_sync2_0(&self) -> GPIO_IN_SYNC2_0_R {
        GPIO_IN_SYNC2_0_R::new(self.bits)
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
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 70: GPIO Synchronization Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq15::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq15::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOIRQ15_SPEC;
impl crate::RegisterSpec for IOIRQ15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioirq15::R`](R) reader structure"]
impl crate::Readable for IOIRQ15_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ioirq15::W`](W) writer structure"]
impl crate::Writable for IOIRQ15_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ioirq15 to value 0"]
impl crate::Resettable for IOIRQ15_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
