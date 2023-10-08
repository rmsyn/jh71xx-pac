#[doc = "Register `ssp_dr` reader"]
pub type R = crate::R<SSP_DR_SPEC>;
#[doc = "Register `ssp_dr` writer"]
pub type W = crate::W<SSP_DR_SPEC>;
#[doc = "Field `data` reader - Transmit/Receive FIFO - Read: Receive FIFO, Write: Transmit FIFO. You must right-justify data when the PrimeCell SSP is programmed for a data size that is less than 16 bits. Unused bits at the top are ignored by transmit logic. The receive logic automatically right-justifies."]
pub type DATA_R = crate::FieldReader<u16>;
#[doc = "Field `data` writer - Transmit/Receive FIFO - Read: Receive FIFO, Write: Transmit FIFO. You must right-justify data when the PrimeCell SSP is programmed for a data size that is less than 16 bits. Unused bits at the top are ignored by transmit logic. The receive logic automatically right-justifies."]
pub type DATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Transmit/Receive FIFO - Read: Receive FIFO, Write: Transmit FIFO. You must right-justify data when the PrimeCell SSP is programmed for a data size that is less than 16 bits. Unused bits at the top are ignored by transmit logic. The receive logic automatically right-justifies."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit/Receive FIFO - Read: Receive FIFO, Write: Transmit FIFO. You must right-justify data when the PrimeCell SSP is programmed for a data size that is less than 16 bits. Unused bits at the top are ignored by transmit logic. The receive logic automatically right-justifies."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<SSP_DR_SPEC, 0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SSPDR is the data register and is 16-bits wide. When SSPDR is read, the entry in the receive FIFO, pointed to by the current FIFO read pointer, is accessed. As data values are removed by the PrimeCell SSP receive logic from the incoming data frame, they are placed into the entry in the receive FIFO, pointed to by the current FIFO write pointer. When SSPDR is written to, the entry in the transmit FIFO, pointed to by the write pointer, is written to. Data values are removed from the transmit FIFO one value at a time by the transmit logic. It is loaded into the transmit serial shifter, then serially shifted out onto the SSPTXD pin at the programmed bit rate. When a data size of less than 16 bits is selected, the user must right-justify data written to the transmit FIFO. The transmit logic ignores the unused bits. Received data less than 16 bits is automatically right-justified in the receive buffer.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp_dr::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssp_dr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSP_DR_SPEC;
impl crate::RegisterSpec for SSP_DR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ssp_dr::R`](R) reader structure"]
impl crate::Readable for SSP_DR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ssp_dr::W`](W) writer structure"]
impl crate::Writable for SSP_DR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
