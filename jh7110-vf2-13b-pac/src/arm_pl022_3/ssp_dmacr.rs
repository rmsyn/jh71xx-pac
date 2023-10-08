#[doc = "Register `ssp_dmacr` reader"]
pub type R = crate::R<SSP_DMACR_SPEC>;
#[doc = "Register `ssp_dmacr` writer"]
pub type W = crate::W<SSP_DMACR_SPEC>;
#[doc = "Field `rxdmae` reader - Receive DMA Enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
pub type RXDMAE_R = crate::BitReader;
#[doc = "Field `rxdmae` writer - Receive DMA Enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
pub type RXDMAE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `txdmae` reader - Transmit DMA Enable. If this bit is set to 1, DMA for the transmit FIFO is enabled."]
pub type TXDMAE_R = crate::BitReader;
#[doc = "Field `txdmae` writer - Transmit DMA Enable. If this bit is set to 1, DMA for the transmit FIFO is enabled."]
pub type TXDMAE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Receive DMA Enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
    #[inline(always)]
    pub fn rxdmae(&self) -> RXDMAE_R {
        RXDMAE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit DMA Enable. If this bit is set to 1, DMA for the transmit FIFO is enabled."]
    #[inline(always)]
    pub fn txdmae(&self) -> TXDMAE_R {
        TXDMAE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive DMA Enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn rxdmae(&mut self) -> RXDMAE_W<SSP_DMACR_SPEC, 0> {
        RXDMAE_W::new(self)
    }
    #[doc = "Bit 1 - Transmit DMA Enable. If this bit is set to 1, DMA for the transmit FIFO is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn txdmae(&mut self) -> TXDMAE_W<SSP_DMACR_SPEC, 1> {
        TXDMAE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "The SSPDMACR register is the DMA control register. It is a RW register. All the bits are cleared to 0 on reset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp_dmacr::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssp_dmacr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSP_DMACR_SPEC;
impl crate::RegisterSpec for SSP_DMACR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ssp_dmacr::R`](R) reader structure"]
impl crate::Readable for SSP_DMACR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ssp_dmacr::W`](W) writer structure"]
impl crate::Writable for SSP_DMACR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
