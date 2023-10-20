#[doc = "Register `rx_tl` reader"]
pub type R = crate::R<RX_TL_SPEC>;
#[doc = "Register `rx_tl` writer"]
pub type W = crate::W<RX_TL_SPEC>;
#[doc = "Field `rx_tl` reader - rx_tl"]
pub type RX_TL_R = crate::FieldReader<u32>;
#[doc = "Field `rx_tl` writer - rx_tl"]
pub type RX_TL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - rx_tl"]
    #[inline(always)]
    pub fn rx_tl(&self) -> RX_TL_R {
        RX_TL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - rx_tl"]
    #[inline(always)]
    #[must_use]
    pub fn rx_tl(&mut self) -> RX_TL_W<RX_TL_SPEC, 0> {
        RX_TL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DesignWare I2C RX TL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_tl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_tl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_TL_SPEC;
impl crate::RegisterSpec for RX_TL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_tl::R`](R) reader structure"]
impl crate::Readable for RX_TL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx_tl::W`](W) writer structure"]
impl crate::Writable for RX_TL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rx_tl to value 0"]
impl crate::Resettable for RX_TL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
