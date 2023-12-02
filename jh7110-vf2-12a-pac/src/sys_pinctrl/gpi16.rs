#[doc = "Register `gpi16` reader"]
pub type R = crate::R<GPI16_SPEC>;
#[doc = "Register `gpi16` writer"]
pub type W = crate::W<GPI16_SPEC>;
#[doc = "Field `u0_hifi4_jtdi_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U0_HIFI4_JTDI_CFG_R = crate::FieldReader;
#[doc = "Field `u0_hifi4_jtdi_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U0_HIFI4_JTDI_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `u0_hifi4_jtms_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U0_HIFI4_JTMS_CFG_R = crate::FieldReader;
#[doc = "Field `u0_hifi4_jtms_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U0_HIFI4_JTMS_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `u0_hifi4_jtrstn_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U0_HIFI4_JTRSTN_CFG_R = crate::FieldReader;
#[doc = "Field `u0_hifi4_jtrstn_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U0_HIFI4_JTRSTN_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `u0_jtag_certification_tdi_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U0_JTAG_CERTIFICATION_TDI_CFG_R = crate::FieldReader;
#[doc = "Field `u0_jtag_certification_tdi_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U0_JTAG_CERTIFICATION_TDI_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u0_hifi4_jtdi_cfg(&self) -> U0_HIFI4_JTDI_CFG_R {
        U0_HIFI4_JTDI_CFG_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u0_hifi4_jtms_cfg(&self) -> U0_HIFI4_JTMS_CFG_R {
        U0_HIFI4_JTMS_CFG_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u0_hifi4_jtrstn_cfg(&self) -> U0_HIFI4_JTRSTN_CFG_R {
        U0_HIFI4_JTRSTN_CFG_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u0_jtag_certification_tdi_cfg(&self) -> U0_JTAG_CERTIFICATION_TDI_CFG_R {
        U0_JTAG_CERTIFICATION_TDI_CFG_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u0_hifi4_jtdi_cfg(&mut self) -> U0_HIFI4_JTDI_CFG_W<GPI16_SPEC> {
        U0_HIFI4_JTDI_CFG_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u0_hifi4_jtms_cfg(&mut self) -> U0_HIFI4_JTMS_CFG_W<GPI16_SPEC> {
        U0_HIFI4_JTMS_CFG_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u0_hifi4_jtrstn_cfg(&mut self) -> U0_HIFI4_JTRSTN_CFG_W<GPI16_SPEC> {
        U0_HIFI4_JTRSTN_CFG_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u0_jtag_certification_tdi_cfg(&mut self) -> U0_JTAG_CERTIFICATION_TDI_CFG_W<GPI16_SPEC> {
        U0_JTAG_CERTIFICATION_TDI_CFG_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 16 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi16::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi16::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPI16_SPEC;
impl crate::RegisterSpec for GPI16_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpi16::R`](R) reader structure"]
impl crate::Readable for GPI16_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpi16::W`](W) writer structure"]
impl crate::Writable for GPI16_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpi16 to value 0x040f_0e0c"]
impl crate::Resettable for GPI16_SPEC {
    const RESET_VALUE: Self::Ux = 0x040f_0e0c;
}
