#[doc = "Register `gpi_5` reader"]
pub type R = crate::R<GPI_5_SPEC>;
#[doc = "Register `gpi_5` writer"]
pub type W = crate::W<GPI_5_SPEC>;
#[doc = "Field `jtag_tms` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type JTAG_TMS_R = crate::FieldReader;
#[doc = "Field `jtag_tms` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type JTAG_TMS_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `pdm_dmic_0` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type PDM_DMIC_0_R = crate::FieldReader;
#[doc = "Field `pdm_dmic_0` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type PDM_DMIC_0_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `pdm_dmic_1` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type PDM_DMIC_1_R = crate::FieldReader;
#[doc = "Field `pdm_dmic_1` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type PDM_DMIC_1_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `audio_i2srx_0` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type AUDIO_I2SRX_0_R = crate::FieldReader;
#[doc = "Field `audio_i2srx_0` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type AUDIO_I2SRX_0_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn jtag_tms(&self) -> JTAG_TMS_R {
        JTAG_TMS_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn pdm_dmic_0(&self) -> PDM_DMIC_0_R {
        PDM_DMIC_0_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn pdm_dmic_1(&self) -> PDM_DMIC_1_R {
        PDM_DMIC_1_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn audio_i2srx_0(&self) -> AUDIO_I2SRX_0_R {
        AUDIO_I2SRX_0_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_tms(&mut self) -> JTAG_TMS_W<GPI_5_SPEC> {
        JTAG_TMS_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn pdm_dmic_0(&mut self) -> PDM_DMIC_0_W<GPI_5_SPEC> {
        PDM_DMIC_0_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn pdm_dmic_1(&mut self) -> PDM_DMIC_1_W<GPI_5_SPEC> {
        PDM_DMIC_1_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn audio_i2srx_0(&mut self) -> AUDIO_I2SRX_0_W<GPI_5_SPEC> {
        AUDIO_I2SRX_0_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 20 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi_5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi_5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPI_5_SPEC;
impl crate::RegisterSpec for GPI_5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpi_5::R`](R) reader structure"]
impl crate::Readable for GPI_5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpi_5::W`](W) writer structure"]
impl crate::Writable for GPI_5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpi_5 to value 0x06"]
impl crate::Resettable for GPI_5_SPEC {
    const RESET_VALUE: Self::Ux = 0x06;
}
