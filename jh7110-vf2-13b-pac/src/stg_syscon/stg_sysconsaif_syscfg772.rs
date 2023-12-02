#[doc = "Register `stg_sysconsaif_syscfg772` reader"]
pub type R = crate::R<STG_SYSCONSAIF_SYSCFG772_SPEC>;
#[doc = "Register `stg_sysconsaif_syscfg772` writer"]
pub type W = crate::W<STG_SYSCONSAIF_SYSCFG772_SPEC>;
#[doc = "Field `u1_plda_pcie_pl_sideband_in_63_32` reader - u1_plda_pcie_pl_sideband_in_63_32"]
pub type U1_PLDA_PCIE_PL_SIDEBAND_IN_63_32_R = crate::FieldReader<u32>;
#[doc = "Field `u1_plda_pcie_pl_sideband_in_63_32` writer - u1_plda_pcie_pl_sideband_in_63_32"]
pub type U1_PLDA_PCIE_PL_SIDEBAND_IN_63_32_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - u1_plda_pcie_pl_sideband_in_63_32"]
    #[inline(always)]
    pub fn u1_plda_pcie_pl_sideband_in_63_32(&self) -> U1_PLDA_PCIE_PL_SIDEBAND_IN_63_32_R {
        U1_PLDA_PCIE_PL_SIDEBAND_IN_63_32_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - u1_plda_pcie_pl_sideband_in_63_32"]
    #[inline(always)]
    #[must_use]
    pub fn u1_plda_pcie_pl_sideband_in_63_32(
        &mut self,
    ) -> U1_PLDA_PCIE_PL_SIDEBAND_IN_63_32_W<STG_SYSCONSAIF_SYSCFG772_SPEC> {
        U1_PLDA_PCIE_PL_SIDEBAND_IN_63_32_W::new(self, 0)
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
#[doc = "STG SYSCONSAIF SYSCFG 772\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_sysconsaif_syscfg772::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_sysconsaif_syscfg772::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCONSAIF_SYSCFG772_SPEC;
impl crate::RegisterSpec for STG_SYSCONSAIF_SYSCFG772_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_sysconsaif_syscfg772::R`](R) reader structure"]
impl crate::Readable for STG_SYSCONSAIF_SYSCFG772_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_sysconsaif_syscfg772::W`](W) writer structure"]
impl crate::Writable for STG_SYSCONSAIF_SYSCFG772_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
