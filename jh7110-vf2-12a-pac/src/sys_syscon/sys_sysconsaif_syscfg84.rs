#[doc = "Register `sys_sysconsaif_syscfg84` reader"]
pub type R = crate::R<SYS_SYSCONSAIF_SYSCFG84_SPEC>;
#[doc = "Register `sys_sysconsaif_syscfg84` writer"]
pub type W = crate::W<SYS_SYSCONSAIF_SYSCFG84_SPEC>;
#[doc = "Field `u0_sft7110_noc_bus_oic_qch_clock_stop_threshold_6` reader - u0_sft7110_noc_bus_oic_qch_clock_stop_threshold_6"]
pub type U0_SFT7110_NOC_BUS_OIC_QCH_CLOCK_STOP_THRESHOLD_6_R = crate::FieldReader<u32>;
#[doc = "Field `u0_sft7110_noc_bus_oic_qch_clock_stop_threshold_6` writer - u0_sft7110_noc_bus_oic_qch_clock_stop_threshold_6"]
pub type U0_SFT7110_NOC_BUS_OIC_QCH_CLOCK_STOP_THRESHOLD_6_W<'a, REG> =
    crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - u0_sft7110_noc_bus_oic_qch_clock_stop_threshold_6"]
    #[inline(always)]
    pub fn u0_sft7110_noc_bus_oic_qch_clock_stop_threshold_6(
        &self,
    ) -> U0_SFT7110_NOC_BUS_OIC_QCH_CLOCK_STOP_THRESHOLD_6_R {
        U0_SFT7110_NOC_BUS_OIC_QCH_CLOCK_STOP_THRESHOLD_6_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - u0_sft7110_noc_bus_oic_qch_clock_stop_threshold_6"]
    #[inline(always)]
    #[must_use]
    pub fn u0_sft7110_noc_bus_oic_qch_clock_stop_threshold_6(
        &mut self,
    ) -> U0_SFT7110_NOC_BUS_OIC_QCH_CLOCK_STOP_THRESHOLD_6_W<SYS_SYSCONSAIF_SYSCFG84_SPEC> {
        U0_SFT7110_NOC_BUS_OIC_QCH_CLOCK_STOP_THRESHOLD_6_W::new(self, 0)
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
#[doc = "SYS SYSCONSAIF SYSCFG 84\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg84::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg84::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_SYSCONSAIF_SYSCFG84_SPEC;
impl crate::RegisterSpec for SYS_SYSCONSAIF_SYSCFG84_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_sysconsaif_syscfg84::R`](R) reader structure"]
impl crate::Readable for SYS_SYSCONSAIF_SYSCFG84_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_sysconsaif_syscfg84::W`](W) writer structure"]
impl crate::Writable for SYS_SYSCONSAIF_SYSCFG84_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
