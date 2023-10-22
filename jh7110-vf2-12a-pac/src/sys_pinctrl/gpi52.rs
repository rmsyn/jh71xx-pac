#[doc = "Register `gpi52` reader"]
pub type R = crate::R<GPI52_SPEC>;
#[doc = "Register `gpi52` writer"]
pub type W = crate::W<GPI52_SPEC>;
#[doc = "Field `u1_sdio_cdata_in_7_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U1_SDIO_CDATA_IN_7_CFG_R = crate::FieldReader;
#[doc = "Field `u1_sdio_cdata_in_7_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U1_SDIO_CDATA_IN_7_CFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `u1_sdio_data_strobe_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U1_SDIO_DATA_STROBE_CFG_R = crate::FieldReader;
#[doc = "Field `u1_sdio_data_strobe_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U1_SDIO_DATA_STROBE_CFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `u1_uart_cts_n_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U1_UART_CTS_N_CFG_R = crate::FieldReader;
#[doc = "Field `u1_uart_cts_n_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U1_UART_CTS_N_CFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `u1_uart_sin_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U1_UART_SIN_CFG_R = crate::FieldReader;
#[doc = "Field `u1_uart_sin_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U1_UART_SIN_CFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u1_sdio_cdata_in_7_cfg(&self) -> U1_SDIO_CDATA_IN_7_CFG_R {
        U1_SDIO_CDATA_IN_7_CFG_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u1_sdio_data_strobe_cfg(&self) -> U1_SDIO_DATA_STROBE_CFG_R {
        U1_SDIO_DATA_STROBE_CFG_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u1_uart_cts_n_cfg(&self) -> U1_UART_CTS_N_CFG_R {
        U1_UART_CTS_N_CFG_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u1_uart_sin_cfg(&self) -> U1_UART_SIN_CFG_R {
        U1_UART_SIN_CFG_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u1_sdio_cdata_in_7_cfg(&mut self) -> U1_SDIO_CDATA_IN_7_CFG_W<GPI52_SPEC, 0> {
        U1_SDIO_CDATA_IN_7_CFG_W::new(self)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u1_sdio_data_strobe_cfg(&mut self) -> U1_SDIO_DATA_STROBE_CFG_W<GPI52_SPEC, 8> {
        U1_SDIO_DATA_STROBE_CFG_W::new(self)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u1_uart_cts_n_cfg(&mut self) -> U1_UART_CTS_N_CFG_W<GPI52_SPEC, 16> {
        U1_UART_CTS_N_CFG_W::new(self)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u1_uart_sin_cfg(&mut self) -> U1_UART_SIN_CFG_W<GPI52_SPEC, 24> {
        U1_UART_SIN_CFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 52 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi52::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi52::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPI52_SPEC;
impl crate::RegisterSpec for GPI52_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpi52::R`](R) reader structure"]
impl crate::Readable for GPI52_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpi52::W`](W) writer structure"]
impl crate::Writable for GPI52_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpi52 to value 0"]
impl crate::Resettable for GPI52_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
