#[doc = "Register `ioirq_6` reader"]
pub type R = crate::R<IOIRQ_6_SPEC>;
#[doc = "Register `ioirq_6` writer"]
pub type W = crate::W<IOIRQ_6_SPEC>;
#[doc = "Field `ibe_1` reader - 1: Trigger on both edges, 0: Trigger on a single edge"]
pub type IBE_1_R = crate::FieldReader<u32>;
#[doc = "Field `ibe_1` writer - 1: Trigger on both edges, 0: Trigger on a single edge"]
pub type IBE_1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 1: Trigger on both edges, 0: Trigger on a single edge"]
    #[inline(always)]
    pub fn ibe_1(&self) -> IBE_1_R {
        IBE_1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1: Trigger on both edges, 0: Trigger on a single edge"]
    #[inline(always)]
    #[must_use]
    pub fn ibe_1(&mut self) -> IBE_1_W<IOIRQ_6_SPEC> {
        IBE_1_W::new(self, 0)
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
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 244: GPIO Interrupt Both Edge Trigger Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq_6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq_6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOIRQ_6_SPEC;
impl crate::RegisterSpec for IOIRQ_6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioirq_6::R`](R) reader structure"]
impl crate::Readable for IOIRQ_6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ioirq_6::W`](W) writer structure"]
impl crate::Writable for IOIRQ_6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ioirq_6 to value 0"]
impl crate::Resettable for IOIRQ_6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
