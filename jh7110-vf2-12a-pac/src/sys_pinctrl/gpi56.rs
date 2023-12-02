#[doc = "Register `gpi56` reader"]
pub type R = crate::R<GPI56_SPEC>;
#[doc = "Register `gpi56` writer"]
pub type W = crate::W<GPI56_SPEC>;
#[doc = "Field `u1_ssp_spi_ssp_clkin_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U1_SSP_SPI_SSP_CLKIN_CFG_R = crate::FieldReader;
#[doc = "Field `u1_ssp_spi_ssp_clkin_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U1_SSP_SPI_SSP_CLKIN_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `u1_ssp_spi_sspfssin_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U1_SSP_SPI_SSPFSSIN_CFG_R = crate::FieldReader;
#[doc = "Field `u1_ssp_spi_sspfssin_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U1_SSP_SPI_SSPFSSIN_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `u1_ssp_spi_ssprxd_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U1_SSP_SPI_SSPRXD_CFG_R = crate::FieldReader;
#[doc = "Field `u1_ssp_spi_ssprxd_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U1_SSP_SPI_SSPRXD_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `u2_i2c_ic_clk_in_a_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U2_I2C_IC_CLK_IN_A_CFG_R = crate::FieldReader;
#[doc = "Field `u2_i2c_ic_clk_in_a_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U2_I2C_IC_CLK_IN_A_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u1_ssp_spi_ssp_clkin_cfg(&self) -> U1_SSP_SPI_SSP_CLKIN_CFG_R {
        U1_SSP_SPI_SSP_CLKIN_CFG_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u1_ssp_spi_sspfssin_cfg(&self) -> U1_SSP_SPI_SSPFSSIN_CFG_R {
        U1_SSP_SPI_SSPFSSIN_CFG_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u1_ssp_spi_ssprxd_cfg(&self) -> U1_SSP_SPI_SSPRXD_CFG_R {
        U1_SSP_SPI_SSPRXD_CFG_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u2_i2c_ic_clk_in_a_cfg(&self) -> U2_I2C_IC_CLK_IN_A_CFG_R {
        U2_I2C_IC_CLK_IN_A_CFG_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u1_ssp_spi_ssp_clkin_cfg(&mut self) -> U1_SSP_SPI_SSP_CLKIN_CFG_W<GPI56_SPEC> {
        U1_SSP_SPI_SSP_CLKIN_CFG_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u1_ssp_spi_sspfssin_cfg(&mut self) -> U1_SSP_SPI_SSPFSSIN_CFG_W<GPI56_SPEC> {
        U1_SSP_SPI_SSPFSSIN_CFG_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u1_ssp_spi_ssprxd_cfg(&mut self) -> U1_SSP_SPI_SSPRXD_CFG_W<GPI56_SPEC> {
        U1_SSP_SPI_SSPRXD_CFG_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u2_i2c_ic_clk_in_a_cfg(&mut self) -> U2_I2C_IC_CLK_IN_A_CFG_W<GPI56_SPEC> {
        U2_I2C_IC_CLK_IN_A_CFG_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 56 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi56::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi56::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPI56_SPEC;
impl crate::RegisterSpec for GPI56_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpi56::R`](R) reader structure"]
impl crate::Readable for GPI56_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpi56::W`](W) writer structure"]
impl crate::Writable for GPI56_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpi56 to value 0x0038_3637"]
impl crate::Resettable for GPI56_SPEC {
    const RESET_VALUE: Self::Ux = 0x0038_3637;
}
