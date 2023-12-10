#[doc = "Register `gpi_0` reader"]
pub type R = crate::R<GPI_0_SPEC>;
#[doc = "Register `gpi_0` writer"]
pub type W = crate::W<GPI_0_SPEC>;
#[doc = "Field `wave511_uart_rxsin` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type WAVE511_UART_RXSIN_R = crate::FieldReader;
#[doc = "Field `wave511_uart_rxsin` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type WAVE511_UART_RXSIN_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `can_rxd_0` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type CAN_RXD_0_R = crate::FieldReader;
#[doc = "Field `can_rxd_0` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type CAN_RXD_0_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `usb_over_current` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type USB_OVER_CURRENT_R = crate::FieldReader;
#[doc = "Field `usb_over_current` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type USB_OVER_CURRENT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `spdif_spdi_fi` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SPDIF_SPDI_FI_R = crate::FieldReader;
#[doc = "Field `spdif_spdi_fi` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SPDIF_SPDI_FI_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn wave511_uart_rxsin(&self) -> WAVE511_UART_RXSIN_R {
        WAVE511_UART_RXSIN_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn can_rxd_0(&self) -> CAN_RXD_0_R {
        CAN_RXD_0_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn usb_over_current(&self) -> USB_OVER_CURRENT_R {
        USB_OVER_CURRENT_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn spdif_spdi_fi(&self) -> SPDIF_SPDI_FI_R {
        SPDIF_SPDI_FI_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn wave511_uart_rxsin(&mut self) -> WAVE511_UART_RXSIN_W<GPI_0_SPEC> {
        WAVE511_UART_RXSIN_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn can_rxd_0(&mut self) -> CAN_RXD_0_W<GPI_0_SPEC> {
        CAN_RXD_0_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn usb_over_current(&mut self) -> USB_OVER_CURRENT_W<GPI_0_SPEC> {
        USB_OVER_CURRENT_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn spdif_spdi_fi(&mut self) -> SPDIF_SPDI_FI_W<GPI_0_SPEC> {
        SPDIF_SPDI_FI_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 0 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPI_0_SPEC;
impl crate::RegisterSpec for GPI_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpi_0::R`](R) reader structure"]
impl crate::Readable for GPI_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpi_0::W`](W) writer structure"]
impl crate::Writable for GPI_0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpi_0 to value 0"]
impl crate::Resettable for GPI_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
