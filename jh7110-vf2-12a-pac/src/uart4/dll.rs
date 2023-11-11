#[doc = "Register `dll` reader"]
pub type R = crate::R<DLL_SPEC>;
#[doc = "Register `dll` writer"]
pub type W = crate::W<DLL_SPEC>;
#[doc = "Field `dll` reader - Lower 8 bits of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may only be accessed when the DLAB bit (LCR\\[7\\]) is set and the UART is not busy (USR\\[0\\]
is zero). The output baud rate is equal to the serial clock (pclk if one clock design, sclk if two clock design (CLOCK_MODE == Enabled)) frequency divided by sixteen times the value of the baud rate divisor, as follows: baud rate = (serial clock freq) / (16 * divisor). Note that with the Divisor Latch Registers (DLL and DLH) set to zero, the baud clock is disabled and no serial communications occur. Also, once the DLL is set, at least 8 clock cycles of the slowest DW_apb_uart clock should be allowed to pass before transmitting or receiving data."]
pub type DLL_R = crate::FieldReader;
#[doc = "Field `dll` writer - Lower 8 bits of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may only be accessed when the DLAB bit (LCR\\[7\\]) is set and the UART is not busy (USR\\[0\\]
is zero). The output baud rate is equal to the serial clock (pclk if one clock design, sclk if two clock design (CLOCK_MODE == Enabled)) frequency divided by sixteen times the value of the baud rate divisor, as follows: baud rate = (serial clock freq) / (16 * divisor). Note that with the Divisor Latch Registers (DLL and DLH) set to zero, the baud clock is disabled and no serial communications occur. Also, once the DLL is set, at least 8 clock cycles of the slowest DW_apb_uart clock should be allowed to pass before transmitting or receiving data."]
pub type DLL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Lower 8 bits of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may only be accessed when the DLAB bit (LCR\\[7\\]) is set and the UART is not busy (USR\\[0\\]
is zero). The output baud rate is equal to the serial clock (pclk if one clock design, sclk if two clock design (CLOCK_MODE == Enabled)) frequency divided by sixteen times the value of the baud rate divisor, as follows: baud rate = (serial clock freq) / (16 * divisor). Note that with the Divisor Latch Registers (DLL and DLH) set to zero, the baud clock is disabled and no serial communications occur. Also, once the DLL is set, at least 8 clock cycles of the slowest DW_apb_uart clock should be allowed to pass before transmitting or receiving data."]
    #[inline(always)]
    pub fn dll(&self) -> DLL_R {
        DLL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Lower 8 bits of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may only be accessed when the DLAB bit (LCR\\[7\\]) is set and the UART is not busy (USR\\[0\\]
is zero). The output baud rate is equal to the serial clock (pclk if one clock design, sclk if two clock design (CLOCK_MODE == Enabled)) frequency divided by sixteen times the value of the baud rate divisor, as follows: baud rate = (serial clock freq) / (16 * divisor). Note that with the Divisor Latch Registers (DLL and DLH) set to zero, the baud clock is disabled and no serial communications occur. Also, once the DLL is set, at least 8 clock cycles of the slowest DW_apb_uart clock should be allowed to pass before transmitting or receiving data."]
    #[inline(always)]
    #[must_use]
    pub fn dll(&mut self) -> DLL_W<DLL_SPEC, 0> {
        DLL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Divisor Latch Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dll::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dll::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DLL_SPEC;
impl crate::RegisterSpec for DLL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dll::R`](R) reader structure"]
impl crate::Readable for DLL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dll::W`](W) writer structure"]
impl crate::Writable for DLL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dll to value 0"]
impl crate::Resettable for DLL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
