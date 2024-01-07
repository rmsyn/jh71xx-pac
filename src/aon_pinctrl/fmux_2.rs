#[doc = "Register `fmux_2` reader"]
pub type R = crate::R<FMUX_2_SPEC>;
#[doc = "Register `fmux_2` writer"]
pub type W = crate::W<FMUX_2_SPEC>;
#[doc = "Field `gpi_pmu_wakeup_0` reader - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type GPI_PMU_WAKEUP_0_R = crate::FieldReader;
#[doc = "Field `gpi_pmu_wakeup_0` writer - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type GPI_PMU_WAKEUP_0_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gpi_pmu_wakeup_1` reader - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type GPI_PMU_WAKEUP_1_R = crate::FieldReader;
#[doc = "Field `gpi_pmu_wakeup_1` writer - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type GPI_PMU_WAKEUP_1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gpi_pmu_wakeup_2` reader - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type GPI_PMU_WAKEUP_2_R = crate::FieldReader;
#[doc = "Field `gpi_pmu_wakeup_2` writer - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type GPI_PMU_WAKEUP_2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gpi_pmu_wakeup_3` reader - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type GPI_PMU_WAKEUP_3_R = crate::FieldReader;
#[doc = "Field `gpi_pmu_wakeup_3` writer - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type GPI_PMU_WAKEUP_3_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn gpi_pmu_wakeup_0(&self) -> GPI_PMU_WAKEUP_0_R {
        GPI_PMU_WAKEUP_0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn gpi_pmu_wakeup_1(&self) -> GPI_PMU_WAKEUP_1_R {
        GPI_PMU_WAKEUP_1_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:18 - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn gpi_pmu_wakeup_2(&self) -> GPI_PMU_WAKEUP_2_R {
        GPI_PMU_WAKEUP_2_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:26 - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn gpi_pmu_wakeup_3(&self) -> GPI_PMU_WAKEUP_3_R {
        GPI_PMU_WAKEUP_3_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn gpi_pmu_wakeup_0(&mut self) -> GPI_PMU_WAKEUP_0_W<FMUX_2_SPEC> {
        GPI_PMU_WAKEUP_0_W::new(self, 0)
    }
    #[doc = "Bits 8:10 - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn gpi_pmu_wakeup_1(&mut self) -> GPI_PMU_WAKEUP_1_W<FMUX_2_SPEC> {
        GPI_PMU_WAKEUP_1_W::new(self, 8)
    }
    #[doc = "Bits 16:18 - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn gpi_pmu_wakeup_2(&mut self) -> GPI_PMU_WAKEUP_2_W<FMUX_2_SPEC> {
        GPI_PMU_WAKEUP_2_W::new(self, 16)
    }
    #[doc = "Bits 24:26 - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn gpi_pmu_wakeup_3(&mut self) -> GPI_PMU_WAKEUP_3_W<FMUX_2_SPEC> {
        GPI_PMU_WAKEUP_3_W::new(self, 24)
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
#[doc = "AON IOMUX CFG SAIF SYSCFG FMUX 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmux_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmux_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMUX_2_SPEC;
impl crate::RegisterSpec for FMUX_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmux_2::R`](R) reader structure"]
impl crate::Readable for FMUX_2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fmux_2::W`](W) writer structure"]
impl crate::Writable for FMUX_2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets fmux_2 to value 0"]
impl crate::Resettable for FMUX_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
