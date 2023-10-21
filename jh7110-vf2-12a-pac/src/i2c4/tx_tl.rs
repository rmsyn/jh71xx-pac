#[doc = "Register `tx_tl` reader"]
pub type R = crate::R<TX_TL_SPEC>;
#[doc = "Register `tx_tl` writer"]
pub type W = crate::W<TX_TL_SPEC>;
#[doc = "Field `tx_tl` reader - tx_tl"]
pub type TX_TL_R = crate::FieldReader<u32>;
#[doc = "Field `tx_tl` writer - tx_tl"]
pub type TX_TL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - tx_tl"]
    #[inline(always)]
    pub fn tx_tl(&self) -> TX_TL_R {
        TX_TL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - tx_tl"]
    #[inline(always)]
    #[must_use]
    pub fn tx_tl(&mut self) -> TX_TL_W<TX_TL_SPEC, 0> {
        TX_TL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DesignWare I2C TX TL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_tl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_tl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_TL_SPEC;
impl crate::RegisterSpec for TX_TL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_tl::R`](R) reader structure"]
impl crate::Readable for TX_TL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_tl::W`](W) writer structure"]
impl crate::Writable for TX_TL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tx_tl to value 0"]
impl crate::Resettable for TX_TL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
