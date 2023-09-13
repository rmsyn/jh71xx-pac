#[doc = "Register `aon_iomux_cfgsaif_syscfg_fmux2` reader"]
pub type R = crate::R<AON_IOMUX_CFGSAIF_SYSCFG_FMUX2_SPEC>;
#[doc = "Register `aon_iomux_cfgsaif_syscfg_fmux2` writer"]
pub type W = crate::W<AON_IOMUX_CFGSAIF_SYSCFG_FMUX2_SPEC>;
#[doc = "Field `aon_iomux_gpi_u0_pmu_io_event_stub_gpio_wakeup_0_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type AON_IOMUX_GPI_U0_PMU_IO_EVENT_STUB_GPIO_WAKEUP_0_CFG_R = crate::FieldReader;
#[doc = "Field `aon_iomux_gpi_u0_pmu_io_event_stub_gpio_wakeup_0_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type AON_IOMUX_GPI_U0_PMU_IO_EVENT_STUB_GPIO_WAKEUP_0_CFG_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `aon_iomux_gpi_u0_pmu_io_event_stub_gpio_wakeup_1_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type AON_IOMUX_GPI_U0_PMU_IO_EVENT_STUB_GPIO_WAKEUP_1_CFG_R = crate::FieldReader;
#[doc = "Field `aon_iomux_gpi_u0_pmu_io_event_stub_gpio_wakeup_1_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type AON_IOMUX_GPI_U0_PMU_IO_EVENT_STUB_GPIO_WAKEUP_1_CFG_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `aon_iomux_gpi_u0_pmu_io_event_stub_gpio_wakeup_2_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type AON_IOMUX_GPI_U0_PMU_IO_EVENT_STUB_GPIO_WAKEUP_2_CFG_R = crate::FieldReader;
#[doc = "Field `aon_iomux_gpi_u0_pmu_io_event_stub_gpio_wakeup_2_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type AON_IOMUX_GPI_U0_PMU_IO_EVENT_STUB_GPIO_WAKEUP_2_CFG_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `aon_iomux_gpi_u0_pmu_io_event_stub_gpio_wakeup_3_cfg` reader - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type AON_IOMUX_GPI_U0_PMU_IO_EVENT_STUB_GPIO_WAKEUP_3_CFG_R = crate::FieldReader;
#[doc = "Field `aon_iomux_gpi_u0_pmu_io_event_stub_gpio_wakeup_3_cfg` writer - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type AON_IOMUX_GPI_U0_PMU_IO_EVENT_STUB_GPIO_WAKEUP_3_CFG_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:2 - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn aon_iomux_gpi_u0_pmu_io_event_stub_gpio_wakeup_0_cfg(
        &self,
    ) -> AON_IOMUX_GPI_U0_PMU_IO_EVENT_STUB_GPIO_WAKEUP_0_CFG_R {
        AON_IOMUX_GPI_U0_PMU_IO_EVENT_STUB_GPIO_WAKEUP_0_CFG_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn aon_iomux_gpi_u0_pmu_io_event_stub_gpio_wakeup_1_cfg(
        &self,
    ) -> AON_IOMUX_GPI_U0_PMU_IO_EVENT_STUB_GPIO_WAKEUP_1_CFG_R {
        AON_IOMUX_GPI_U0_PMU_IO_EVENT_STUB_GPIO_WAKEUP_1_CFG_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:18 - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn aon_iomux_gpi_u0_pmu_io_event_stub_gpio_wakeup_2_cfg(
        &self,
    ) -> AON_IOMUX_GPI_U0_PMU_IO_EVENT_STUB_GPIO_WAKEUP_2_CFG_R {
        AON_IOMUX_GPI_U0_PMU_IO_EVENT_STUB_GPIO_WAKEUP_2_CFG_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:26 - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn aon_iomux_gpi_u0_pmu_io_event_stub_gpio_wakeup_3_cfg(
        &self,
    ) -> AON_IOMUX_GPI_U0_PMU_IO_EVENT_STUB_GPIO_WAKEUP_3_CFG_R {
        AON_IOMUX_GPI_U0_PMU_IO_EVENT_STUB_GPIO_WAKEUP_3_CFG_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn aon_iomux_gpi_u0_pmu_io_event_stub_gpio_wakeup_0_cfg(
        &mut self,
    ) -> AON_IOMUX_GPI_U0_PMU_IO_EVENT_STUB_GPIO_WAKEUP_0_CFG_W<
        AON_IOMUX_CFGSAIF_SYSCFG_FMUX2_SPEC,
        0,
    > {
        AON_IOMUX_GPI_U0_PMU_IO_EVENT_STUB_GPIO_WAKEUP_0_CFG_W::new(self)
    }
    #[doc = "Bits 8:10 - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn aon_iomux_gpi_u0_pmu_io_event_stub_gpio_wakeup_1_cfg(
        &mut self,
    ) -> AON_IOMUX_GPI_U0_PMU_IO_EVENT_STUB_GPIO_WAKEUP_1_CFG_W<
        AON_IOMUX_CFGSAIF_SYSCFG_FMUX2_SPEC,
        8,
    > {
        AON_IOMUX_GPI_U0_PMU_IO_EVENT_STUB_GPIO_WAKEUP_1_CFG_W::new(self)
    }
    #[doc = "Bits 16:18 - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn aon_iomux_gpi_u0_pmu_io_event_stub_gpio_wakeup_2_cfg(
        &mut self,
    ) -> AON_IOMUX_GPI_U0_PMU_IO_EVENT_STUB_GPIO_WAKEUP_2_CFG_W<
        AON_IOMUX_CFGSAIF_SYSCFG_FMUX2_SPEC,
        16,
    > {
        AON_IOMUX_GPI_U0_PMU_IO_EVENT_STUB_GPIO_WAKEUP_2_CFG_W::new(self)
    }
    #[doc = "Bits 24:26 - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn aon_iomux_gpi_u0_pmu_io_event_stub_gpio_wakeup_3_cfg(
        &mut self,
    ) -> AON_IOMUX_GPI_U0_PMU_IO_EVENT_STUB_GPIO_WAKEUP_3_CFG_W<
        AON_IOMUX_CFGSAIF_SYSCFG_FMUX2_SPEC,
        24,
    > {
        AON_IOMUX_GPI_U0_PMU_IO_EVENT_STUB_GPIO_WAKEUP_3_CFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AON IOMUX CFG SAIF SYSCFG FMUX 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg_fmux2::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg_fmux2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AON_IOMUX_CFGSAIF_SYSCFG_FMUX2_SPEC;
impl crate::RegisterSpec for AON_IOMUX_CFGSAIF_SYSCFG_FMUX2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_iomux_cfgsaif_syscfg_fmux2::R`](R) reader structure"]
impl crate::Readable for AON_IOMUX_CFGSAIF_SYSCFG_FMUX2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aon_iomux_cfgsaif_syscfg_fmux2::W`](W) writer structure"]
impl crate::Writable for AON_IOMUX_CFGSAIF_SYSCFG_FMUX2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
