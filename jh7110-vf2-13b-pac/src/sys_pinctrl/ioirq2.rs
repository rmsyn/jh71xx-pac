#[doc = "Register `ioirq2` reader"]
pub type R = crate::R<IOIRQ2_SPEC>;
#[doc = "Register `ioirq2` writer"]
pub type W = crate::W<IOIRQ2_SPEC>;
#[doc = "Field `gpiois1` reader - 1: Edge trigger, 0: Level trigger"]
pub type GPIOIS1_R = crate::FieldReader<u32>;
#[doc = "Field `gpiois1` writer - 1: Edge trigger, 0: Level trigger"]
pub type GPIOIS1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 1: Edge trigger, 0: Level trigger"]
    #[inline(always)]
    pub fn gpiois1(&self) -> GPIOIS1_R {
        GPIOIS1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1: Edge trigger, 0: Level trigger"]
    #[inline(always)]
    #[must_use]
    pub fn gpiois1(&mut self) -> GPIOIS1_W<IOIRQ2_SPEC> {
        GPIOIS1_W::new(self, 0)
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
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 57: GPIO Interrupt Edge Trigger Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOIRQ2_SPEC;
impl crate::RegisterSpec for IOIRQ2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioirq2::R`](R) reader structure"]
impl crate::Readable for IOIRQ2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ioirq2::W`](W) writer structure"]
impl crate::Writable for IOIRQ2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ioirq2 to value 0"]
impl crate::Resettable for IOIRQ2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
