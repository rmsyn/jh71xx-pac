#[doc = "Register `sthr1` reader"]
pub type R = crate::R<STHR1_SPEC>;
#[doc = "Register `sthr1` writer"]
pub type W = crate::W<STHR1_SPEC>;
#[doc = "Field `sthr` writer - This is a shadow register for the THR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains data to be transmitted on the serial output port (sout) in UART mode or the serial infrared output (sir_out_n) in infrared mode. Data should only be written to the THR when the THR Empty (THRE) bit (LSR\\[5\\]) is set. If in non-FIFO mode or FIFOs are disabled (FCR\\[0\\]
set to zero) and THRE is set, writing a single character to the THR clears the THRE. Any additional writes to the THR before the THRE is set again causes the THR data to be overwritten. If in FIFO mode and FIFOs are enabled (FCR\\[0\\]
set to one) and THRE is set, x number of characters of data may be written to the THR before the FIFO is full. The number x (default=16) is determined by the value of FIFO Depth that you set during configuration. Any attempt to write data when the FIFO is full results in the write data being lost."]
pub type STHR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl W {
    #[doc = "Bits 0:7 - This is a shadow register for the THR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains data to be transmitted on the serial output port (sout) in UART mode or the serial infrared output (sir_out_n) in infrared mode. Data should only be written to the THR when the THR Empty (THRE) bit (LSR\\[5\\]) is set. If in non-FIFO mode or FIFOs are disabled (FCR\\[0\\]
set to zero) and THRE is set, writing a single character to the THR clears the THRE. Any additional writes to the THR before the THRE is set again causes the THR data to be overwritten. If in FIFO mode and FIFOs are enabled (FCR\\[0\\]
set to one) and THRE is set, x number of characters of data may be written to the THR before the FIFO is full. The number x (default=16) is determined by the value of FIFO Depth that you set during configuration. Any attempt to write data when the FIFO is full results in the write data being lost."]
    #[inline(always)]
    #[must_use]
    pub fn sthr(&mut self) -> STHR_W<STHR1_SPEC, 0> {
        STHR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Shadow Transmit Holding Register 1: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sthr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sthr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STHR1_SPEC;
impl crate::RegisterSpec for STHR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sthr1::R`](R) reader structure"]
impl crate::Readable for STHR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sthr1::W`](W) writer structure"]
impl crate::Writable for STHR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sthr1 to value 0"]
impl crate::Resettable for STHR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
