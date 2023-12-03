#[doc = "Register `stg_syscfg_232` reader"]
pub type R = crate::R<STG_SYSCFG_232_SPEC>;
#[doc = "Register `stg_syscfg_232` writer"]
pub type W = crate::W<STG_SYSCFG_232_SPEC>;
#[doc = "Field `u1_plda_pcie_tl_ctrl_hotplug` reader - u1_plda_pcie_tl_ctrl_hotplug"]
pub type U1_PLDA_PCIE_TL_CTRL_HOTPLUG_R = crate::FieldReader<u16>;
#[doc = "Field `u1_plda_pcie_tl_report_hotplug` reader - u1_plda_pcie_tl_report_hotplug"]
pub type U1_PLDA_PCIE_TL_REPORT_HOTPLUG_R = crate::FieldReader<u16>;
#[doc = "Field `u1_plda_pcie_tl_report_hotplug` writer - u1_plda_pcie_tl_report_hotplug"]
pub type U1_PLDA_PCIE_TL_REPORT_HOTPLUG_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - u1_plda_pcie_tl_ctrl_hotplug"]
    #[inline(always)]
    pub fn u1_plda_pcie_tl_ctrl_hotplug(&self) -> U1_PLDA_PCIE_TL_CTRL_HOTPLUG_R {
        U1_PLDA_PCIE_TL_CTRL_HOTPLUG_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - u1_plda_pcie_tl_report_hotplug"]
    #[inline(always)]
    pub fn u1_plda_pcie_tl_report_hotplug(&self) -> U1_PLDA_PCIE_TL_REPORT_HOTPLUG_R {
        U1_PLDA_PCIE_TL_REPORT_HOTPLUG_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - u1_plda_pcie_tl_report_hotplug"]
    #[inline(always)]
    #[must_use]
    pub fn u1_plda_pcie_tl_report_hotplug(
        &mut self,
    ) -> U1_PLDA_PCIE_TL_REPORT_HOTPLUG_W<STG_SYSCFG_232_SPEC> {
        U1_PLDA_PCIE_TL_REPORT_HOTPLUG_W::new(self, 16)
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
#[doc = "STG SYSCONSAIF SYSCFG 928\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_232::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_232::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCFG_232_SPEC;
impl crate::RegisterSpec for STG_SYSCFG_232_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_232::R`](R) reader structure"]
impl crate::Readable for STG_SYSCFG_232_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_232::W`](W) writer structure"]
impl crate::Writable for STG_SYSCFG_232_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets stg_syscfg_232 to value 0"]
impl crate::Resettable for STG_SYSCFG_232_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
