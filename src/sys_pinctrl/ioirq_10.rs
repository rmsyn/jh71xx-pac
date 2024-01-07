#[doc = "Register `ioirq_10` reader"]
pub type R = crate::R<IOIRQ_10_SPEC>;
#[doc = "Register `ioirq_10` writer"]
pub type W = crate::W<IOIRQ_10_SPEC>;
#[doc = "Field `ie_1` reader - 1: Unmask, 0: Mask"]
pub type IE_1_R = crate::FieldReader<u32>;
#[doc = "Field `ie_1` writer - 1: Unmask, 0: Mask"]
pub type IE_1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 1: Unmask, 0: Mask"]
    #[inline(always)]
    pub fn ie_1(&self) -> IE_1_R {
        IE_1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1: Unmask, 0: Mask"]
    #[inline(always)]
    #[must_use]
    pub fn ie_1(&mut self) -> IE_1_W<IOIRQ_10_SPEC> {
        IE_1_W::new(self, 0)
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
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 260: GPIO Interrupt Edge Mask Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq_10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq_10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOIRQ_10_SPEC;
impl crate::RegisterSpec for IOIRQ_10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioirq_10::R`](R) reader structure"]
impl crate::Readable for IOIRQ_10_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ioirq_10::W`](W) writer structure"]
impl crate::Writable for IOIRQ_10_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ioirq_10 to value 0"]
impl crate::Resettable for IOIRQ_10_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
