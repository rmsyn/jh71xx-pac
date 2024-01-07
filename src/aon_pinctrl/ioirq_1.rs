#[doc = "Register `ioirq_1` reader"]
pub type R = crate::R<IOIRQ_1_SPEC>;
#[doc = "Register `ioirq_1` writer"]
pub type W = crate::W<IOIRQ_1_SPEC>;
#[doc = "Field `ic` reader - 1: Do not clear the register, 0: Clear the register"]
pub type IC_R = crate::FieldReader;
#[doc = "Field `ic` writer - 1: Do not clear the register, 0: Clear the register"]
pub type IC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 1: Do not clear the register, 0: Clear the register"]
    #[inline(always)]
    pub fn ic(&self) -> IC_R {
        IC_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1: Do not clear the register, 0: Clear the register"]
    #[inline(always)]
    #[must_use]
    pub fn ic(&mut self) -> IC_W<IOIRQ_1_SPEC> {
        IC_W::new(self, 0)
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
#[doc = "AON IOMUX CFG SAIF SYSCFG IOIRQ 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOIRQ_1_SPEC;
impl crate::RegisterSpec for IOIRQ_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioirq_1::R`](R) reader structure"]
impl crate::Readable for IOIRQ_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ioirq_1::W`](W) writer structure"]
impl crate::Writable for IOIRQ_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ioirq_1 to value 0"]
impl crate::Resettable for IOIRQ_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
