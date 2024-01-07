#[doc = "Register `ioirq_4` reader"]
pub type R = crate::R<IOIRQ_4_SPEC>;
#[doc = "Register `ioirq_4` writer"]
pub type W = crate::W<IOIRQ_4_SPEC>;
#[doc = "Field `ic_1` reader - 1: Do not clear the register, 0: Clear the register"]
pub type IC_1_R = crate::FieldReader<u32>;
#[doc = "Field `ic_1` writer - 1: Do not clear the register, 0: Clear the register"]
pub type IC_1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 1: Do not clear the register, 0: Clear the register"]
    #[inline(always)]
    pub fn ic_1(&self) -> IC_1_R {
        IC_1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1: Do not clear the register, 0: Clear the register"]
    #[inline(always)]
    #[must_use]
    pub fn ic_1(&mut self) -> IC_1_W<IOIRQ_4_SPEC> {
        IC_1_W::new(self, 0)
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
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 236: GPIO Interrupt Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq_4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq_4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOIRQ_4_SPEC;
impl crate::RegisterSpec for IOIRQ_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioirq_4::R`](R) reader structure"]
impl crate::Readable for IOIRQ_4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ioirq_4::W`](W) writer structure"]
impl crate::Writable for IOIRQ_4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ioirq_4 to value 0"]
impl crate::Resettable for IOIRQ_4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
