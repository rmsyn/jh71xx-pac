#[doc = "Register `gpi84` reader"]
pub type R = crate::R<GPI84_SPEC>;
#[doc = "Register `gpi84` writer"]
pub type W = crate::W<GPI84_SPEC>;
#[doc = "Field `u5_ssp_spi_sspfssin_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U5_SSP_SPI_SSPFSSIN_CFG_R = crate::FieldReader;
#[doc = "Field `u5_ssp_spi_sspfssin_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U5_SSP_SPI_SSPFSSIN_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `u5_ssp_spi_ssprxd_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U5_SSP_SPI_SSPRXD_CFG_R = crate::FieldReader;
#[doc = "Field `u5_ssp_spi_ssprxd_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U5_SSP_SPI_SSPRXD_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `u6_i2c_ic_clk_in_a_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U6_I2C_IC_CLK_IN_A_CFG_R = crate::FieldReader;
#[doc = "Field `u6_i2c_ic_clk_in_a_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U6_I2C_IC_CLK_IN_A_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `u6_i2c_ic_data_in_a_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U6_I2C_IC_DATA_IN_A_CFG_R = crate::FieldReader;
#[doc = "Field `u6_i2c_ic_data_in_a_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U6_I2C_IC_DATA_IN_A_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u5_ssp_spi_sspfssin_cfg(&self) -> U5_SSP_SPI_SSPFSSIN_CFG_R {
        U5_SSP_SPI_SSPFSSIN_CFG_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u5_ssp_spi_ssprxd_cfg(&self) -> U5_SSP_SPI_SSPRXD_CFG_R {
        U5_SSP_SPI_SSPRXD_CFG_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u6_i2c_ic_clk_in_a_cfg(&self) -> U6_I2C_IC_CLK_IN_A_CFG_R {
        U6_I2C_IC_CLK_IN_A_CFG_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u6_i2c_ic_data_in_a_cfg(&self) -> U6_I2C_IC_DATA_IN_A_CFG_R {
        U6_I2C_IC_DATA_IN_A_CFG_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u5_ssp_spi_sspfssin_cfg(&mut self) -> U5_SSP_SPI_SSPFSSIN_CFG_W<GPI84_SPEC> {
        U5_SSP_SPI_SSPFSSIN_CFG_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u5_ssp_spi_ssprxd_cfg(&mut self) -> U5_SSP_SPI_SSPRXD_CFG_W<GPI84_SPEC> {
        U5_SSP_SPI_SSPRXD_CFG_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u6_i2c_ic_clk_in_a_cfg(&mut self) -> U6_I2C_IC_CLK_IN_A_CFG_W<GPI84_SPEC> {
        U6_I2C_IC_CLK_IN_A_CFG_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u6_i2c_ic_data_in_a_cfg(&mut self) -> U6_I2C_IC_DATA_IN_A_CFG_W<GPI84_SPEC> {
        U6_I2C_IC_DATA_IN_A_CFG_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 84 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi84::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi84::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPI84_SPEC;
impl crate::RegisterSpec for GPI84_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpi84::R`](R) reader structure"]
impl crate::Readable for GPI84_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpi84::W`](W) writer structure"]
impl crate::Writable for GPI84_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpi84 to value 0"]
impl crate::Resettable for GPI84_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
