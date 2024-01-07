#[doc = "Register `gpi_1` reader"]
pub type R = crate::R<GPI_1_SPEC>;
#[doc = "Register `gpi_1` writer"]
pub type W = crate::W<GPI_1_SPEC>;
#[doc = "Field `jtag_trstn` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type JTAG_TRSTN_R = crate::FieldReader;
#[doc = "Field `jtag_trstn` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type JTAG_TRSTN_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `hdmi_cec_sda` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type HDMI_CEC_SDA_R = crate::FieldReader;
#[doc = "Field `hdmi_cec_sda` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type HDMI_CEC_SDA_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `hdmi_ddc_scl` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type HDMI_DDC_SCL_R = crate::FieldReader;
#[doc = "Field `hdmi_ddc_scl` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type HDMI_DDC_SCL_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `hdmi_ddc_sda` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type HDMI_DDC_SDA_R = crate::FieldReader;
#[doc = "Field `hdmi_ddc_sda` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type HDMI_DDC_SDA_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn jtag_trstn(&self) -> JTAG_TRSTN_R {
        JTAG_TRSTN_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn hdmi_cec_sda(&self) -> HDMI_CEC_SDA_R {
        HDMI_CEC_SDA_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn hdmi_ddc_scl(&self) -> HDMI_DDC_SCL_R {
        HDMI_DDC_SCL_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn hdmi_ddc_sda(&self) -> HDMI_DDC_SDA_R {
        HDMI_DDC_SDA_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_trstn(&mut self) -> JTAG_TRSTN_W<GPI_1_SPEC> {
        JTAG_TRSTN_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn hdmi_cec_sda(&mut self) -> HDMI_CEC_SDA_W<GPI_1_SPEC> {
        HDMI_CEC_SDA_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn hdmi_ddc_scl(&mut self) -> HDMI_DDC_SCL_W<GPI_1_SPEC> {
        HDMI_DDC_SCL_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn hdmi_ddc_sda(&mut self) -> HDMI_DDC_SDA_W<GPI_1_SPEC> {
        HDMI_DDC_SDA_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 4 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPI_1_SPEC;
impl crate::RegisterSpec for GPI_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpi_1::R`](R) reader structure"]
impl crate::Readable for GPI_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpi_1::W`](W) writer structure"]
impl crate::Writable for GPI_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpi_1 to value 0x02"]
impl crate::Resettable for GPI_1_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
