#[doc = "Register `stg_sysconsaif_syscfg536` reader"]
pub type R = crate::R<STG_SYSCONSAIF_SYSCFG536_SPEC>;
#[doc = "Register `stg_sysconsaif_syscfg536` writer"]
pub type W = crate::W<STG_SYSCONSAIF_SYSCFG536_SPEC>;
#[doc = "Field `u0_plda_pcie_axi4_mst0_aratomop_257_256` reader - u0_plda_pcie_axi4_mst0_aratomop_257_256"]
pub type U0_PLDA_PCIE_AXI4_MST0_ARATOMOP_257_256_R = crate::FieldReader;
#[doc = "Field `u0_plda_pcie_axi4_mst0_arfunc` reader - u0_plda_pcie_axi4_mst0_arfunc"]
pub type U0_PLDA_PCIE_AXI4_MST0_ARFUNC_R = crate::FieldReader<u16>;
#[doc = "Field `u0_plda_pcie_axi4_mst0_arregion` reader - u0_plda_pcie_axi4_mst0_arregion"]
pub type U0_PLDA_PCIE_AXI4_MST0_ARREGION_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - u0_plda_pcie_axi4_mst0_aratomop_257_256"]
    #[inline(always)]
    pub fn u0_plda_pcie_axi4_mst0_aratomop_257_256(
        &self,
    ) -> U0_PLDA_PCIE_AXI4_MST0_ARATOMOP_257_256_R {
        U0_PLDA_PCIE_AXI4_MST0_ARATOMOP_257_256_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:16 - u0_plda_pcie_axi4_mst0_arfunc"]
    #[inline(always)]
    pub fn u0_plda_pcie_axi4_mst0_arfunc(&self) -> U0_PLDA_PCIE_AXI4_MST0_ARFUNC_R {
        U0_PLDA_PCIE_AXI4_MST0_ARFUNC_R::new(((self.bits >> 2) & 0x7fff) as u16)
    }
    #[doc = "Bits 17:20 - u0_plda_pcie_axi4_mst0_arregion"]
    #[inline(always)]
    pub fn u0_plda_pcie_axi4_mst0_arregion(&self) -> U0_PLDA_PCIE_AXI4_MST0_ARREGION_R {
        U0_PLDA_PCIE_AXI4_MST0_ARREGION_R::new(((self.bits >> 17) & 0x0f) as u8)
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
#[doc = "STG SYSCONSAIF SYSCFG 536\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_sysconsaif_syscfg536::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_sysconsaif_syscfg536::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCONSAIF_SYSCFG536_SPEC;
impl crate::RegisterSpec for STG_SYSCONSAIF_SYSCFG536_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_sysconsaif_syscfg536::R`](R) reader structure"]
impl crate::Readable for STG_SYSCONSAIF_SYSCFG536_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_sysconsaif_syscfg536::W`](W) writer structure"]
impl crate::Writable for STG_SYSCONSAIF_SYSCFG536_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
