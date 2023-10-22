#[doc = "Register `gpi72` reader"]
pub type R = crate::R<GPI72_SPEC>;
#[doc = "Register `gpi72` writer"]
pub type W = crate::W<GPI72_SPEC>;
#[doc = "Field `u4_i2c_ic_clk_in_a_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U4_I2C_IC_CLK_IN_A_CFG_R = crate::FieldReader;
#[doc = "Field `u4_i2c_ic_clk_in_a_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U4_I2C_IC_CLK_IN_A_CFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `u4_i2c_ic_data_in_a_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U4_I2C_IC_DATA_IN_A_CFG_R = crate::FieldReader;
#[doc = "Field `u4_i2c_ic_data_in_a_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U4_I2C_IC_DATA_IN_A_CFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `u4_uart_cts_n_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U4_UART_CTS_N_CFG_R = crate::FieldReader;
#[doc = "Field `u4_uart_cts_n_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U4_UART_CTS_N_CFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `u4_uart_sin_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U4_UART_SIN_CFG_R = crate::FieldReader;
#[doc = "Field `u4_uart_sin_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U4_UART_SIN_CFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u4_i2c_ic_clk_in_a_cfg(&self) -> U4_I2C_IC_CLK_IN_A_CFG_R {
        U4_I2C_IC_CLK_IN_A_CFG_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u4_i2c_ic_data_in_a_cfg(&self) -> U4_I2C_IC_DATA_IN_A_CFG_R {
        U4_I2C_IC_DATA_IN_A_CFG_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u4_uart_cts_n_cfg(&self) -> U4_UART_CTS_N_CFG_R {
        U4_UART_CTS_N_CFG_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u4_uart_sin_cfg(&self) -> U4_UART_SIN_CFG_R {
        U4_UART_SIN_CFG_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u4_i2c_ic_clk_in_a_cfg(&mut self) -> U4_I2C_IC_CLK_IN_A_CFG_W<GPI72_SPEC, 0> {
        U4_I2C_IC_CLK_IN_A_CFG_W::new(self)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u4_i2c_ic_data_in_a_cfg(&mut self) -> U4_I2C_IC_DATA_IN_A_CFG_W<GPI72_SPEC, 8> {
        U4_I2C_IC_DATA_IN_A_CFG_W::new(self)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u4_uart_cts_n_cfg(&mut self) -> U4_UART_CTS_N_CFG_W<GPI72_SPEC, 16> {
        U4_UART_CTS_N_CFG_W::new(self)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u4_uart_sin_cfg(&mut self) -> U4_UART_SIN_CFG_W<GPI72_SPEC, 24> {
        U4_UART_SIN_CFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 72 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi72::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi72::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPI72_SPEC;
impl crate::RegisterSpec for GPI72_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpi72::R`](R) reader structure"]
impl crate::Readable for GPI72_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpi72::W`](W) writer structure"]
impl crate::Writable for GPI72_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpi72 to value 0x2e31_0000"]
impl crate::Resettable for GPI72_SPEC {
    const RESET_VALUE: Self::Ux = 0x2e31_0000;
}