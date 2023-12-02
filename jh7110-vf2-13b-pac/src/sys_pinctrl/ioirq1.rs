#[doc = "Register `ioirq1` reader"]
pub type R = crate::R<IOIRQ1_SPEC>;
#[doc = "Register `ioirq1` writer"]
pub type W = crate::W<IOIRQ1_SPEC>;
#[doc = "Field `gpiois0` reader - 1: Edge trigger, 0: Level trigger"]
pub type GPIOIS0_R = crate::FieldReader<u32>;
#[doc = "Field `gpiois0` writer - 1: Edge trigger, 0: Level trigger"]
pub type GPIOIS0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 1: Edge trigger, 0: Level trigger"]
    #[inline(always)]
    pub fn gpiois0(&self) -> GPIOIS0_R {
        GPIOIS0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1: Edge trigger, 0: Level trigger"]
    #[inline(always)]
    #[must_use]
    pub fn gpiois0(&mut self) -> GPIOIS0_W<IOIRQ1_SPEC> {
        GPIOIS0_W::new(self, 0)
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
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 56: GPIO Interrupt Edge Trigger Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOIRQ1_SPEC;
impl crate::RegisterSpec for IOIRQ1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioirq1::R`](R) reader structure"]
impl crate::Readable for IOIRQ1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ioirq1::W`](W) writer structure"]
impl crate::Writable for IOIRQ1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ioirq1 to value 0"]
impl crate::Resettable for IOIRQ1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
