#[doc = "Register `gmac0_rxd0` reader"]
pub type R = crate::R<GMAC0_RXD0_SPEC>;
#[doc = "Register `gmac0_rxd0` writer"]
pub type W = crate::W<GMAC0_RXD0_SPEC>;
#[doc = "Field `value` reader - 0: GMAC0 IO voltage select 3.3V, 1: GMAC0 IO voltage select 2.5V, 2: GMAC0 IO voltage select 1.8V"]
pub type VALUE_R = crate::FieldReader;
#[doc = "Field `value` writer - 0: GMAC0 IO voltage select 3.3V, 1: GMAC0 IO voltage select 2.5V, 2: GMAC0 IO voltage select 1.8V"]
pub type VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 0: GMAC0 IO voltage select 3.3V, 1: GMAC0 IO voltage select 2.5V, 2: GMAC0 IO voltage select 1.8V"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 0: GMAC0 IO voltage select 3.3V, 1: GMAC0 IO voltage select 2.5V, 2: GMAC0 IO voltage select 1.8V"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<GMAC0_RXD0_SPEC> {
        VALUE_W::new(self, 0)
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
#[doc = "AON IOMUX CFG SAIF SYSCFG 96\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac0_rxd0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac0_rxd0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GMAC0_RXD0_SPEC;
impl crate::RegisterSpec for GMAC0_RXD0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac0_rxd0::R`](R) reader structure"]
impl crate::Readable for GMAC0_RXD0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gmac0_rxd0::W`](W) writer structure"]
impl crate::Writable for GMAC0_RXD0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gmac0_rxd0 to value 0"]
impl crate::Resettable for GMAC0_RXD0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
