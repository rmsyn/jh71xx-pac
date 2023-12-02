#[doc = "Register `stg_sysconsaif_syscfg592` reader"]
pub type R = crate::R<STG_SYSCONSAIF_SYSCFG592_SPEC>;
#[doc = "Register `stg_sysconsaif_syscfg592` writer"]
pub type W = crate::W<STG_SYSCONSAIF_SYSCFG592_SPEC>;
#[doc = "Field `u1_plda_pcie_axi4_slv0_aratomop_223_192` reader - u1_plda_pcie_axi4_slv0_aratomop_223_192"]
pub type U1_PLDA_PCIE_AXI4_SLV0_ARATOMOP_223_192_R = crate::FieldReader<u32>;
#[doc = "Field `u1_plda_pcie_axi4_slv0_aratomop_223_192` writer - u1_plda_pcie_axi4_slv0_aratomop_223_192"]
pub type U1_PLDA_PCIE_AXI4_SLV0_ARATOMOP_223_192_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - u1_plda_pcie_axi4_slv0_aratomop_223_192"]
    #[inline(always)]
    pub fn u1_plda_pcie_axi4_slv0_aratomop_223_192(
        &self,
    ) -> U1_PLDA_PCIE_AXI4_SLV0_ARATOMOP_223_192_R {
        U1_PLDA_PCIE_AXI4_SLV0_ARATOMOP_223_192_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - u1_plda_pcie_axi4_slv0_aratomop_223_192"]
    #[inline(always)]
    #[must_use]
    pub fn u1_plda_pcie_axi4_slv0_aratomop_223_192(
        &mut self,
    ) -> U1_PLDA_PCIE_AXI4_SLV0_ARATOMOP_223_192_W<STG_SYSCONSAIF_SYSCFG592_SPEC> {
        U1_PLDA_PCIE_AXI4_SLV0_ARATOMOP_223_192_W::new(self, 0)
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
#[doc = "STG SYSCONSAIF SYSCFG 592\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_sysconsaif_syscfg592::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_sysconsaif_syscfg592::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCONSAIF_SYSCFG592_SPEC;
impl crate::RegisterSpec for STG_SYSCONSAIF_SYSCFG592_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_sysconsaif_syscfg592::R`](R) reader structure"]
impl crate::Readable for STG_SYSCONSAIF_SYSCFG592_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_sysconsaif_syscfg592::W`](W) writer structure"]
impl crate::Writable for STG_SYSCONSAIF_SYSCFG592_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
