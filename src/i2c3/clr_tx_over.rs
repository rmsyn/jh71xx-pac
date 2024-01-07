#[doc = "Register `clr_tx_over` reader"]
pub type R = crate::R<CLR_TX_OVER_SPEC>;
#[doc = "Register `clr_tx_over` writer"]
pub type W = crate::W<CLR_TX_OVER_SPEC>;
#[doc = "Field `clr_tx_over` reader - clr_tx_over"]
pub type CLR_TX_OVER_R = crate::FieldReader<u32>;
#[doc = "Field `clr_tx_over` writer - clr_tx_over"]
pub type CLR_TX_OVER_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - clr_tx_over"]
    #[inline(always)]
    pub fn clr_tx_over(&self) -> CLR_TX_OVER_R {
        CLR_TX_OVER_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - clr_tx_over"]
    #[inline(always)]
    #[must_use]
    pub fn clr_tx_over(&mut self) -> CLR_TX_OVER_W<CLR_TX_OVER_SPEC> {
        CLR_TX_OVER_W::new(self, 0)
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
#[doc = "DesignWare I2C Clear TX Overrun\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr_tx_over::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr_tx_over::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLR_TX_OVER_SPEC;
impl crate::RegisterSpec for CLR_TX_OVER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr_tx_over::R`](R) reader structure"]
impl crate::Readable for CLR_TX_OVER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clr_tx_over::W`](W) writer structure"]
impl crate::Writable for CLR_TX_OVER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets clr_tx_over to value 0"]
impl crate::Resettable for CLR_TX_OVER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
