#[doc = "Register `clr_rx_over` reader"]
pub type R = crate::R<CLR_RX_OVER_SPEC>;
#[doc = "Register `clr_rx_over` writer"]
pub type W = crate::W<CLR_RX_OVER_SPEC>;
#[doc = "Field `clr_rx_over` reader - clr_rx_over"]
pub type CLR_RX_OVER_R = crate::FieldReader<u32>;
#[doc = "Field `clr_rx_over` writer - clr_rx_over"]
pub type CLR_RX_OVER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - clr_rx_over"]
    #[inline(always)]
    pub fn clr_rx_over(&self) -> CLR_RX_OVER_R {
        CLR_RX_OVER_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - clr_rx_over"]
    #[inline(always)]
    #[must_use]
    pub fn clr_rx_over(&mut self) -> CLR_RX_OVER_W<CLR_RX_OVER_SPEC, 0> {
        CLR_RX_OVER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DesignWare I2C Clear RX Overrun\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr_rx_over::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr_rx_over::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLR_RX_OVER_SPEC;
impl crate::RegisterSpec for CLR_RX_OVER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr_rx_over::R`](R) reader structure"]
impl crate::Readable for CLR_RX_OVER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clr_rx_over::W`](W) writer structure"]
impl crate::Writable for CLR_RX_OVER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
