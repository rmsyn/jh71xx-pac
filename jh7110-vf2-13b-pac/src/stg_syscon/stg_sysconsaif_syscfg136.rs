#[doc = "Register `stg_sysconsaif_syscfg136` reader"]
pub type R = crate::R<STG_SYSCONSAIF_SYSCFG136_SPEC>;
#[doc = "Register `stg_sysconsaif_syscfg136` writer"]
pub type W = crate::W<STG_SYSCONSAIF_SYSCFG136_SPEC>;
#[doc = "Field `u0_plda_pcie_axi4_slv0_aratomop_31_0` reader - u0_plda_pcie_axi4_slv0_aratomop_31_0"]
pub type U0_PLDA_PCIE_AXI4_SLV0_ARATOMOP_31_0_R = crate::FieldReader<u32>;
#[doc = "Field `u0_plda_pcie_axi4_slv0_aratomop_31_0` writer - u0_plda_pcie_axi4_slv0_aratomop_31_0"]
pub type U0_PLDA_PCIE_AXI4_SLV0_ARATOMOP_31_0_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - u0_plda_pcie_axi4_slv0_aratomop_31_0"]
    #[inline(always)]
    pub fn u0_plda_pcie_axi4_slv0_aratomop_31_0(&self) -> U0_PLDA_PCIE_AXI4_SLV0_ARATOMOP_31_0_R {
        U0_PLDA_PCIE_AXI4_SLV0_ARATOMOP_31_0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - u0_plda_pcie_axi4_slv0_aratomop_31_0"]
    #[inline(always)]
    #[must_use]
    pub fn u0_plda_pcie_axi4_slv0_aratomop_31_0(
        &mut self,
    ) -> U0_PLDA_PCIE_AXI4_SLV0_ARATOMOP_31_0_W<STG_SYSCONSAIF_SYSCFG136_SPEC, 0> {
        U0_PLDA_PCIE_AXI4_SLV0_ARATOMOP_31_0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 136\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_sysconsaif_syscfg136::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_sysconsaif_syscfg136::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCONSAIF_SYSCFG136_SPEC;
impl crate::RegisterSpec for STG_SYSCONSAIF_SYSCFG136_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_sysconsaif_syscfg136::R`](R) reader structure"]
impl crate::Readable for STG_SYSCONSAIF_SYSCFG136_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_sysconsaif_syscfg136::W`](W) writer structure"]
impl crate::Writable for STG_SYSCONSAIF_SYSCFG136_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
