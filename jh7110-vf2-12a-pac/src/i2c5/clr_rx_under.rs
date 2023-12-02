#[doc = "Register `clr_rx_under` reader"]
pub type R = crate::R<CLR_RX_UNDER_SPEC>;
#[doc = "Register `clr_rx_under` writer"]
pub type W = crate::W<CLR_RX_UNDER_SPEC>;
#[doc = "Field `clr_rx_under` reader - clr_rx_under"]
pub type CLR_RX_UNDER_R = crate::FieldReader<u32>;
#[doc = "Field `clr_rx_under` writer - clr_rx_under"]
pub type CLR_RX_UNDER_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - clr_rx_under"]
    #[inline(always)]
    pub fn clr_rx_under(&self) -> CLR_RX_UNDER_R {
        CLR_RX_UNDER_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - clr_rx_under"]
    #[inline(always)]
    #[must_use]
    pub fn clr_rx_under(&mut self) -> CLR_RX_UNDER_W<CLR_RX_UNDER_SPEC> {
        CLR_RX_UNDER_W::new(self, 0)
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
#[doc = "DesignWare I2C Clear RX Underrun\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr_rx_under::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr_rx_under::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLR_RX_UNDER_SPEC;
impl crate::RegisterSpec for CLR_RX_UNDER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr_rx_under::R`](R) reader structure"]
impl crate::Readable for CLR_RX_UNDER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clr_rx_under::W`](W) writer structure"]
impl crate::Writable for CLR_RX_UNDER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets clr_rx_under to value 0"]
impl crate::Resettable for CLR_RX_UNDER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
