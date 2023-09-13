#[doc = "Register `stg_sysconsaif_syscfg184` reader"]
pub type R = crate::R<STG_SYSCONSAIF_SYSCFG184_SPEC>;
#[doc = "Register `stg_sysconsaif_syscfg184` writer"]
pub type W = crate::W<STG_SYSCONSAIF_SYSCFG184_SPEC>;
#[doc = "Field `u0_plda_pcie_axi4_slv0_awuser_40_32` reader - u0_plda_pcie_axi4_slv0_awuser_40_32"]
pub type U0_PLDA_PCIE_AXI4_SLV0_AWUSER_40_32_R = crate::FieldReader<u16>;
#[doc = "Field `u0_plda_pcie_axi4_slv0_awuser_40_32` writer - u0_plda_pcie_axi4_slv0_awuser_40_32"]
pub type U0_PLDA_PCIE_AXI4_SLV0_AWUSER_40_32_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 9, O, u16>;
#[doc = "Field `u0_plda_pcie_axi4_slv0_rderr` reader - u0_plda_pcie_axi4_slv0_rderr"]
pub type U0_PLDA_PCIE_AXI4_SLV0_RDERR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:8 - u0_plda_pcie_axi4_slv0_awuser_40_32"]
    #[inline(always)]
    pub fn u0_plda_pcie_axi4_slv0_awuser_40_32(&self) -> U0_PLDA_PCIE_AXI4_SLV0_AWUSER_40_32_R {
        U0_PLDA_PCIE_AXI4_SLV0_AWUSER_40_32_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:16 - u0_plda_pcie_axi4_slv0_rderr"]
    #[inline(always)]
    pub fn u0_plda_pcie_axi4_slv0_rderr(&self) -> U0_PLDA_PCIE_AXI4_SLV0_RDERR_R {
        U0_PLDA_PCIE_AXI4_SLV0_RDERR_R::new(((self.bits >> 9) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - u0_plda_pcie_axi4_slv0_awuser_40_32"]
    #[inline(always)]
    #[must_use]
    pub fn u0_plda_pcie_axi4_slv0_awuser_40_32(
        &mut self,
    ) -> U0_PLDA_PCIE_AXI4_SLV0_AWUSER_40_32_W<STG_SYSCONSAIF_SYSCFG184_SPEC, 0> {
        U0_PLDA_PCIE_AXI4_SLV0_AWUSER_40_32_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 184\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_sysconsaif_syscfg184::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_sysconsaif_syscfg184::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCONSAIF_SYSCFG184_SPEC;
impl crate::RegisterSpec for STG_SYSCONSAIF_SYSCFG184_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_sysconsaif_syscfg184::R`](R) reader structure"]
impl crate::Readable for STG_SYSCONSAIF_SYSCFG184_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_sysconsaif_syscfg184::W`](W) writer structure"]
impl crate::Writable for STG_SYSCONSAIF_SYSCFG184_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
