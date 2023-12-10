#[doc = "Register `gpi_6` reader"]
pub type R = crate::R<GPI_6_SPEC>;
#[doc = "Register `gpi_6` writer"]
pub type W = crate::W<GPI_6_SPEC>;
#[doc = "Field `audio_i2srx_1` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type AUDIO_I2SRX_1_R = crate::FieldReader;
#[doc = "Field `audio_i2srx_1` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type AUDIO_I2SRX_1_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `audio_i2srx_2` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type AUDIO_I2SRX_2_R = crate::FieldReader;
#[doc = "Field `audio_i2srx_2` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type AUDIO_I2SRX_2_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `spi_clkin_0` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SPI_CLKIN_0_R = crate::FieldReader;
#[doc = "Field `spi_clkin_0` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SPI_CLKIN_0_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `spi_fssin_0` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SPI_FSSIN_0_R = crate::FieldReader;
#[doc = "Field `spi_fssin_0` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SPI_FSSIN_0_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn audio_i2srx_1(&self) -> AUDIO_I2SRX_1_R {
        AUDIO_I2SRX_1_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn audio_i2srx_2(&self) -> AUDIO_I2SRX_2_R {
        AUDIO_I2SRX_2_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn spi_clkin_0(&self) -> SPI_CLKIN_0_R {
        SPI_CLKIN_0_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn spi_fssin_0(&self) -> SPI_FSSIN_0_R {
        SPI_FSSIN_0_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn audio_i2srx_1(&mut self) -> AUDIO_I2SRX_1_W<GPI_6_SPEC> {
        AUDIO_I2SRX_1_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn audio_i2srx_2(&mut self) -> AUDIO_I2SRX_2_W<GPI_6_SPEC> {
        AUDIO_I2SRX_2_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn spi_clkin_0(&mut self) -> SPI_CLKIN_0_W<GPI_6_SPEC> {
        SPI_CLKIN_0_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn spi_fssin_0(&mut self) -> SPI_FSSIN_0_W<GPI_6_SPEC> {
        SPI_FSSIN_0_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 24 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi_6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi_6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPI_6_SPEC;
impl crate::RegisterSpec for GPI_6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpi_6::R`](R) reader structure"]
impl crate::Readable for GPI_6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpi_6::W`](W) writer structure"]
impl crate::Writable for GPI_6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpi_6 to value 0x3233_0000"]
impl crate::Resettable for GPI_6_SPEC {
    const RESET_VALUE: Self::Ux = 0x3233_0000;
}
