#[doc = "Register `ioirq_5` reader"]
pub type R = crate::R<IOIRQ_5_SPEC>;
#[doc = "Register `ioirq_5` writer"]
pub type W = crate::W<IOIRQ_5_SPEC>;
#[doc = "Field `ibe_0` reader - 1: Trigger on both edges, 0: Trigger on a single edge"]
pub type IBE_0_R = crate::FieldReader<u32>;
#[doc = "Field `ibe_0` writer - 1: Trigger on both edges, 0: Trigger on a single edge"]
pub type IBE_0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 1: Trigger on both edges, 0: Trigger on a single edge"]
    #[inline(always)]
    pub fn ibe_0(&self) -> IBE_0_R {
        IBE_0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1: Trigger on both edges, 0: Trigger on a single edge"]
    #[inline(always)]
    #[must_use]
    pub fn ibe_0(&mut self) -> IBE_0_W<IOIRQ_5_SPEC> {
        IBE_0_W::new(self, 0)
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
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 240: GPIO Interrupt Both Edge Trigger Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq_5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq_5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
