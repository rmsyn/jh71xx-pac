#[doc = "Register `ssp_pcell_id1` reader"]
pub type R = crate::R<SSP_PCELL_ID1_SPEC>;
#[doc = "Register `ssp_pcell_id1` writer"]
pub type W = crate::W<SSP_PCELL_ID1_SPEC>;
#[doc = "Field `ssp_pcell_id1` reader - The bits are read as 0xF0"]
pub type SSP_PCELL_ID1_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - The bits are read as 0xF0"]
    #[inline(always)]
    pub fn ssp_pcell_id1(&self) -> SSP_PCELL_ID1_R {
        SSP_PCELL_ID1_R::new((self.bits & 0xff) as u8)
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
#[doc = "The SSPPCellID0-3 registers are four 8-bit wide registers, that span address locations 0xFF0-0xFFC. The registers can conceptually be treated as a 32-bit register. The register is used as a standard cross-peripheral identification system. The SSPPCellID register is set to 0xB105F00D.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp_pcell_id1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssp_pcell_id1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSP_PCELL_ID1_SPEC;
impl crate::RegisterSpec for SSP_PCELL_ID1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ssp_pcell_id1::R`](R) reader structure"]
impl crate::Readable for SSP_PCELL_ID1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ssp_pcell_id1::W`](W) writer structure"]
impl crate::Writable for SSP_PCELL_ID1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ssp_pcell_id1 to value 0"]
impl crate::Resettable for SSP_PCELL_ID1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
