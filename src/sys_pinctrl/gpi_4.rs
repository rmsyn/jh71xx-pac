#[doc = "Register `gpi_4` reader"]
pub type R = crate::R<GPI_4_SPEC>;
#[doc = "Register `gpi_4` writer"]
pub type W = crate::W<GPI_4_SPEC>;
#[doc = "Field `hifi4_jtdi` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type HIFI4_JTDI_R = crate::FieldReader;
#[doc = "Field `hifi4_jtdi` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type HIFI4_JTDI_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `hifi4_jtms` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type HIFI4_JTMS_R = crate::FieldReader;
#[doc = "Field `hifi4_jtms` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type HIFI4_JTMS_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `hifi4_jtrstn` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type HIFI4_JTRSTN_R = crate::FieldReader;
#[doc = "Field `hifi4_jtrstn` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type HIFI4_JTRSTN_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `jtag_tdi` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type JTAG_TDI_R = crate::FieldReader;
#[doc = "Field `jtag_tdi` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type JTAG_TDI_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn hifi4_jtdi(&self) -> HIFI4_JTDI_R {
        HIFI4_JTDI_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn hifi4_jtms(&self) -> HIFI4_JTMS_R {
        HIFI4_JTMS_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn hifi4_jtrstn(&self) -> HIFI4_JTRSTN_R {
        HIFI4_JTRSTN_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn jtag_tdi(&self) -> JTAG_TDI_R {
        JTAG_TDI_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn hifi4_jtdi(&mut self) -> HIFI4_JTDI_W<GPI_4_SPEC> {
        HIFI4_JTDI_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn hifi4_jtms(&mut self) -> HIFI4_JTMS_W<GPI_4_SPEC> {
        HIFI4_JTMS_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn hifi4_jtrstn(&mut self) -> HIFI4_JTRSTN_W<GPI_4_SPEC> {
        HIFI4_JTRSTN_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_tdi(&mut self) -> JTAG_TDI_W<GPI_4_SPEC> {
        JTAG_TDI_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 16 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi_4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi_4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPI_4_SPEC;
impl crate::RegisterSpec for GPI_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpi_4::R`](R) reader structure"]
impl crate::Readable for GPI_4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpi_4::W`](W) writer structure"]
impl crate::Writable for GPI_4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpi_4 to value 0x040f_0e0c"]
impl crate::Resettable for GPI_4_SPEC {
    const RESET_VALUE: Self::Ux = 0x040f_0e0c;
}
