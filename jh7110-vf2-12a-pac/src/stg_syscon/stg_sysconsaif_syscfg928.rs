#[doc = "Register `stg_sysconsaif_syscfg928` reader"]
pub type R = crate::R<STG_SYSCONSAIF_SYSCFG928_SPEC>;
#[doc = "Register `stg_sysconsaif_syscfg928` writer"]
pub type W = crate::W<STG_SYSCONSAIF_SYSCFG928_SPEC>;
#[doc = "Field `u1_plda_pcie_tl_ctrl_hotplug` reader - u1_plda_pcie_tl_ctrl_hotplug"]
pub type U1_PLDA_PCIE_TL_CTRL_HOTPLUG_R = crate::FieldReader<u16>;
#[doc = "Field `u1_plda_pcie_tl_report_hotplug` reader - u1_plda_pcie_tl_report_hotplug"]
pub type U1_PLDA_PCIE_TL_REPORT_HOTPLUG_R = crate::FieldReader<u16>;
#[doc = "Field `u1_plda_pcie_tl_report_hotplug` writer - u1_plda_pcie_tl_report_hotplug"]
pub type U1_PLDA_PCIE_TL_REPORT_HOTPLUG_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 16, O, u16>;
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
    ) -> U1_PLDA_PCIE_TL_REPORT_HOTPLUG_W<STG_SYSCONSAIF_SYSCFG928_SPEC, 16> {
        U1_PLDA_PCIE_TL_REPORT_HOTPLUG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 928\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_sysconsaif_syscfg928::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_sysconsaif_syscfg928::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCONSAIF_SYSCFG928_SPEC;
impl crate::RegisterSpec for STG_SYSCONSAIF_SYSCFG928_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_sysconsaif_syscfg928::R`](R) reader structure"]
impl crate::Readable for STG_SYSCONSAIF_SYSCFG928_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_sysconsaif_syscfg928::W`](W) writer structure"]
impl crate::Writable for STG_SYSCONSAIF_SYSCFG928_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
