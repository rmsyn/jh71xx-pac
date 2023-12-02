#[doc = "Register `gpi48` reader"]
pub type R = crate::R<GPI48_SPEC>;
#[doc = "Register `gpi48` writer"]
pub type W = crate::W<GPI48_SPEC>;
#[doc = "Field `u1_sdio_cdata_in_3_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U1_SDIO_CDATA_IN_3_CFG_R = crate::FieldReader;
#[doc = "Field `u1_sdio_cdata_in_3_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U1_SDIO_CDATA_IN_3_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `u1_sdio_cdata_in_4_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U1_SDIO_CDATA_IN_4_CFG_R = crate::FieldReader;
#[doc = "Field `u1_sdio_cdata_in_4_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U1_SDIO_CDATA_IN_4_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `u1_sdio_cdata_in_5_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U1_SDIO_CDATA_IN_5_CFG_R = crate::FieldReader;
#[doc = "Field `u1_sdio_cdata_in_5_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U1_SDIO_CDATA_IN_5_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `u1_sdio_cdata_in_6_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U1_SDIO_CDATA_IN_6_CFG_R = crate::FieldReader;
#[doc = "Field `u1_sdio_cdata_in_6_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type U1_SDIO_CDATA_IN_6_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u1_sdio_cdata_in_3_cfg(&self) -> U1_SDIO_CDATA_IN_3_CFG_R {
        U1_SDIO_CDATA_IN_3_CFG_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u1_sdio_cdata_in_4_cfg(&self) -> U1_SDIO_CDATA_IN_4_CFG_R {
        U1_SDIO_CDATA_IN_4_CFG_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u1_sdio_cdata_in_5_cfg(&self) -> U1_SDIO_CDATA_IN_5_CFG_R {
        U1_SDIO_CDATA_IN_5_CFG_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn u1_sdio_cdata_in_6_cfg(&self) -> U1_SDIO_CDATA_IN_6_CFG_R {
        U1_SDIO_CDATA_IN_6_CFG_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u1_sdio_cdata_in_3_cfg(&mut self) -> U1_SDIO_CDATA_IN_3_CFG_W<GPI48_SPEC> {
        U1_SDIO_CDATA_IN_3_CFG_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u1_sdio_cdata_in_4_cfg(&mut self) -> U1_SDIO_CDATA_IN_4_CFG_W<GPI48_SPEC> {
        U1_SDIO_CDATA_IN_4_CFG_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u1_sdio_cdata_in_5_cfg(&mut self) -> U1_SDIO_CDATA_IN_5_CFG_W<GPI48_SPEC> {
        U1_SDIO_CDATA_IN_5_CFG_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn u1_sdio_cdata_in_6_cfg(&mut self) -> U1_SDIO_CDATA_IN_6_CFG_W<GPI48_SPEC> {
        U1_SDIO_CDATA_IN_6_CFG_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 48 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi48::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi48::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPI48_SPEC;
impl crate::RegisterSpec for GPI48_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpi48::R`](R) reader structure"]
impl crate::Readable for GPI48_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpi48::W`](W) writer structure"]
impl crate::Writable for GPI48_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpi48 to value 0"]
impl crate::Resettable for GPI48_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
