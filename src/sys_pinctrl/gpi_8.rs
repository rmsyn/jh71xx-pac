#[doc = "Register `gpi_8` reader"]
pub type R = crate::R<GPI_8_SPEC>;
#[doc = "Register `gpi_8` writer"]
pub type W = crate::W<GPI_8_SPEC>;
#[doc = "Field `i2srx_lrck_slv_0` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2SRX_LRCK_SLV_0_R = crate::FieldReader;
#[doc = "Field `i2srx_lrck_slv_0` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2SRX_LRCK_SLV_0_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `i2stx_bclk_slv_0` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2STX_BCLK_SLV_0_R = crate::FieldReader;
#[doc = "Field `i2stx_bclk_slv_0` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2STX_BCLK_SLV_0_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `i2stx_lrck_slv_0` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2STX_LRCK_SLV_0_R = crate::FieldReader;
#[doc = "Field `i2stx_lrck_slv_0` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2STX_LRCK_SLV_0_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `tdm_clk_slv_0` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type TDM_CLK_SLV_0_R = crate::FieldReader;
#[doc = "Field `tdm_clk_slv_0` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type TDM_CLK_SLV_0_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn i2srx_lrck_slv_0(&self) -> I2SRX_LRCK_SLV_0_R {
        I2SRX_LRCK_SLV_0_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn i2stx_bclk_slv_0(&self) -> I2STX_BCLK_SLV_0_R {
        I2STX_BCLK_SLV_0_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn i2stx_lrck_slv_0(&self) -> I2STX_LRCK_SLV_0_R {
        I2STX_LRCK_SLV_0_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn tdm_clk_slv_0(&self) -> TDM_CLK_SLV_0_R {
        TDM_CLK_SLV_0_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn i2srx_lrck_slv_0(&mut self) -> I2SRX_LRCK_SLV_0_W<GPI_8_SPEC> {
        I2SRX_LRCK_SLV_0_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn i2stx_bclk_slv_0(&mut self) -> I2STX_BCLK_SLV_0_W<GPI_8_SPEC> {
        I2STX_BCLK_SLV_0_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn i2stx_lrck_slv_0(&mut self) -> I2STX_LRCK_SLV_0_W<GPI_8_SPEC> {
        I2STX_LRCK_SLV_0_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn tdm_clk_slv_0(&mut self) -> TDM_CLK_SLV_0_W<GPI_8_SPEC> {
        TDM_CLK_SLV_0_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 32 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi_8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi_8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPI_8_SPEC;
impl crate::RegisterSpec for GPI_8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpi_8::R`](R) reader structure"]
impl crate::Readable for GPI_8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpi_8::W`](W) writer structure"]
impl crate::Writable for GPI_8_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpi_8 to value 0"]
impl crate::Resettable for GPI_8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
