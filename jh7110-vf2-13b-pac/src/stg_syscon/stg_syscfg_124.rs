#[doc = "Register `stg_syscfg_124` reader"]
pub type R = crate::R<STG_SYSCFG_124_SPEC>;
#[doc = "Register `stg_syscfg_124` writer"]
pub type W = crate::W<STG_SYSCFG_124_SPEC>;
#[doc = "Field `u0_plda_pcie_tl_ctrl_hotplug` reader - u0_plda_pcie_tl_ctrl_hotplug"]
pub type U0_PLDA_PCIE_TL_CTRL_HOTPLUG_R = crate::FieldReader<u16>;
#[doc = "Field `u0_plda_pcie_tl_report_hotplug` reader - u0_plda_pcie_tl_report_hotplug"]
pub type U0_PLDA_PCIE_TL_REPORT_HOTPLUG_R = crate::FieldReader<u16>;
#[doc = "Field `u0_plda_pcie_tl_report_hotplug` writer - u0_plda_pcie_tl_report_hotplug"]
pub type U0_PLDA_PCIE_TL_REPORT_HOTPLUG_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - u0_plda_pcie_tl_ctrl_hotplug"]
    #[inline(always)]
    pub fn u0_plda_pcie_tl_ctrl_hotplug(&self) -> U0_PLDA_PCIE_TL_CTRL_HOTPLUG_R {
        U0_PLDA_PCIE_TL_CTRL_HOTPLUG_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - u0_plda_pcie_tl_report_hotplug"]
    #[inline(always)]
    pub fn u0_plda_pcie_tl_report_hotplug(&self) -> U0_PLDA_PCIE_TL_REPORT_HOTPLUG_R {
        U0_PLDA_PCIE_TL_REPORT_HOTPLUG_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - u0_plda_pcie_tl_report_hotplug"]
    #[inline(always)]
    #[must_use]
    pub fn u0_plda_pcie_tl_report_hotplug(
        &mut self,
    ) -> U0_PLDA_PCIE_TL_REPORT_HOTPLUG_W<STG_SYSCFG_124_SPEC> {
        U0_PLDA_PCIE_TL_REPORT_HOTPLUG_W::new(self, 16)
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
#[doc = "STG SYSCONSAIF SYSCFG 496\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_124::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_124::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCFG_124_SPEC;
impl crate::RegisterSpec for STG_SYSCFG_124_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_124::R`](R) reader structure"]
impl crate::Readable for STG_SYSCFG_124_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_124::W`](W) writer structure"]
impl crate::Writable for STG_SYSCFG_124_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets stg_syscfg_124 to value 0"]
impl crate::Resettable for STG_SYSCFG_124_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
