#[doc = "Register `ssp_sr` reader"]
pub type R = crate::R<SSP_SR_SPEC>;
#[doc = "Register `ssp_sr` writer"]
pub type W = crate::W<SSP_SR_SPEC>;
#[doc = "Field `tfe` reader - Transmit FIFO empty, RO - 0: Transmit FIFO is not empty, 1: Transmit FIFO is empty."]
pub type TFE_R = crate::BitReader;
#[doc = "Field `tnf` reader - Transmit FIFO not full, RO - 0: Transmit FIFO is full, 1: Transmit FIFO is not full."]
pub type TNF_R = crate::BitReader;
#[doc = "Field `rne` reader - Receive FIFO not empty, RO - 0: Receive FIFO is empty, 1: Receive FIFO is not empty."]
pub type RNE_R = crate::BitReader;
#[doc = "Field `rff` reader - Receive FIFO full, RO - 0: Receive FIFO is not full, 1: Receive FIFO is full."]
pub type RFF_R = crate::BitReader;
#[doc = "Field `bsy` reader - PrimeCell SSP busy flag, RO - 0: SSP is idle, 1: SSP is currently transmitting and/or receiving a frame or the transmit FIFO is not empty."]
pub type BSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmit FIFO empty, RO - 0: Transmit FIFO is not empty, 1: Transmit FIFO is empty."]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO not full, RO - 0: Transmit FIFO is full, 1: Transmit FIFO is not full."]
    #[inline(always)]
    pub fn tnf(&self) -> TNF_R {
        TNF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO not empty, RO - 0: Receive FIFO is empty, 1: Receive FIFO is not empty."]
    #[inline(always)]
    pub fn rne(&self) -> RNE_R {
        RNE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO full, RO - 0: Receive FIFO is not full, 1: Receive FIFO is full."]
    #[inline(always)]
    pub fn rff(&self) -> RFF_R {
        RFF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PrimeCell SSP busy flag, RO - 0: SSP is idle, 1: SSP is currently transmitting and/or receiving a frame or the transmit FIFO is not empty."]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SSPSR is a RO status register that contains bits that indicate the FIFO fill status and the PrimeCell SSP busy status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp_sr::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssp_sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSP_SR_SPEC;
impl crate::RegisterSpec for SSP_SR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ssp_sr::R`](R) reader structure"]
impl crate::Readable for SSP_SR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ssp_sr::W`](W) writer structure"]
impl crate::Writable for SSP_SR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
