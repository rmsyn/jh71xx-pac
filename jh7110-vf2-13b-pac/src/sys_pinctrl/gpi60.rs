#[doc = "Register `gpi60` reader"]
pub type R = crate::R<GPI60_SPEC>;
#[doc = "Register `gpi60` writer"]
pub type W = crate::W<GPI60_SPEC>;
#[doc = "Field `u2_i2c_ic_data_in_a_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U2_I2C_IC_DATA_IN_A_CFG_R = crate::FieldReader;
#[doc = "Field `u2_i2c_ic_data_in_a_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U2_I2C_IC_DATA_IN_A_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `u2_uart_cts_n_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U2_UART_CTS_N_CFG_R = crate::FieldReader;
#[doc = "Field `u2_uart_cts_n_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U2_UART_CTS_N_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `u2_uart_sin_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U2_UART_SIN_CFG_R = crate::FieldReader;
#[doc = "Field `u2_uart_sin_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U2_UART_SIN_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `u2_ssp_spi_sspclkin_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U2_SSP_SPI_SSPCLKIN_CFG_R = crate::FieldReader;
#[doc = "Field `u2_ssp_spi_sspclkin_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U2_SSP_SPI_SSPCLKIN_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u2_i2c_ic_data_in_a_cfg(&self) -> U2_I2C_IC_DATA_IN_A_CFG_R {
        U2_I2C_IC_DATA_IN_A_CFG_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u2_uart_cts_n_cfg(&self) -> U2_UART_CTS_N_CFG_R {
        U2_UART_CTS_N_CFG_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u2_uart_sin_cfg(&self) -> U2_UART_SIN_CFG_R {
        U2_UART_SIN_CFG_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u2_ssp_spi_sspclkin_cfg(&self) -> U2_SSP_SPI_SSPCLKIN_CFG_R {
        U2_SSP_SPI_SSPCLKIN_CFG_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u2_i2c_ic_data_in_a_cfg(&mut self) -> U2_I2C_IC_DATA_IN_A_CFG_W<GPI60_SPEC> {
        U2_I2C_IC_DATA_IN_A_CFG_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u2_uart_cts_n_cfg(&mut self) -> U2_UART_CTS_N_CFG_W<GPI60_SPEC> {
        U2_UART_CTS_N_CFG_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u2_uart_sin_cfg(&mut self) -> U2_UART_SIN_CFG_W<GPI60_SPEC> {
        U2_UART_SIN_CFG_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u2_ssp_spi_sspclkin_cfg(&mut self) -> U2_SSP_SPI_SSPCLKIN_CFG_W<GPI60_SPEC> {
        U2_SSP_SPI_SSPCLKIN_CFG_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 60 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi60::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi60::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPI60_SPEC;
impl crate::RegisterSpec for GPI60_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpi60::R`](R) reader structure"]
impl crate::Readable for GPI60_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpi60::W`](W) writer structure"]
impl crate::Writable for GPI60_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpi60 to value 0x002a_2d00"]
impl crate::Resettable for GPI60_SPEC {
    const RESET_VALUE: Self::Ux = 0x002a_2d00;
}
