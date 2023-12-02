#[doc = "Register `stg_sysconsaif_syscfg548` reader"]
pub type R = crate::R<STG_SYSCONSAIF_SYSCFG548_SPEC>;
#[doc = "Register `stg_sysconsaif_syscfg548` writer"]
pub type W = crate::W<STG_SYSCONSAIF_SYSCFG548_SPEC>;
#[doc = "Field `u1_plda_pcie_axi4_mst0_awfunc` reader - u1_plda_pcie_axi4_mst0_awfunc"]
pub type U1_PLDA_PCIE_AXI4_MST0_AWFUNC_R = crate::FieldReader<u16>;
#[doc = "Field `u1_plda_pcie_axi4_mst0_awregion` reader - u1_plda_pcie_axi4_mst0_awregion"]
pub type U1_PLDA_PCIE_AXI4_MST0_AWREGION_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:14 - u1_plda_pcie_axi4_mst0_awfunc"]
    #[inline(always)]
    pub fn u1_plda_pcie_axi4_mst0_awfunc(&self) -> U1_PLDA_PCIE_AXI4_MST0_AWFUNC_R {
        U1_PLDA_PCIE_AXI4_MST0_AWFUNC_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 15:18 - u1_plda_pcie_axi4_mst0_awregion"]
    #[inline(always)]
    pub fn u1_plda_pcie_axi4_mst0_awregion(&self) -> U1_PLDA_PCIE_AXI4_MST0_AWREGION_R {
        U1_PLDA_PCIE_AXI4_MST0_AWREGION_R::new(((self.bits >> 15) & 0x0f) as u8)
    }
}
impl W {
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
#[doc = "STG SYSCONSAIF SYSCFG 548\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_sysconsaif_syscfg548::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_sysconsaif_syscfg548::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCONSAIF_SYSCFG548_SPEC;
impl crate::RegisterSpec for STG_SYSCONSAIF_SYSCFG548_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_sysconsaif_syscfg548::R`](R) reader structure"]
impl crate::Readable for STG_SYSCONSAIF_SYSCFG548_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_sysconsaif_syscfg548::W`](W) writer structure"]
impl crate::Writable for STG_SYSCONSAIF_SYSCFG548_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
