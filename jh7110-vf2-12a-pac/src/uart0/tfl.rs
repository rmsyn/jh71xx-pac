#[doc = "Register `tfl` reader"]
pub type R = crate::R<TFL_SPEC>;
#[doc = "Register `tfl` writer"]
pub type W = crate::W<TFL_SPEC>;
#[doc = "Field `tfl` reader - Transmit FIFO Level. This is indicates the number of data entries in the transmit FIFO."]
pub type TFL_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Transmit FIFO Level. This is indicates the number of data entries in the transmit FIFO."]
    #[inline(always)]
    pub fn tfl(&self) -> TFL_R {
        TFL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transmit FIFO Level: This register is only valid when the DW_apb_uart is configured to have additional FIFO status registers implemented (FIFO_STAT == YES). If status registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tfl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tfl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TFL_SPEC;
impl crate::RegisterSpec for TFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tfl::R`](R) reader structure"]
impl crate::Readable for TFL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tfl::W`](W) writer structure"]
impl crate::Writable for TFL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tfl to value 0"]
impl crate::Resettable for TFL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
