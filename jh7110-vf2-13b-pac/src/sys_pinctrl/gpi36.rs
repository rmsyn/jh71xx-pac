#[doc = "Register `gpi36` reader"]
pub type R = crate::R<GPI36_SPEC>;
#[doc = "Register `gpi36` writer"]
pub type W = crate::W<GPI36_SPEC>;
#[doc = "Field `u0_tdm16slot_pcm_rxd_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U0_TDM16SLOT_PCM_RXD_CFG_R = crate::FieldReader;
#[doc = "Field `u0_tdm16slot_pcm_rxd_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U0_TDM16SLOT_PCM_RXD_CFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `u0_tdm16slot_pcm_synon_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U0_TDM16SLOT_PCM_SYNON_CFG_R = crate::FieldReader;
#[doc = "Field `u0_tdm16slot_pcm_synon_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U0_TDM16SLOT_PCM_SYNON_CFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `u1_can_ctrl_rxd_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U1_CAN_CTRL_RXD_CFG_R = crate::FieldReader;
#[doc = "Field `u1_can_ctrl_rxd_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U1_CAN_CTRL_RXD_CFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `u1_i2c_ic_clk_in_a_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U1_I2C_IC_CLK_IN_A_CFG_R = crate::FieldReader;
#[doc = "Field `u1_i2c_ic_clk_in_a_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U1_I2C_IC_CLK_IN_A_CFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u0_tdm16slot_pcm_rxd_cfg(&self) -> U0_TDM16SLOT_PCM_RXD_CFG_R {
        U0_TDM16SLOT_PCM_RXD_CFG_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u0_tdm16slot_pcm_synon_cfg(&self) -> U0_TDM16SLOT_PCM_SYNON_CFG_R {
        U0_TDM16SLOT_PCM_SYNON_CFG_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u1_can_ctrl_rxd_cfg(&self) -> U1_CAN_CTRL_RXD_CFG_R {
        U1_CAN_CTRL_RXD_CFG_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u1_i2c_ic_clk_in_a_cfg(&self) -> U1_I2C_IC_CLK_IN_A_CFG_R {
        U1_I2C_IC_CLK_IN_A_CFG_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u0_tdm16slot_pcm_rxd_cfg(&mut self) -> U0_TDM16SLOT_PCM_RXD_CFG_W<GPI36_SPEC, 0> {
        U0_TDM16SLOT_PCM_RXD_CFG_W::new(self)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u0_tdm16slot_pcm_synon_cfg(&mut self) -> U0_TDM16SLOT_PCM_SYNON_CFG_W<GPI36_SPEC, 8> {
        U0_TDM16SLOT_PCM_SYNON_CFG_W::new(self)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u1_can_ctrl_rxd_cfg(&mut self) -> U1_CAN_CTRL_RXD_CFG_W<GPI36_SPEC, 16> {
        U1_CAN_CTRL_RXD_CFG_W::new(self)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u1_i2c_ic_clk_in_a_cfg(&mut self) -> U1_I2C_IC_CLK_IN_A_CFG_W<GPI36_SPEC, 24> {
        U1_I2C_IC_CLK_IN_A_CFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 36 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi36::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi36::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPI36_SPEC;
impl crate::RegisterSpec for GPI36_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpi36::R`](R) reader structure"]
impl crate::Readable for GPI36_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpi36::W`](W) writer structure"]
impl crate::Writable for GPI36_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpi36 to value 0"]
impl crate::Resettable for GPI36_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}