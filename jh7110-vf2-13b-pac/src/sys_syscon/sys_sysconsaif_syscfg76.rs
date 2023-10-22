#[doc = "Register `sys_sysconsaif_syscfg76` reader"]
pub type R = crate::R<SYS_SYSCONSAIF_SYSCFG76_SPEC>;
#[doc = "Register `sys_sysconsaif_syscfg76` writer"]
pub type W = crate::W<SYS_SYSCONSAIF_SYSCFG76_SPEC>;
#[doc = "Field `u0_sft7110_noc_bus_oic_qch_clock_stop_threshold_4` reader - u0_sft7110_noc_bus_oic_qch_clock_stop_threshold_4"]
pub type U0_SFT7110_NOC_BUS_OIC_QCH_CLOCK_STOP_THRESHOLD_4_R = crate::FieldReader<u32>;
#[doc = "Field `u0_sft7110_noc_bus_oic_qch_clock_stop_threshold_4` writer - u0_sft7110_noc_bus_oic_qch_clock_stop_threshold_4"]
pub type U0_SFT7110_NOC_BUS_OIC_QCH_CLOCK_STOP_THRESHOLD_4_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - u0_sft7110_noc_bus_oic_qch_clock_stop_threshold_4"]
    #[inline(always)]
    pub fn u0_sft7110_noc_bus_oic_qch_clock_stop_threshold_4(
        &self,
    ) -> U0_SFT7110_NOC_BUS_OIC_QCH_CLOCK_STOP_THRESHOLD_4_R {
        U0_SFT7110_NOC_BUS_OIC_QCH_CLOCK_STOP_THRESHOLD_4_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - u0_sft7110_noc_bus_oic_qch_clock_stop_threshold_4"]
    #[inline(always)]
    #[must_use]
    pub fn u0_sft7110_noc_bus_oic_qch_clock_stop_threshold_4(
        &mut self,
    ) -> U0_SFT7110_NOC_BUS_OIC_QCH_CLOCK_STOP_THRESHOLD_4_W<SYS_SYSCONSAIF_SYSCFG76_SPEC, 0> {
        U0_SFT7110_NOC_BUS_OIC_QCH_CLOCK_STOP_THRESHOLD_4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYS SYSCONSAIF SYSCFG 76\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg76::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg76::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_SYSCONSAIF_SYSCFG76_SPEC;
impl crate::RegisterSpec for SYS_SYSCONSAIF_SYSCFG76_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_sysconsaif_syscfg76::R`](R) reader structure"]
impl crate::Readable for SYS_SYSCONSAIF_SYSCFG76_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_sysconsaif_syscfg76::W`](W) writer structure"]
impl crate::Writable for SYS_SYSCONSAIF_SYSCFG76_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}