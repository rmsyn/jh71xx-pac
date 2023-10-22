#[doc = "Register `ioirq7` reader"]
pub type R = crate::R<IOIRQ7_SPEC>;
#[doc = "Register `ioirq7` writer"]
pub type W = crate::W<IOIRQ7_SPEC>;
#[doc = "Field `gpioiev0` reader - 1: Positive/Low, 0: Negative/High"]
pub type GPIOIEV0_R = crate::FieldReader<u32>;
#[doc = "Field `gpioiev0` writer - 1: Positive/Low, 0: Negative/High"]
pub type GPIOIEV0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - 1: Positive/Low, 0: Negative/High"]
    #[inline(always)]
    pub fn gpioiev0(&self) -> GPIOIEV0_R {
        GPIOIEV0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1: Positive/Low, 0: Negative/High"]
    #[inline(always)]
    #[must_use]
    pub fn gpioiev0(&mut self) -> GPIOIEV0_W<IOIRQ7_SPEC, 0> {
        GPIOIEV0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 62: GPIO Interrupt Edge Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOIRQ7_SPEC;
impl crate::RegisterSpec for IOIRQ7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioirq7::R`](R) reader structure"]
impl crate::Readable for IOIRQ7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ioirq7::W`](W) writer structure"]
impl crate::Writable for IOIRQ7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ioirq7 to value 0"]
impl crate::Resettable for IOIRQ7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
