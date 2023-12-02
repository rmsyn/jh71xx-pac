#[doc = "Register `stg_sysconsaif_syscfg556` reader"]
pub type R = crate::R<STG_SYSCONSAIF_SYSCFG556_SPEC>;
#[doc = "Register `stg_sysconsaif_syscfg556` writer"]
pub type W = crate::W<STG_SYSCONSAIF_SYSCFG556_SPEC>;
#[doc = "Field `u1_plda_pcie_axi4_mst0_awuser_42_32` reader - u1_plda_pcie_axi4_mst0_awuser_42_32"]
pub type U1_PLDA_PCIE_AXI4_MST0_AWUSER_42_32_R = crate::FieldReader<u16>;
#[doc = "Field `u1_plda_pcie_axi4_mst0_rderr` reader - u1_plda_pcie_axi4_mst0_rderr"]
pub type U1_PLDA_PCIE_AXI4_MST0_RDERR_R = crate::FieldReader;
#[doc = "Field `u1_plda_pcie_axi4_mst0_rderr` writer - u1_plda_pcie_axi4_mst0_rderr"]
pub type U1_PLDA_PCIE_AXI4_MST0_RDERR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:10 - u1_plda_pcie_axi4_mst0_awuser_42_32"]
    #[inline(always)]
    pub fn u1_plda_pcie_axi4_mst0_awuser_42_32(&self) -> U1_PLDA_PCIE_AXI4_MST0_AWUSER_42_32_R {
        U1_PLDA_PCIE_AXI4_MST0_AWUSER_42_32_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:18 - u1_plda_pcie_axi4_mst0_rderr"]
    #[inline(always)]
    pub fn u1_plda_pcie_axi4_mst0_rderr(&self) -> U1_PLDA_PCIE_AXI4_MST0_RDERR_R {
        U1_PLDA_PCIE_AXI4_MST0_RDERR_R::new(((self.bits >> 11) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 11:18 - u1_plda_pcie_axi4_mst0_rderr"]
    #[inline(always)]
    #[must_use]
    pub fn u1_plda_pcie_axi4_mst0_rderr(
        &mut self,
    ) -> U1_PLDA_PCIE_AXI4_MST0_RDERR_W<STG_SYSCONSAIF_SYSCFG556_SPEC> {
        U1_PLDA_PCIE_AXI4_MST0_RDERR_W::new(self, 11)
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
#[doc = "STG SYSCONSAIF SYSCFG 556\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_sysconsaif_syscfg556::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_sysconsaif_syscfg556::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCONSAIF_SYSCFG556_SPEC;
impl crate::RegisterSpec for STG_SYSCONSAIF_SYSCFG556_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_sysconsaif_syscfg556::R`](R) reader structure"]
impl crate::Readable for STG_SYSCONSAIF_SYSCFG556_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_sysconsaif_syscfg556::W`](W) writer structure"]
impl crate::Writable for STG_SYSCONSAIF_SYSCFG556_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
