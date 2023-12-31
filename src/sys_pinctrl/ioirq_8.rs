#[doc = "Register `ioirq_8` reader"]
pub type R = crate::R<IOIRQ_8_SPEC>;
#[doc = "Register `ioirq_8` writer"]
pub type W = crate::W<IOIRQ_8_SPEC>;
#[doc = "Field `iev_1` reader - 1: Positive/Low, 0: Negative/High"]
pub type IEV_1_R = crate::FieldReader<u32>;
#[doc = "Field `iev_1` writer - 1: Positive/Low, 0: Negative/High"]
pub type IEV_1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 1: Positive/Low, 0: Negative/High"]
    #[inline(always)]
    pub fn iev_1(&self) -> IEV_1_R {
        IEV_1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1: Positive/Low, 0: Negative/High"]
    #[inline(always)]
    #[must_use]
    pub fn iev_1(&mut self) -> IEV_1_W<IOIRQ_8_SPEC> {
        IEV_1_W::new(self, 0)
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
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 252: GPIO Interrupt Edge Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq_8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq_8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOIRQ_8_SPEC;
impl crate::RegisterSpec for IOIRQ_8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioirq_8::R`](R) reader structure"]
impl crate::Readable for IOIRQ_8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ioirq_8::W`](W) writer structure"]
impl crate::Writable for IOIRQ_8_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ioirq_8 to value 0"]
impl crate::Resettable for IOIRQ_8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
