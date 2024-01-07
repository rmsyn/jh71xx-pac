#[doc = "Register `ioirq_13` reader"]
pub type R = crate::R<IOIRQ_13_SPEC>;
#[doc = "Register `ioirq_13` writer"]
pub type W = crate::W<IOIRQ_13_SPEC>;
#[doc = "Field `mis_0` reader - The masked GPIO IRQ status"]
pub type MIS_0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The masked GPIO IRQ status"]
    #[inline(always)]
    pub fn mis_0(&self) -> MIS_0_R {
        MIS_0_R::new(self.bits)
    }
}
impl W {
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
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 272: GPIO Masked Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq_13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq_13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOIRQ_13_SPEC;
impl crate::RegisterSpec for IOIRQ_13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioirq_13::R`](R) reader structure"]
impl crate::Readable for IOIRQ_13_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ioirq_13::W`](W) writer structure"]
impl crate::Writable for IOIRQ_13_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ioirq_13 to value 0"]
impl crate::Resettable for IOIRQ_13_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
