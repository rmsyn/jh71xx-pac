#[doc = "Register `gpi88` reader"]
pub type R = crate::R<GPI88_SPEC>;
#[doc = "Register `gpi88` writer"]
pub type W = crate::W<GPI88_SPEC>;
#[doc = "Field `u6_ssp_spi_sspclkin_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U6_SSP_SPI_SSPCLKIN_CFG_R = crate::FieldReader;
#[doc = "Field `u6_ssp_spi_sspclkin_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U6_SSP_SPI_SSPCLKIN_CFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `u6_ssp_spi_sspfssin_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U6_SSP_SPI_SSPFSSIN_CFG_R = crate::FieldReader;
#[doc = "Field `u6_ssp_spi_sspfssin_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U6_SSP_SPI_SSPFSSIN_CFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `u6_ssp_spi_ssprxd_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U6_SSP_SPI_SSPRXD_CFG_R = crate::FieldReader;
#[doc = "Field `u6_ssp_spi_ssprxd_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U6_SSP_SPI_SSPRXD_CFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u6_ssp_spi_sspclkin_cfg(&self) -> U6_SSP_SPI_SSPCLKIN_CFG_R {
        U6_SSP_SPI_SSPCLKIN_CFG_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u6_ssp_spi_sspfssin_cfg(&self) -> U6_SSP_SPI_SSPFSSIN_CFG_R {
        U6_SSP_SPI_SSPFSSIN_CFG_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u6_ssp_spi_ssprxd_cfg(&self) -> U6_SSP_SPI_SSPRXD_CFG_R {
        U6_SSP_SPI_SSPRXD_CFG_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u6_ssp_spi_sspclkin_cfg(&mut self) -> U6_SSP_SPI_SSPCLKIN_CFG_W<GPI88_SPEC, 0> {
        U6_SSP_SPI_SSPCLKIN_CFG_W::new(self)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u6_ssp_spi_sspfssin_cfg(&mut self) -> U6_SSP_SPI_SSPFSSIN_CFG_W<GPI88_SPEC, 8> {
        U6_SSP_SPI_SSPFSSIN_CFG_W::new(self)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u6_ssp_spi_ssprxd_cfg(&mut self) -> U6_SSP_SPI_SSPRXD_CFG_W<GPI88_SPEC, 16> {
        U6_SSP_SPI_SSPRXD_CFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 88 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi88::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi88::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPI88_SPEC;
impl crate::RegisterSpec for GPI88_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpi88::R`](R) reader structure"]
impl crate::Readable for GPI88_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpi88::W`](W) writer structure"]
impl crate::Writable for GPI88_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpi88 to value 0"]
impl crate::Resettable for GPI88_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
