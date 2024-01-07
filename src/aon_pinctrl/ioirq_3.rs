#[doc = "Register `ioirq_3` reader"]
pub type R = crate::R<IOIRQ_3_SPEC>;
#[doc = "Register `ioirq_3` writer"]
pub type W = crate::W<IOIRQ_3_SPEC>;
#[doc = "Field `iev` reader - 1: Positive/Low, 0: Negative/High"]
pub type IEV_R = crate::FieldReader;
#[doc = "Field `iev` writer - 1: Positive/Low, 0: Negative/High"]
pub type IEV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 1: Positive/Low, 0: Negative/High"]
    #[inline(always)]
    pub fn iev(&self) -> IEV_R {
        IEV_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1: Positive/Low, 0: Negative/High"]
    #[inline(always)]
    #[must_use]
    pub fn iev(&mut self) -> IEV_W<IOIRQ_3_SPEC> {
        IEV_W::new(self, 0)
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
#[doc = "AON IOMUX CFG SAIF SYSCFG IOIRQ 28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOIRQ_3_SPEC;
impl crate::RegisterSpec for IOIRQ_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioirq_3::R`](R) reader structure"]
impl crate::Readable for IOIRQ_3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ioirq_3::W`](W) writer structure"]
impl crate::Writable for IOIRQ_3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ioirq_3 to value 0"]
impl crate::Resettable for IOIRQ_3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
