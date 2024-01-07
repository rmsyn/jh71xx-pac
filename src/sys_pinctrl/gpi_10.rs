#[doc = "Register `gpi_10` reader"]
pub type R = crate::R<GPI_10_SPEC>;
#[doc = "Register `gpi_10` writer"]
pub type W = crate::W<GPI_10_SPEC>;
#[doc = "Field `i2c_data_1` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2C_DATA_1_R = crate::FieldReader;
#[doc = "Field `i2c_data_1` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2C_DATA_1_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `sdio_detect_1` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SDIO_DETECT_1_R = crate::FieldReader;
#[doc = "Field `sdio_detect_1` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SDIO_DETECT_1_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `sdio_int_1` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SDIO_INT_1_R = crate::FieldReader;
#[doc = "Field `sdio_int_1` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SDIO_INT_1_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `sdio_write_prt_1` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SDIO_WRITE_PRT_1_R = crate::FieldReader;
#[doc = "Field `sdio_write_prt_1` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SDIO_WRITE_PRT_1_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn i2c_data_1(&self) -> I2C_DATA_1_R {
        I2C_DATA_1_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn sdio_detect_1(&self) -> SDIO_DETECT_1_R {
        SDIO_DETECT_1_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn sdio_int_1(&self) -> SDIO_INT_1_R {
        SDIO_INT_1_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn sdio_write_prt_1(&self) -> SDIO_WRITE_PRT_1_R {
        SDIO_WRITE_PRT_1_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_data_1(&mut self) -> I2C_DATA_1_W<GPI_10_SPEC> {
        I2C_DATA_1_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn sdio_detect_1(&mut self) -> SDIO_DETECT_1_W<GPI_10_SPEC> {
        SDIO_DETECT_1_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn sdio_int_1(&mut self) -> SDIO_INT_1_W<GPI_10_SPEC> {
        SDIO_INT_1_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn sdio_write_prt_1(&mut self) -> SDIO_WRITE_PRT_1_W<GPI_10_SPEC> {
        SDIO_WRITE_PRT_1_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 40 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi_10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi_10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPI_10_SPEC;
impl crate::RegisterSpec for GPI_10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpi_10::R`](R) reader structure"]
impl crate::Readable for GPI_10_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpi_10::W`](W) writer structure"]
impl crate::Writable for GPI_10_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpi_10 to value 0"]
impl crate::Resettable for GPI_10_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
