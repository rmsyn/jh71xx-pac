#[doc = "Register `ioirq_7` reader"]
pub type R = crate::R<IOIRQ_7_SPEC>;
#[doc = "Register `ioirq_7` writer"]
pub type W = crate::W<IOIRQ_7_SPEC>;
#[doc = "Field `in_sync2` reader - Status of gpio_in after synchronization."]
pub type IN_SYNC2_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Status of gpio_in after synchronization."]
    #[inline(always)]
    pub fn in_sync2(&self) -> IN_SYNC2_R {
        IN_SYNC2_R::new((self.bits & 0x0f) as u8)
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
#[doc = "AON IOMUX CFG SAIF SYSCFG IOIRQ 44\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq_7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq_7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOIRQ_7_SPEC;
impl crate::RegisterSpec for IOIRQ_7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioirq_7::R`](R) reader structure"]
impl crate::Readable for IOIRQ_7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ioirq_7::W`](W) writer structure"]
impl crate::Writable for IOIRQ_7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ioirq_7 to value 0"]
impl crate::Resettable for IOIRQ_7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
