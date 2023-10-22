#[doc = "Register `ioirq4` reader"]
pub type R = crate::R<IOIRQ4_SPEC>;
#[doc = "Register `ioirq4` writer"]
pub type W = crate::W<IOIRQ4_SPEC>;
#[doc = "Field `gpioic1` reader - 1: Do not clear the register, 0: Clear the register"]
pub type GPIOIC1_R = crate::FieldReader<u32>;
#[doc = "Field `gpioic1` writer - 1: Do not clear the register, 0: Clear the register"]
pub type GPIOIC1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - 1: Do not clear the register, 0: Clear the register"]
    #[inline(always)]
    pub fn gpioic1(&self) -> GPIOIC1_R {
        GPIOIC1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1: Do not clear the register, 0: Clear the register"]
    #[inline(always)]
    #[must_use]
    pub fn gpioic1(&mut self) -> GPIOIC1_W<IOIRQ4_SPEC, 0> {
        GPIOIC1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 59: GPIO Interrupt Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOIRQ4_SPEC;
impl crate::RegisterSpec for IOIRQ4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioirq4::R`](R) reader structure"]
impl crate::Readable for IOIRQ4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ioirq4::W`](W) writer structure"]
impl crate::Writable for IOIRQ4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ioirq4 to value 0"]
impl crate::Resettable for IOIRQ4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
