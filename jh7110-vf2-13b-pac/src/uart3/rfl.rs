#[doc = "Register `rfl` reader"]
pub type R = crate::R<RFL_SPEC>;
#[doc = "Register `rfl` writer"]
pub type W = crate::W<RFL_SPEC>;
#[doc = "Field `rfl` reader - Receive FIFO Level. This is indicates the number of data entries in the receive FIFO."]
pub type RFL_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Receive FIFO Level. This is indicates the number of data entries in the receive FIFO."]
    #[inline(always)]
    pub fn rfl(&self) -> RFL_R {
        RFL_R::new(self.bits)
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
#[doc = "Receive FIFO Level: This register is only valid when the DW_apb_uart is configured to have additional FIFO status registers implemented (FIFO_STAT == YES). If status registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RFL_SPEC;
impl crate::RegisterSpec for RFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfl::R`](R) reader structure"]
impl crate::Readable for RFL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rfl::W`](W) writer structure"]
impl crate::Writable for RFL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rfl to value 0"]
impl crate::Resettable for RFL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
