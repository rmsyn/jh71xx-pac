#[doc = "Register `ioirq3` reader"]
pub type R = crate::R<IOIRQ3_SPEC>;
#[doc = "Register `ioirq3` writer"]
pub type W = crate::W<IOIRQ3_SPEC>;
#[doc = "Field `gpioic0` reader - 1: Do not clear the register, 0: Clear the register"]
pub type GPIOIC0_R = crate::FieldReader<u32>;
#[doc = "Field `gpioic0` writer - 1: Do not clear the register, 0: Clear the register"]
pub type GPIOIC0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 1: Do not clear the register, 0: Clear the register"]
    #[inline(always)]
    pub fn gpioic0(&self) -> GPIOIC0_R {
        GPIOIC0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1: Do not clear the register, 0: Clear the register"]
    #[inline(always)]
    #[must_use]
    pub fn gpioic0(&mut self) -> GPIOIC0_W<IOIRQ3_SPEC> {
        GPIOIC0_W::new(self, 0)
    }
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
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 58: GPIO Interrupt Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOIRQ3_SPEC;
impl crate::RegisterSpec for IOIRQ3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioirq3::R`](R) reader structure"]
impl crate::Readable for IOIRQ3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ioirq3::W`](W) writer structure"]
impl crate::Writable for IOIRQ3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ioirq3 to value 0"]
impl crate::Resettable for IOIRQ3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
