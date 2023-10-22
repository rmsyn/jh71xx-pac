#[doc = "Register `gpi4` reader"]
pub type R = crate::R<GPI4_SPEC>;
#[doc = "Register `gpi4` writer"]
pub type W = crate::W<GPI4_SPEC>;
#[doc = "Field `u0_clkrst_src_bypass_jtag_trstn_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U0_CLKRST_SRC_BYPASS_JTAG_TRSTN_CFG_R = crate::FieldReader;
#[doc = "Field `u0_clkrst_src_bypass_jtag_trstn_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U0_CLKRST_SRC_BYPASS_JTAG_TRSTN_CFG_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `u0_dom_vout_top_u0_hdmi_tx_pin_cec_sda_in_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U0_DOM_VOUT_TOP_U0_HDMI_TX_PIN_CEC_SDA_IN_CFG_R = crate::FieldReader;
#[doc = "Field `u0_dom_vout_top_u0_hdmi_tx_pin_cec_sda_in_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U0_DOM_VOUT_TOP_U0_HDMI_TX_PIN_CEC_SDA_IN_CFG_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `u0_dom_vout_top_u0_hdmi_tx_pin_ddc_scl_in_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U0_DOM_VOUT_TOP_U0_HDMI_TX_PIN_DDC_SCL_IN_CFG_R = crate::FieldReader;
#[doc = "Field `u0_dom_vout_top_u0_hdmi_tx_pin_ddc_scl_in_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U0_DOM_VOUT_TOP_U0_HDMI_TX_PIN_DDC_SCL_IN_CFG_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `u0_dom_vout_top_u0_hdmi_tx_pin_ddc_sda_in_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U0_DOM_VOUT_TOP_U0_HDMI_TX_PIN_DDC_SDA_IN_CFG_R = crate::FieldReader;
#[doc = "Field `u0_dom_vout_top_u0_hdmi_tx_pin_ddc_sda_in_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U0_DOM_VOUT_TOP_U0_HDMI_TX_PIN_DDC_SDA_IN_CFG_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u0_clkrst_src_bypass_jtag_trstn_cfg(&self) -> U0_CLKRST_SRC_BYPASS_JTAG_TRSTN_CFG_R {
        U0_CLKRST_SRC_BYPASS_JTAG_TRSTN_CFG_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u0_dom_vout_top_u0_hdmi_tx_pin_cec_sda_in_cfg(
        &self,
    ) -> U0_DOM_VOUT_TOP_U0_HDMI_TX_PIN_CEC_SDA_IN_CFG_R {
        U0_DOM_VOUT_TOP_U0_HDMI_TX_PIN_CEC_SDA_IN_CFG_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u0_dom_vout_top_u0_hdmi_tx_pin_ddc_scl_in_cfg(
        &self,
    ) -> U0_DOM_VOUT_TOP_U0_HDMI_TX_PIN_DDC_SCL_IN_CFG_R {
        U0_DOM_VOUT_TOP_U0_HDMI_TX_PIN_DDC_SCL_IN_CFG_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u0_dom_vout_top_u0_hdmi_tx_pin_ddc_sda_in_cfg(
        &self,
    ) -> U0_DOM_VOUT_TOP_U0_HDMI_TX_PIN_DDC_SDA_IN_CFG_R {
        U0_DOM_VOUT_TOP_U0_HDMI_TX_PIN_DDC_SDA_IN_CFG_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u0_clkrst_src_bypass_jtag_trstn_cfg(
        &mut self,
    ) -> U0_CLKRST_SRC_BYPASS_JTAG_TRSTN_CFG_W<GPI4_SPEC, 0> {
        U0_CLKRST_SRC_BYPASS_JTAG_TRSTN_CFG_W::new(self)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u0_dom_vout_top_u0_hdmi_tx_pin_cec_sda_in_cfg(
        &mut self,
    ) -> U0_DOM_VOUT_TOP_U0_HDMI_TX_PIN_CEC_SDA_IN_CFG_W<GPI4_SPEC, 8> {
        U0_DOM_VOUT_TOP_U0_HDMI_TX_PIN_CEC_SDA_IN_CFG_W::new(self)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u0_dom_vout_top_u0_hdmi_tx_pin_ddc_scl_in_cfg(
        &mut self,
    ) -> U0_DOM_VOUT_TOP_U0_HDMI_TX_PIN_DDC_SCL_IN_CFG_W<GPI4_SPEC, 16> {
        U0_DOM_VOUT_TOP_U0_HDMI_TX_PIN_DDC_SCL_IN_CFG_W::new(self)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u0_dom_vout_top_u0_hdmi_tx_pin_ddc_sda_in_cfg(
        &mut self,
    ) -> U0_DOM_VOUT_TOP_U0_HDMI_TX_PIN_DDC_SDA_IN_CFG_W<GPI4_SPEC, 24> {
        U0_DOM_VOUT_TOP_U0_HDMI_TX_PIN_DDC_SDA_IN_CFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 4 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPI4_SPEC;
impl crate::RegisterSpec for GPI4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpi4::R`](R) reader structure"]
impl crate::Readable for GPI4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpi4::W`](W) writer structure"]
impl crate::Writable for GPI4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpi4 to value 0x02"]
impl crate::Resettable for GPI4_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}