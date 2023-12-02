#[doc = "Register `stg_sysconsaif_syscfg108` reader"]
pub type R = crate::R<STG_SYSCONSAIF_SYSCFG108_SPEC>;
#[doc = "Register `stg_sysconsaif_syscfg108` writer"]
pub type W = crate::W<STG_SYSCONSAIF_SYSCFG108_SPEC>;
#[doc = "Field `u0_plda_pcie_axi4_mst0_aruser_31_0` reader - u0_plda_pcie_axi4_mst0_aruser_31_0"]
pub type U0_PLDA_PCIE_AXI4_MST0_ARUSER_31_0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - u0_plda_pcie_axi4_mst0_aruser_31_0"]
    #[inline(always)]
    pub fn u0_plda_pcie_axi4_mst0_aruser_31_0(&self) -> U0_PLDA_PCIE_AXI4_MST0_ARUSER_31_0_R {
        U0_PLDA_PCIE_AXI4_MST0_ARUSER_31_0_R::new(self.bits)
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
#[doc = "STG SYSCONSAIF SYSCFG 108\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_sysconsaif_syscfg108::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_sysconsaif_syscfg108::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCONSAIF_SYSCFG108_SPEC;
impl crate::RegisterSpec for STG_SYSCONSAIF_SYSCFG108_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_sysconsaif_syscfg108::R`](R) reader structure"]
impl crate::Readable for STG_SYSCONSAIF_SYSCFG108_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_sysconsaif_syscfg108::W`](W) writer structure"]
impl crate::Writable for STG_SYSCONSAIF_SYSCFG108_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
