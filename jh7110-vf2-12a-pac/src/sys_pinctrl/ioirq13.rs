#[doc = "Register `ioirq13` reader"]
pub type R = crate::R<IOIRQ13_SPEC>;
#[doc = "Register `ioirq13` writer"]
pub type W = crate::W<IOIRQ13_SPEC>;
#[doc = "Field `gpiomis0` reader - The masked GPIO IRQ status"]
pub type GPIOMIS0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The masked GPIO IRQ status"]
    #[inline(always)]
    pub fn gpiomis0(&self) -> GPIOMIS0_R {
        GPIOMIS0_R::new(self.bits)
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
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 68: GPIO Masked Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOIRQ13_SPEC;
impl crate::RegisterSpec for IOIRQ13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioirq13::R`](R) reader structure"]
impl crate::Readable for IOIRQ13_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ioirq13::W`](W) writer structure"]
impl crate::Writable for IOIRQ13_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ioirq13 to value 0"]
impl crate::Resettable for IOIRQ13_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
