#[doc = "Register `sfe` reader"]
pub type R = crate::R<SFE_SPEC>;
#[doc = "Register `sfe` writer"]
pub type W = crate::W<SFE_SPEC>;
#[doc = "Field `sfe` reader - Shadow FIFO Enable. This is a shadow register for the FIFO enable bit (FCR\\[0\\]). This can be used to remove the burden of having to store the previously written value to the FCR in memory and having to mask this value so that only the FIFO enable bit gets updated.This enables/disables the transmit (XMIT) and receive (RCVR) FIFOs. If this bit is set to zero (disabled) after being enabled then both the XMIT and RCVR controller portion of FIFOs are reset."]
pub type SFE_R = crate::BitReader;
#[doc = "Field `sfe` writer - Shadow FIFO Enable. This is a shadow register for the FIFO enable bit (FCR\\[0\\]). This can be used to remove the burden of having to store the previously written value to the FCR in memory and having to mask this value so that only the FIFO enable bit gets updated.This enables/disables the transmit (XMIT) and receive (RCVR) FIFOs. If this bit is set to zero (disabled) after being enabled then both the XMIT and RCVR controller portion of FIFOs are reset."]
pub type SFE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Shadow FIFO Enable. This is a shadow register for the FIFO enable bit (FCR\\[0\\]). This can be used to remove the burden of having to store the previously written value to the FCR in memory and having to mask this value so that only the FIFO enable bit gets updated.This enables/disables the transmit (XMIT) and receive (RCVR) FIFOs. If this bit is set to zero (disabled) after being enabled then both the XMIT and RCVR controller portion of FIFOs are reset."]
    #[inline(always)]
    pub fn sfe(&self) -> SFE_R {
        SFE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shadow FIFO Enable. This is a shadow register for the FIFO enable bit (FCR\\[0\\]). This can be used to remove the burden of having to store the previously written value to the FCR in memory and having to mask this value so that only the FIFO enable bit gets updated.This enables/disables the transmit (XMIT) and receive (RCVR) FIFOs. If this bit is set to zero (disabled) after being enabled then both the XMIT and RCVR controller portion of FIFOs are reset."]
    #[inline(always)]
    #[must_use]
    pub fn sfe(&mut self) -> SFE_W<SFE_SPEC> {
        SFE_W::new(self, 0)
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
#[doc = "Shadow FIFO Enable: This register is only valid when the DW_apb_uart is configured to have additional FIFO registers implemented (FIFO_MODE != None) and additional shadow registers implemented (SHADOW == YES). If these registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sfe::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sfe::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SFE_SPEC;
impl crate::RegisterSpec for SFE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfe::R`](R) reader structure"]
impl crate::Readable for SFE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sfe::W`](W) writer structure"]
impl crate::Writable for SFE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sfe to value 0"]
impl crate::Resettable for SFE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
