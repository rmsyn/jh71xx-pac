#[doc = "Register `dmasa` reader"]
pub type R = crate::R<DMASA_SPEC>;
#[doc = "Register `dmasa` writer"]
pub type W = crate::W<DMASA_SPEC>;
#[doc = "Field `dmasa` writer - This register is use to perform a DMA software acknowledge if a transfer needs to be terminated due to an error condition. For example, if the DMA disables the channel, then the DW_apb_uart should clear its request. This causes the TX request, TX single, RX request and RX single signals to de-assert. Note that this bit is 'self-clearing'. It is not necessary to clear this bit."]
pub type DMASA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - This register is use to perform a DMA software acknowledge if a transfer needs to be terminated due to an error condition. For example, if the DMA disables the channel, then the DW_apb_uart should clear its request. This causes the TX request, TX single, RX request and RX single signals to de-assert. Note that this bit is 'self-clearing'. It is not necessary to clear this bit."]
    #[inline(always)]
    #[must_use]
    pub fn dmasa(&mut self) -> DMASA_W<DMASA_SPEC, 0> {
        DMASA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DMA Software Acknowledge\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmasa::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmasa::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMASA_SPEC;
impl crate::RegisterSpec for DMASA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmasa::R`](R) reader structure"]
impl crate::Readable for DMASA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmasa::W`](W) writer structure"]
impl crate::Writable for DMASA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dmasa to value 0"]
impl crate::Resettable for DMASA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
