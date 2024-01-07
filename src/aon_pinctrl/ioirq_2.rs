#[doc = "Register `ioirq_2` reader"]
pub type R = crate::R<IOIRQ_2_SPEC>;
#[doc = "Register `ioirq_2` writer"]
pub type W = crate::W<IOIRQ_2_SPEC>;
#[doc = "Field `ibe` reader - 1: Trigger on both edges, 0: Trigger on a single edge"]
pub type IBE_R = crate::FieldReader;
#[doc = "Field `ibe` writer - 1: Trigger on both edges, 0: Trigger on a single edge"]
pub type IBE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 1: Trigger on both edges, 0: Trigger on a single edge"]
    #[inline(always)]
    pub fn ibe(&self) -> IBE_R {
        IBE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1: Trigger on both edges, 0: Trigger on a single edge"]
    #[inline(always)]
    #[must_use]
    pub fn ibe(&mut self) -> IBE_W<IOIRQ_2_SPEC> {
        IBE_W::new(self, 0)
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
#[doc = "AON IOMUX CFG SAIF SYSCFG IOIRQ 24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOIRQ_2_SPEC;
impl crate::RegisterSpec for IOIRQ_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioirq_2::R`](R) reader structure"]
impl crate::Readable for IOIRQ_2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ioirq_2::W`](W) writer structure"]
impl crate::Writable for IOIRQ_2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ioirq_2 to value 0"]
impl crate::Resettable for IOIRQ_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
