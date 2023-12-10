#[doc = "Register `gpi_9` reader"]
pub type R = crate::R<GPI_9_SPEC>;
#[doc = "Register `gpi_9` writer"]
pub type W = crate::W<GPI_9_SPEC>;
#[doc = "Field `pcm_rxd_0` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type PCM_RXD_0_R = crate::FieldReader;
#[doc = "Field `pcm_rxd_0` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type PCM_RXD_0_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `pcm_synon_0` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type PCM_SYNON_0_R = crate::FieldReader;
#[doc = "Field `pcm_synon_0` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type PCM_SYNON_0_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `can_rxd_1` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type CAN_RXD_1_R = crate::FieldReader;
#[doc = "Field `can_rxd_1` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type CAN_RXD_1_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `i2c_clk_1` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2C_CLK_1_R = crate::FieldReader;
#[doc = "Field `i2c_clk_1` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2C_CLK_1_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn pcm_rxd_0(&self) -> PCM_RXD_0_R {
        PCM_RXD_0_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn pcm_synon_0(&self) -> PCM_SYNON_0_R {
        PCM_SYNON_0_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn can_rxd_1(&self) -> CAN_RXD_1_R {
        CAN_RXD_1_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn i2c_clk_1(&self) -> I2C_CLK_1_R {
        I2C_CLK_1_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn pcm_rxd_0(&mut self) -> PCM_RXD_0_W<GPI_9_SPEC> {
        PCM_RXD_0_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn pcm_synon_0(&mut self) -> PCM_SYNON_0_W<GPI_9_SPEC> {
        PCM_SYNON_0_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn can_rxd_1(&mut self) -> CAN_RXD_1_W<GPI_9_SPEC> {
        CAN_RXD_1_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_clk_1(&mut self) -> I2C_CLK_1_W<GPI_9_SPEC> {
        I2C_CLK_1_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 36 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi_9::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi_9::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPI_9_SPEC;
impl crate::RegisterSpec for GPI_9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpi_9::R`](R) reader structure"]
impl crate::Readable for GPI_9_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpi_9::W`](W) writer structure"]
impl crate::Writable for GPI_9_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpi_9 to value 0"]
impl crate::Resettable for GPI_9_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
