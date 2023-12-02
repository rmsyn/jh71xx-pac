#[doc = "Register `rfw` reader"]
pub type R = crate::R<RFW_SPEC>;
#[doc = "Register `rfw` writer"]
pub type W = crate::W<RFW_SPEC>;
#[doc = "Field `rfwd` writer - Receive FIFO Write Data. These bits are only valid when FIFO access mode is enabled (FAR\\[0\\]
is set to one). When FIFOs are implemented and enabled, the data that is written to the RFWD is pushed into the receive FIFO. Each consecutive write pushes the new data to the next write location in the receive FIFO. When FIFOs are not implemented or not enabled, the data that is written to the RFWD is pushed into the RBR."]
pub type RFWD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `rfpe` writer - Receive FIFO Parity Error. These bits are only valid when FIFO access mode is enabled (FAR\\[0\\]
is set to one). When FIFOs are implemented and enabled, this bit is used to write parity error detection information to the receive FIFO. When FIFOs are not implemented or not enabled, this bit is used to write parity error detection information to the RBR."]
pub type RFPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rffe` writer - Receive FIFO Framing Error. These bits are only valid when FIFO access mode is enabled (FAR\\[0\\]
is set to one). When FIFOs are implemented and enabled, this bit is used to write framing error detection information to the receive FIFO. When FIFOs are not implemented or not enabled, this bit is used to write framing error detection information to the RBR."]
pub type RFFE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:7 - Receive FIFO Write Data. These bits are only valid when FIFO access mode is enabled (FAR\\[0\\]
is set to one). When FIFOs are implemented and enabled, the data that is written to the RFWD is pushed into the receive FIFO. Each consecutive write pushes the new data to the next write location in the receive FIFO. When FIFOs are not implemented or not enabled, the data that is written to the RFWD is pushed into the RBR."]
    #[inline(always)]
    #[must_use]
    pub fn rfwd(&mut self) -> RFWD_W<RFW_SPEC> {
        RFWD_W::new(self, 0)
    }
    #[doc = "Bit 8 - Receive FIFO Parity Error. These bits are only valid when FIFO access mode is enabled (FAR\\[0\\]
is set to one). When FIFOs are implemented and enabled, this bit is used to write parity error detection information to the receive FIFO. When FIFOs are not implemented or not enabled, this bit is used to write parity error detection information to the RBR."]
    #[inline(always)]
    #[must_use]
    pub fn rfpe(&mut self) -> RFPE_W<RFW_SPEC> {
        RFPE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Receive FIFO Framing Error. These bits are only valid when FIFO access mode is enabled (FAR\\[0\\]
is set to one). When FIFOs are implemented and enabled, this bit is used to write framing error detection information to the receive FIFO. When FIFOs are not implemented or not enabled, this bit is used to write framing error detection information to the RBR."]
    #[inline(always)]
    #[must_use]
    pub fn rffe(&mut self) -> RFFE_W<RFW_SPEC> {
        RFFE_W::new(self, 9)
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
#[doc = "Receive FIFO Write\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RFW_SPEC;
impl crate::RegisterSpec for RFW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfw::R`](R) reader structure"]
impl crate::Readable for RFW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rfw::W`](W) writer structure"]
impl crate::Writable for RFW_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rfw to value 0"]
impl crate::Resettable for RFW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
