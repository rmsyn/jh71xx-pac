#[doc = "Register `gpi20` reader"]
pub type R = crate::R<GPI20_SPEC>;
#[doc = "Register `gpi20` writer"]
pub type W = crate::W<GPI20_SPEC>;
#[doc = "Field `u0_jtag_certification_tms_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U0_JTAG_CERTIFICATION_TMS_CFG_R = crate::FieldReader;
#[doc = "Field `u0_jtag_certification_tms_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U0_JTAG_CERTIFICATION_TMS_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `u0_pdm_4mic_dmic0_din_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U0_PDM_4MIC_DMIC0_DIN_CFG_R = crate::FieldReader;
#[doc = "Field `u0_pdm_4mic_dmic0_din_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U0_PDM_4MIC_DMIC0_DIN_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `u0_pdm_4mic_dmic1_din_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U0_PDM_4MIC_DMIC1_DIN_CFG_R = crate::FieldReader;
#[doc = "Field `u0_pdm_4mic_dmic1_din_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U0_PDM_4MIC_DMIC1_DIN_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `u0_saif_audio_sdin_mux_i2srx_ext_sdin0_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U0_SAIF_AUDIO_SDIN_MUX_I2SRX_EXT_SDIN0_CFG_R = crate::FieldReader;
#[doc = "Field `u0_saif_audio_sdin_mux_i2srx_ext_sdin0_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U0_SAIF_AUDIO_SDIN_MUX_I2SRX_EXT_SDIN0_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u0_jtag_certification_tms_cfg(&self) -> U0_JTAG_CERTIFICATION_TMS_CFG_R {
        U0_JTAG_CERTIFICATION_TMS_CFG_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u0_pdm_4mic_dmic0_din_cfg(&self) -> U0_PDM_4MIC_DMIC0_DIN_CFG_R {
        U0_PDM_4MIC_DMIC0_DIN_CFG_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u0_pdm_4mic_dmic1_din_cfg(&self) -> U0_PDM_4MIC_DMIC1_DIN_CFG_R {
        U0_PDM_4MIC_DMIC1_DIN_CFG_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u0_saif_audio_sdin_mux_i2srx_ext_sdin0_cfg(
        &self,
    ) -> U0_SAIF_AUDIO_SDIN_MUX_I2SRX_EXT_SDIN0_CFG_R {
        U0_SAIF_AUDIO_SDIN_MUX_I2SRX_EXT_SDIN0_CFG_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u0_jtag_certification_tms_cfg(&mut self) -> U0_JTAG_CERTIFICATION_TMS_CFG_W<GPI20_SPEC> {
        U0_JTAG_CERTIFICATION_TMS_CFG_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u0_pdm_4mic_dmic0_din_cfg(&mut self) -> U0_PDM_4MIC_DMIC0_DIN_CFG_W<GPI20_SPEC> {
        U0_PDM_4MIC_DMIC0_DIN_CFG_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u0_pdm_4mic_dmic1_din_cfg(&mut self) -> U0_PDM_4MIC_DMIC1_DIN_CFG_W<GPI20_SPEC> {
        U0_PDM_4MIC_DMIC1_DIN_CFG_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u0_saif_audio_sdin_mux_i2srx_ext_sdin0_cfg(
        &mut self,
    ) -> U0_SAIF_AUDIO_SDIN_MUX_I2SRX_EXT_SDIN0_CFG_W<GPI20_SPEC> {
        U0_SAIF_AUDIO_SDIN_MUX_I2SRX_EXT_SDIN0_CFG_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 20 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi20::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi20::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPI20_SPEC;
impl crate::RegisterSpec for GPI20_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpi20::R`](R) reader structure"]
impl crate::Readable for GPI20_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpi20::W`](W) writer structure"]
impl crate::Writable for GPI20_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpi20 to value 0x06"]
impl crate::Resettable for GPI20_SPEC {
    const RESET_VALUE: Self::Ux = 0x06;
}
