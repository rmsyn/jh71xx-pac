#[doc = "Register `ssp_mis` reader"]
pub type R = crate::R<SSP_MIS_SPEC>;
#[doc = "Register `ssp_mis` writer"]
pub type W = crate::W<SSP_MIS_SPEC>;
#[doc = "Field `rormis` reader - Gives the receive over run masked interrupt status, after masking, of the SSPRORINTR interrupt"]
pub type RORMIS_R = crate::BitReader;
#[doc = "Field `rtmis` reader - Gives the receive timeout masked interrupt state, after masking, of the SSPRTINTR interrupt"]
pub type RTMIS_R = crate::BitReader;
#[doc = "Field `rxmis` reader - Gives the receive FIFO masked interrupt state, after masking, of the SSPRXINTR interrupt"]
pub type RXMIS_R = crate::BitReader;
#[doc = "Field `txmis` reader - Gives the transmit FIFO masked interrupt state, after masking, of the SSPTXINTR interrupt"]
pub type TXMIS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Gives the receive over run masked interrupt status, after masking, of the SSPRORINTR interrupt"]
    #[inline(always)]
    pub fn rormis(&self) -> RORMIS_R {
        RORMIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Gives the receive timeout masked interrupt state, after masking, of the SSPRTINTR interrupt"]
    #[inline(always)]
    pub fn rtmis(&self) -> RTMIS_R {
        RTMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Gives the receive FIFO masked interrupt state, after masking, of the SSPRXINTR interrupt"]
    #[inline(always)]
    pub fn rxmis(&self) -> RXMIS_R {
        RXMIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Gives the transmit FIFO masked interrupt state, after masking, of the SSPTXINTR interrupt"]
    #[inline(always)]
    pub fn txmis(&self) -> TXMIS_R {
        TXMIS_R::new(((self.bits >> 3) & 1) != 0)
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
#[doc = "The SSPMIS register is the masked interrupt status register. It is a RO register. On a read this register gives the current masked status value of the corresponding interrupt. A write has no effect.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp_mis::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssp_mis::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSP_MIS_SPEC;
impl crate::RegisterSpec for SSP_MIS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ssp_mis::R`](R) reader structure"]
impl crate::Readable for SSP_MIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ssp_mis::W`](W) writer structure"]
impl crate::Writable for SSP_MIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
