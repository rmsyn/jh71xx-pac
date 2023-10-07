#[doc = "Register `clr_tx_abrt` reader"]
pub type R = crate::R<CLR_TX_ABRT_SPEC>;
#[doc = "Register `clr_tx_abrt` writer"]
pub type W = crate::W<CLR_TX_ABRT_SPEC>;
#[doc = "Field `clr_tx_abrt` reader - clr_tx_abrt"]
pub type CLR_TX_ABRT_R = crate::FieldReader<u32>;
#[doc = "Field `clr_tx_abrt` writer - clr_tx_abrt"]
pub type CLR_TX_ABRT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - clr_tx_abrt"]
    #[inline(always)]
    pub fn clr_tx_abrt(&self) -> CLR_TX_ABRT_R {
        CLR_TX_ABRT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - clr_tx_abrt"]
    #[inline(always)]
    #[must_use]
    pub fn clr_tx_abrt(&mut self) -> CLR_TX_ABRT_W<CLR_TX_ABRT_SPEC, 0> {
        CLR_TX_ABRT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DesignWare I2C Clear TX Abort\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr_tx_abrt::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr_tx_abrt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLR_TX_ABRT_SPEC;
impl crate::RegisterSpec for CLR_TX_ABRT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr_tx_abrt::R`](R) reader structure"]
impl crate::Readable for CLR_TX_ABRT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clr_tx_abrt::W`](W) writer structure"]
impl crate::Writable for CLR_TX_ABRT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
