#[doc = "Register `gpi_15` reader"]
pub type R = crate::R<GPI_15_SPEC>;
#[doc = "Register `gpi_15` writer"]
pub type W = crate::W<GPI_15_SPEC>;
#[doc = "Field `i2c_data_2` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2C_DATA_2_R = crate::FieldReader;
#[doc = "Field `i2c_data_2` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2C_DATA_2_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `uart_cts_2` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type UART_CTS_2_R = crate::FieldReader;
#[doc = "Field `uart_cts_2` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type UART_CTS_2_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `uart_sin_2` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type UART_SIN_2_R = crate::FieldReader;
#[doc = "Field `uart_sin_2` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type UART_SIN_2_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `spi_clkin_2` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SPI_CLKIN_2_R = crate::FieldReader;
#[doc = "Field `spi_clkin_2` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SPI_CLKIN_2_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn i2c_data_2(&self) -> I2C_DATA_2_R {
        I2C_DATA_2_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn uart_cts_2(&self) -> UART_CTS_2_R {
        UART_CTS_2_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn uart_sin_2(&self) -> UART_SIN_2_R {
        UART_SIN_2_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn spi_clkin_2(&self) -> SPI_CLKIN_2_R {
        SPI_CLKIN_2_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_data_2(&mut self) -> I2C_DATA_2_W<GPI_15_SPEC> {
        I2C_DATA_2_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn uart_cts_2(&mut self) -> UART_CTS_2_W<GPI_15_SPEC> {
        UART_CTS_2_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn uart_sin_2(&mut self) -> UART_SIN_2_W<GPI_15_SPEC> {
        UART_SIN_2_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn spi_clkin_2(&mut self) -> SPI_CLKIN_2_W<GPI_15_SPEC> {
        SPI_CLKIN_2_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 60 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi_15::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi_15::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPI_15_SPEC;
impl crate::RegisterSpec for GPI_15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpi_15::R`](R) reader structure"]
impl crate::Readable for GPI_15_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpi_15::W`](W) writer structure"]
impl crate::Writable for GPI_15_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpi_15 to value 0x002a_2d00"]
impl crate::Resettable for GPI_15_SPEC {
    const RESET_VALUE: Self::Ux = 0x002a_2d00;
}
