#[doc = "Register `fcr` reader"]
pub type R = crate::R<FCR_SPEC>;
#[doc = "Register `fcr` writer"]
pub type W = crate::W<FCR_SPEC>;
#[doc = "Field `fifoe` writer - FIFO Enable. This enables/disables the transmit (XMIT) and receive (RCVR) FIFOs. Whenever the value of this bit is changed both the XMIT and RCVR controller portion of FIFOs is reset."]
pub type FIFOE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rfifor` writer - RCVR FIFO Reset. This resets the control portion of the receive FIFO and treats the FIFO as empty. This also de-asserts the DMA RX request and single signals when additional DMA handshaking signals are selected (DMA_EXTRA == YES). Note that this bit is 'self-clearing'. It is not necessary to clear this bit."]
pub type RFIFOR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `xfifor` writer - XMIT FIFO Reset. This resets the control portion of the transmit FIFO and treats the FIFO as empty. This also de-asserts the DMA TX request and single signals when additional DMA handshaking signals are selected (DMA_EXTRA == YES). Note that this bit is 'self-clearing'. It is not necessary to clear this bit."]
pub type XFIFOR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `dmam` writer - DMA Mode. This determines the DMA signalling mode used for the dma_tx_req_n and dma_rx_req_n output signals when additional DMA handshaking signals are not selected (DMA_EXTRA == No). For details on DMA support, refer to “DMA Support” on page 58. 0 = mode 0 1 = mode 1"]
pub type DMAM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `tet` writer - TX Empty Trigger. Writes have no effect when THRE_MODE_USER == Disabled. This is used to select the empty threshold level at which the THRE Interrupts are generated when the mode is active. It also determines when the dma_tx_req_n signal is asserted when in certain modes of operation. For details on DMA support, refer to “DMA Support” on page 58. The following trigger levels are supported: 00 = FIFO empty 01 = 2 characters in the FIFO 10 = FIFO ¼ full 11 = FIFO ½ full"]
pub type TET_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `rt` writer - RCVR Trigger. This is used to select the trigger level in the receiver FIFO at which the Received Data Available Interrupt is generated. In auto flow control mode it is used to determine when the rts_n signal is de-asserted. It also determines when the dma_rx_req_n signal is asserted in certain modes of operation. For details on DMA support, refer to “DMA Support” on page 58. The following trigger levels are supported: 00 = 1 character in the FIFO 01 = FIFO ¼ full 10 = FIFO ½ full 11 = FIFO 2 less than full"]
pub type RT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl W {
    #[doc = "Bit 0 - FIFO Enable. This enables/disables the transmit (XMIT) and receive (RCVR) FIFOs. Whenever the value of this bit is changed both the XMIT and RCVR controller portion of FIFOs is reset."]
    #[inline(always)]
    #[must_use]
    pub fn fifoe(&mut self) -> FIFOE_W<FCR_SPEC, 0> {
        FIFOE_W::new(self)
    }
    #[doc = "Bit 1 - RCVR FIFO Reset. This resets the control portion of the receive FIFO and treats the FIFO as empty. This also de-asserts the DMA RX request and single signals when additional DMA handshaking signals are selected (DMA_EXTRA == YES). Note that this bit is 'self-clearing'. It is not necessary to clear this bit."]
    #[inline(always)]
    #[must_use]
    pub fn rfifor(&mut self) -> RFIFOR_W<FCR_SPEC, 1> {
        RFIFOR_W::new(self)
    }
    #[doc = "Bit 2 - XMIT FIFO Reset. This resets the control portion of the transmit FIFO and treats the FIFO as empty. This also de-asserts the DMA TX request and single signals when additional DMA handshaking signals are selected (DMA_EXTRA == YES). Note that this bit is 'self-clearing'. It is not necessary to clear this bit."]
    #[inline(always)]
    #[must_use]
    pub fn xfifor(&mut self) -> XFIFOR_W<FCR_SPEC, 2> {
        XFIFOR_W::new(self)
    }
    #[doc = "Bit 3 - DMA Mode. This determines the DMA signalling mode used for the dma_tx_req_n and dma_rx_req_n output signals when additional DMA handshaking signals are not selected (DMA_EXTRA == No). For details on DMA support, refer to “DMA Support” on page 58. 0 = mode 0 1 = mode 1"]
    #[inline(always)]
    #[must_use]
    pub fn dmam(&mut self) -> DMAM_W<FCR_SPEC, 3> {
        DMAM_W::new(self)
    }
    #[doc = "Bits 4:5 - TX Empty Trigger. Writes have no effect when THRE_MODE_USER == Disabled. This is used to select the empty threshold level at which the THRE Interrupts are generated when the mode is active. It also determines when the dma_tx_req_n signal is asserted when in certain modes of operation. For details on DMA support, refer to “DMA Support” on page 58. The following trigger levels are supported: 00 = FIFO empty 01 = 2 characters in the FIFO 10 = FIFO ¼ full 11 = FIFO ½ full"]
    #[inline(always)]
    #[must_use]
    pub fn tet(&mut self) -> TET_W<FCR_SPEC, 4> {
        TET_W::new(self)
    }
    #[doc = "Bits 6:7 - RCVR Trigger. This is used to select the trigger level in the receiver FIFO at which the Received Data Available Interrupt is generated. In auto flow control mode it is used to determine when the rts_n signal is de-asserted. It also determines when the dma_rx_req_n signal is asserted in certain modes of operation. For details on DMA support, refer to “DMA Support” on page 58. The following trigger levels are supported: 00 = 1 character in the FIFO 01 = FIFO ¼ full 10 = FIFO ½ full 11 = FIFO 2 less than full"]
    #[inline(always)]
    #[must_use]
    pub fn rt(&mut self) -> RT_W<FCR_SPEC, 6> {
        RT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "FIFO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCR_SPEC;
impl crate::RegisterSpec for FCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcr::R`](R) reader structure"]
impl crate::Readable for FCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fcr::W`](W) writer structure"]
impl crate::Writable for FCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets fcr to value 0"]
impl crate::Resettable for FCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
