#[doc = "Register `gpi_12` reader"]
pub type R = crate::R<GPI_12_SPEC>;
#[doc = "Register `gpi_12` writer"]
pub type W = crate::W<GPI_12_SPEC>;
#[doc = "Field `sdio_cdata_3` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SDIO_CDATA_3_R = crate::FieldReader;
#[doc = "Field `sdio_cdata_3` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SDIO_CDATA_3_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `sdio_cdata_4` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SDIO_CDATA_4_R = crate::FieldReader;
#[doc = "Field `sdio_cdata_4` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SDIO_CDATA_4_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `sdio_cdata_5` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SDIO_CDATA_5_R = crate::FieldReader;
#[doc = "Field `sdio_cdata_5` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SDIO_CDATA_5_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `sdio_cdata_6` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SDIO_CDATA_6_R = crate::FieldReader;
#[doc = "Field `sdio_cdata_6` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SDIO_CDATA_6_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn sdio_cdata_3(&self) -> SDIO_CDATA_3_R {
        SDIO_CDATA_3_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn sdio_cdata_4(&self) -> SDIO_CDATA_4_R {
        SDIO_CDATA_4_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn sdio_cdata_5(&self) -> SDIO_CDATA_5_R {
        SDIO_CDATA_5_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn sdio_cdata_6(&self) -> SDIO_CDATA_6_R {
        SDIO_CDATA_6_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn sdio_cdata_3(&mut self) -> SDIO_CDATA_3_W<GPI_12_SPEC> {
        SDIO_CDATA_3_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn sdio_cdata_4(&mut self) -> SDIO_CDATA_4_W<GPI_12_SPEC> {
        SDIO_CDATA_4_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn sdio_cdata_5(&mut self) -> SDIO_CDATA_5_W<GPI_12_SPEC> {
        SDIO_CDATA_5_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn sdio_cdata_6(&mut self) -> SDIO_CDATA_6_W<GPI_12_SPEC> {
        SDIO_CDATA_6_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 48 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi_12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi_12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPI_12_SPEC;
impl crate::RegisterSpec for GPI_12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpi_12::R`](R) reader structure"]
impl crate::Readable for GPI_12_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpi_12::W`](W) writer structure"]
impl crate::Writable for GPI_12_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpi_12 to value 0"]
impl crate::Resettable for GPI_12_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
