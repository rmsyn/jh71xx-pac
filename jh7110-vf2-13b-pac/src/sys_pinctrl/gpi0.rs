#[doc = "Register `gpi0` reader"]
pub type R = crate::R<GPI0_SPEC>;
#[doc = "Register `gpi0` writer"]
pub type W = crate::W<GPI0_SPEC>;
#[doc = "Field `u0_WAVE511_i_uart_rxsin_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U0_WAVE511_I_UART_RXSIN_CFG_R = crate::FieldReader;
#[doc = "Field `u0_WAVE511_i_uart_rxsin_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U0_WAVE511_I_UART_RXSIN_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `u0_can_ctrl_rxd_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U0_CAN_CTRL_RXD_CFG_R = crate::FieldReader;
#[doc = "Field `u0_can_ctrl_rxd_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U0_CAN_CTRL_RXD_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `u0_cdn_usb_over_current_n_io_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U0_CDN_USB_OVER_CURRENT_N_IO_CFG_R = crate::FieldReader;
#[doc = "Field `u0_cdn_usb_over_current_n_io_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U0_CDN_USB_OVER_CURRENT_N_IO_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `u0_cdns_spdif_spdi_fi_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U0_CDNS_SPDIF_SPDI_FI_CFG_R = crate::FieldReader;
#[doc = "Field `u0_cdns_spdif_spdi_fi_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U0_CDNS_SPDIF_SPDI_FI_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u0_wave511_i_uart_rxsin_cfg(&self) -> U0_WAVE511_I_UART_RXSIN_CFG_R {
        U0_WAVE511_I_UART_RXSIN_CFG_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u0_can_ctrl_rxd_cfg(&self) -> U0_CAN_CTRL_RXD_CFG_R {
        U0_CAN_CTRL_RXD_CFG_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u0_cdn_usb_over_current_n_io_cfg(&self) -> U0_CDN_USB_OVER_CURRENT_N_IO_CFG_R {
        U0_CDN_USB_OVER_CURRENT_N_IO_CFG_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u0_cdns_spdif_spdi_fi_cfg(&self) -> U0_CDNS_SPDIF_SPDI_FI_CFG_R {
        U0_CDNS_SPDIF_SPDI_FI_CFG_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u0_wave511_i_uart_rxsin_cfg(&mut self) -> U0_WAVE511_I_UART_RXSIN_CFG_W<GPI0_SPEC> {
        U0_WAVE511_I_UART_RXSIN_CFG_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u0_can_ctrl_rxd_cfg(&mut self) -> U0_CAN_CTRL_RXD_CFG_W<GPI0_SPEC> {
        U0_CAN_CTRL_RXD_CFG_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdn_usb_over_current_n_io_cfg(
        &mut self,
    ) -> U0_CDN_USB_OVER_CURRENT_N_IO_CFG_W<GPI0_SPEC> {
        U0_CDN_USB_OVER_CURRENT_N_IO_CFG_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdns_spdif_spdi_fi_cfg(&mut self) -> U0_CDNS_SPDIF_SPDI_FI_CFG_W<GPI0_SPEC> {
        U0_CDNS_SPDIF_SPDI_FI_CFG_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 0 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPI0_SPEC;
impl crate::RegisterSpec for GPI0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpi0::R`](R) reader structure"]
impl crate::Readable for GPI0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpi0::W`](W) writer structure"]
impl crate::Writable for GPI0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpi0 to value 0"]
impl crate::Resettable for GPI0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
