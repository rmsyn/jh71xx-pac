#[doc = "Register `srr` reader"]
pub type R = crate::R<SRR_SPEC>;
#[doc = "Register `srr` writer"]
pub type W = crate::W<SRR_SPEC>;
#[doc = "Field `ur` writer - UART Reset. This asynchronously resets the DW_apb_uart and synchronously removes the reset assertion. For a two clock implementation both pclk and sclk domains are reset."]
pub type UR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rfr` writer - RCVR FIFO Reset. This is a shadow register for the RCVR FIFO Reset bit (FCR\\[1\\]). This can be used to remove the burden on software having to store previously written FCR values (which are pretty static) just to reset the receive FIFO This resets the control portion of the receive FIFO and treats the FIFO as empty. This also de-asserts the DMA RX request and single signals when additional DMA handshaking signals are selected (DMA_EXTRA == YES). Note that this bit is 'self-clearing'. It is not necessary to clear this bit."]
pub type RFR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `xfr` writer - XMIT FIFO Reset. This is a shadow register for the XMIT FIFO Reset bit (FCR\\[2\\]). This can be used to remove the burden on software having to store previously written FCR values (which are pretty static) just to reset the transmit FIFO. This resets the control portion of the transmit FIFO and treats the FIFO as empty. This also de-asserts the DMA TX request and single signals when additional DMA handshaking signals are selected (DMA_EXTRA == YES). Note that this bit is 'self-clearing'. It is not necessary to clear this bit."]
pub type XFR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - UART Reset. This asynchronously resets the DW_apb_uart and synchronously removes the reset assertion. For a two clock implementation both pclk and sclk domains are reset."]
    #[inline(always)]
    #[must_use]
    pub fn ur(&mut self) -> UR_W<SRR_SPEC, 0> {
        UR_W::new(self)
    }
    #[doc = "Bit 1 - RCVR FIFO Reset. This is a shadow register for the RCVR FIFO Reset bit (FCR\\[1\\]). This can be used to remove the burden on software having to store previously written FCR values (which are pretty static) just to reset the receive FIFO This resets the control portion of the receive FIFO and treats the FIFO as empty. This also de-asserts the DMA RX request and single signals when additional DMA handshaking signals are selected (DMA_EXTRA == YES). Note that this bit is 'self-clearing'. It is not necessary to clear this bit."]
    #[inline(always)]
    #[must_use]
    pub fn rfr(&mut self) -> RFR_W<SRR_SPEC, 1> {
        RFR_W::new(self)
    }
    #[doc = "Bit 2 - XMIT FIFO Reset. This is a shadow register for the XMIT FIFO Reset bit (FCR\\[2\\]). This can be used to remove the burden on software having to store previously written FCR values (which are pretty static) just to reset the transmit FIFO. This resets the control portion of the transmit FIFO and treats the FIFO as empty. This also de-asserts the DMA TX request and single signals when additional DMA handshaking signals are selected (DMA_EXTRA == YES). Note that this bit is 'self-clearing'. It is not necessary to clear this bit."]
    #[inline(always)]
    #[must_use]
    pub fn xfr(&mut self) -> XFR_W<SRR_SPEC, 2> {
        XFR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Software Reset Register: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRR_SPEC;
impl crate::RegisterSpec for SRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srr::R`](R) reader structure"]
impl crate::Readable for SRR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`srr::W`](W) writer structure"]
impl crate::Writable for SRR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets srr to value 0"]
impl crate::Resettable for SRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
