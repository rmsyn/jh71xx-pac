#[doc = "Register `sys_syscfg_20` reader"]
pub type R = crate::R<SYS_SYSCFG_20_SPEC>;
#[doc = "Register `sys_syscfg_20` writer"]
pub type W = crate::W<SYS_SYSCFG_20_SPEC>;
#[doc = "Field `u0_noc_bus_oic_qch_clock_stop_threshold_5` reader - u0_noc_bus_oic_qch_clock_stop_threshold_5"]
pub type U0_NOC_BUS_OIC_QCH_CLOCK_STOP_THRESHOLD_5_R = crate::FieldReader<u32>;
#[doc = "Field `u0_noc_bus_oic_qch_clock_stop_threshold_5` writer - u0_noc_bus_oic_qch_clock_stop_threshold_5"]
pub type U0_NOC_BUS_OIC_QCH_CLOCK_STOP_THRESHOLD_5_W<'a, REG> =
    crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - u0_noc_bus_oic_qch_clock_stop_threshold_5"]
    #[inline(always)]
    pub fn u0_noc_bus_oic_qch_clock_stop_threshold_5(
        &self,
    ) -> U0_NOC_BUS_OIC_QCH_CLOCK_STOP_THRESHOLD_5_R {
        U0_NOC_BUS_OIC_QCH_CLOCK_STOP_THRESHOLD_5_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - u0_noc_bus_oic_qch_clock_stop_threshold_5"]
    #[inline(always)]
    #[must_use]
    pub fn u0_noc_bus_oic_qch_clock_stop_threshold_5(
        &mut self,
    ) -> U0_NOC_BUS_OIC_QCH_CLOCK_STOP_THRESHOLD_5_W<SYS_SYSCFG_20_SPEC> {
        U0_NOC_BUS_OIC_QCH_CLOCK_STOP_THRESHOLD_5_W::new(self, 0)
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
#[doc = "SYS SYSCONSAIF SYSCFG 80\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_20::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_20::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_SYSCFG_20_SPEC;
impl crate::RegisterSpec for SYS_SYSCFG_20_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg_20::R`](R) reader structure"]
impl crate::Readable for SYS_SYSCFG_20_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_syscfg_20::W`](W) writer structure"]
impl crate::Writable for SYS_SYSCFG_20_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sys_syscfg_20 to value 0"]
impl crate::Resettable for SYS_SYSCFG_20_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
