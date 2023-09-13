#[doc = "Register `stg_sysconsaif_syscfg132` reader"]
pub type R = crate::R<STG_SYSCONSAIF_SYSCFG132_SPEC>;
#[doc = "Register `stg_sysconsaif_syscfg132` writer"]
pub type W = crate::W<STG_SYSCONSAIF_SYSCFG132_SPEC>;
#[doc = "Field `u0_plda_pcie_axi4_mst0_wderr` reader - u0_plda_pcie_axi4_mst0_wderr"]
pub type U0_PLDA_PCIE_AXI4_MST0_WDERR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - u0_plda_pcie_axi4_mst0_wderr"]
    #[inline(always)]
    pub fn u0_plda_pcie_axi4_mst0_wderr(&self) -> U0_PLDA_PCIE_AXI4_MST0_WDERR_R {
        U0_PLDA_PCIE_AXI4_MST0_WDERR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 132\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_sysconsaif_syscfg132::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_sysconsaif_syscfg132::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCONSAIF_SYSCFG132_SPEC;
impl crate::RegisterSpec for STG_SYSCONSAIF_SYSCFG132_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_sysconsaif_syscfg132::R`](R) reader structure"]
impl crate::Readable for STG_SYSCONSAIF_SYSCFG132_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_sysconsaif_syscfg132::W`](W) writer structure"]
impl crate::Writable for STG_SYSCONSAIF_SYSCFG132_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
