#[doc = "Register `gmac0_txd2` reader"]
pub type R = crate::R<GMAC0_TXD2_SPEC>;
#[doc = "Register `gmac0_txd2` writer"]
pub type W = crate::W<GMAC0_TXD2_SPEC>;
#[doc = "Field `value` reader - value"]
pub type VALUE_R = crate::FieldReader;
#[doc = "Field `value` writer - value"]
pub type VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - value"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - value"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<GMAC0_TXD2_SPEC> {
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
#[doc = "AON IOMUX CFG SAIF SYSCFG 128\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac0_txd2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac0_txd2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GMAC0_TXD2_SPEC;
impl crate::RegisterSpec for GMAC0_TXD2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac0_txd2::R`](R) reader structure"]
impl crate::Readable for GMAC0_TXD2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gmac0_txd2::W`](W) writer structure"]
impl crate::Writable for GMAC0_TXD2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gmac0_txd2 to value 0"]
impl crate::Resettable for GMAC0_TXD2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
