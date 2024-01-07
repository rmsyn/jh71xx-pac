#[doc = "Register `gpi_11` reader"]
pub type R = crate::R<GPI_11_SPEC>;
#[doc = "Register `gpi_11` writer"]
pub type W = crate::W<GPI_11_SPEC>;
#[doc = "Field `sdio_ccmd_1` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SDIO_CCMD_1_R = crate::FieldReader;
#[doc = "Field `sdio_ccmd_1` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SDIO_CCMD_1_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `sdio_cdata_0` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SDIO_CDATA_0_R = crate::FieldReader;
#[doc = "Field `sdio_cdata_0` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SDIO_CDATA_0_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `sdio_cdata_1` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SDIO_CDATA_1_R = crate::FieldReader;
#[doc = "Field `sdio_cdata_1` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SDIO_CDATA_1_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `sdio_cdata_2` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SDIO_CDATA_2_R = crate::FieldReader;
#[doc = "Field `sdio_cdata_2` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SDIO_CDATA_2_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn sdio_ccmd_1(&self) -> SDIO_CCMD_1_R {
        SDIO_CCMD_1_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn sdio_cdata_0(&self) -> SDIO_CDATA_0_R {
        SDIO_CDATA_0_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn sdio_cdata_1(&self) -> SDIO_CDATA_1_R {
        SDIO_CDATA_1_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn sdio_cdata_2(&self) -> SDIO_CDATA_2_R {
        SDIO_CDATA_2_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn sdio_ccmd_1(&mut self) -> SDIO_CCMD_1_W<GPI_11_SPEC> {
        SDIO_CCMD_1_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn sdio_cdata_0(&mut self) -> SDIO_CDATA_0_W<GPI_11_SPEC> {
        SDIO_CDATA_0_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn sdio_cdata_1(&mut self) -> SDIO_CDATA_1_W<GPI_11_SPEC> {
        SDIO_CDATA_1_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn sdio_cdata_2(&mut self) -> SDIO_CDATA_2_W<GPI_11_SPEC> {
        SDIO_CDATA_2_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 44 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi_11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi_11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPI_11_SPEC;
impl crate::RegisterSpec for GPI_11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpi_11::R`](R) reader structure"]
impl crate::Readable for GPI_11_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpi_11::W`](W) writer structure"]
impl crate::Writable for GPI_11_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpi_11 to value 0"]
impl crate::Resettable for GPI_11_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
