#[doc = "Register `ssp_ris` reader"]
pub type R = crate::R<SSP_RIS_SPEC>;
#[doc = "Register `ssp_ris` writer"]
pub type W = crate::W<SSP_RIS_SPEC>;
#[doc = "Field `rorris` reader - Gives the raw interrupt state, prior to masking, of the SSPRORINTR interrupt"]
pub type RORRIS_R = crate::BitReader;
#[doc = "Field `rtris` reader - Gives the raw interrupt state, prior to masking, of the SSPRTINTR interrupt"]
pub type RTRIS_R = crate::BitReader;
#[doc = "Field `rxris` reader - Gives the raw interrupt state, prior to masking, of the SSPRXINTR interrupt"]
pub type RXRIS_R = crate::BitReader;
#[doc = "Field `txris` reader - Gives the raw interrupt state, prior to masking, of the SSPTXINTR interrupt"]
pub type TXRIS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Gives the raw interrupt state, prior to masking, of the SSPRORINTR interrupt"]
    #[inline(always)]
    pub fn rorris(&self) -> RORRIS_R {
        RORRIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Gives the raw interrupt state, prior to masking, of the SSPRTINTR interrupt"]
    #[inline(always)]
    pub fn rtris(&self) -> RTRIS_R {
        RTRIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Gives the raw interrupt state, prior to masking, of the SSPRXINTR interrupt"]
    #[inline(always)]
    pub fn rxris(&self) -> RXRIS_R {
        RXRIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Gives the raw interrupt state, prior to masking, of the SSPTXINTR interrupt"]
    #[inline(always)]
    pub fn txris(&self) -> TXRIS_R {
        TXRIS_R::new(((self.bits >> 3) & 1) != 0)
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
#[doc = "The SSPRIS register is the raw interrupt status register. It is a RO register. On a read this register gives the current raw status value of the corresponding interrupt prior to masking. A write has no effect.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp_ris::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssp_ris::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSP_RIS_SPEC;
impl crate::RegisterSpec for SSP_RIS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ssp_ris::R`](R) reader structure"]
impl crate::Readable for SSP_RIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ssp_ris::W`](W) writer structure"]
impl crate::Writable for SSP_RIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ssp_ris to value 0"]
impl crate::Resettable for SSP_RIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
