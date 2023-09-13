#[doc = "Register `stg_sysconsaif_syscfg176` reader"]
pub type R = crate::R<STG_SYSCONSAIF_SYSCFG176_SPEC>;
#[doc = "Register `stg_sysconsaif_syscfg176` writer"]
pub type W = crate::W<STG_SYSCONSAIF_SYSCFG176_SPEC>;
#[doc = "Field `u0_plda_pcie_axi4_slv0_aruser_40_32` reader - u0_plda_pcie_axi4_slv0_aruser_40_32"]
pub type U0_PLDA_PCIE_AXI4_SLV0_ARUSER_40_32_R = crate::FieldReader<u16>;
#[doc = "Field `u0_plda_pcie_axi4_slv0_aruser_40_32` writer - u0_plda_pcie_axi4_slv0_aruser_40_32"]
pub type U0_PLDA_PCIE_AXI4_SLV0_ARUSER_40_32_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 9, O, u16>;
#[doc = "Field `u0_plda_pcie_axi4_slv0_awfunc` reader - u0_plda_pcie_axi4_slv0_awfunc"]
pub type U0_PLDA_PCIE_AXI4_SLV0_AWFUNC_R = crate::FieldReader<u16>;
#[doc = "Field `u0_plda_pcie_axi4_slv0_awfunc` writer - u0_plda_pcie_axi4_slv0_awfunc"]
pub type U0_PLDA_PCIE_AXI4_SLV0_AWFUNC_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 15, O, u16>;
#[doc = "Field `u0_plda_pcie_axi4_slv0_awregion` reader - u0_plda_pcie_axi4_slv0_awregion"]
pub type U0_PLDA_PCIE_AXI4_SLV0_AWREGION_R = crate::FieldReader;
#[doc = "Field `u0_plda_pcie_axi4_slv0_awregion` writer - u0_plda_pcie_axi4_slv0_awregion"]
pub type U0_PLDA_PCIE_AXI4_SLV0_AWREGION_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:8 - u0_plda_pcie_axi4_slv0_aruser_40_32"]
    #[inline(always)]
    pub fn u0_plda_pcie_axi4_slv0_aruser_40_32(&self) -> U0_PLDA_PCIE_AXI4_SLV0_ARUSER_40_32_R {
        U0_PLDA_PCIE_AXI4_SLV0_ARUSER_40_32_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:23 - u0_plda_pcie_axi4_slv0_awfunc"]
    #[inline(always)]
    pub fn u0_plda_pcie_axi4_slv0_awfunc(&self) -> U0_PLDA_PCIE_AXI4_SLV0_AWFUNC_R {
        U0_PLDA_PCIE_AXI4_SLV0_AWFUNC_R::new(((self.bits >> 9) & 0x7fff) as u16)
    }
    #[doc = "Bits 24:27 - u0_plda_pcie_axi4_slv0_awregion"]
    #[inline(always)]
    pub fn u0_plda_pcie_axi4_slv0_awregion(&self) -> U0_PLDA_PCIE_AXI4_SLV0_AWREGION_R {
        U0_PLDA_PCIE_AXI4_SLV0_AWREGION_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - u0_plda_pcie_axi4_slv0_aruser_40_32"]
    #[inline(always)]
    #[must_use]
    pub fn u0_plda_pcie_axi4_slv0_aruser_40_32(
        &mut self,
    ) -> U0_PLDA_PCIE_AXI4_SLV0_ARUSER_40_32_W<STG_SYSCONSAIF_SYSCFG176_SPEC, 0> {
        U0_PLDA_PCIE_AXI4_SLV0_ARUSER_40_32_W::new(self)
    }
    #[doc = "Bits 9:23 - u0_plda_pcie_axi4_slv0_awfunc"]
    #[inline(always)]
    #[must_use]
    pub fn u0_plda_pcie_axi4_slv0_awfunc(
        &mut self,
    ) -> U0_PLDA_PCIE_AXI4_SLV0_AWFUNC_W<STG_SYSCONSAIF_SYSCFG176_SPEC, 9> {
        U0_PLDA_PCIE_AXI4_SLV0_AWFUNC_W::new(self)
    }
    #[doc = "Bits 24:27 - u0_plda_pcie_axi4_slv0_awregion"]
    #[inline(always)]
    #[must_use]
    pub fn u0_plda_pcie_axi4_slv0_awregion(
        &mut self,
    ) -> U0_PLDA_PCIE_AXI4_SLV0_AWREGION_W<STG_SYSCONSAIF_SYSCFG176_SPEC, 24> {
        U0_PLDA_PCIE_AXI4_SLV0_AWREGION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 176\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_sysconsaif_syscfg176::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_sysconsaif_syscfg176::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCONSAIF_SYSCFG176_SPEC;
impl crate::RegisterSpec for STG_SYSCONSAIF_SYSCFG176_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_sysconsaif_syscfg176::R`](R) reader structure"]
impl crate::Readable for STG_SYSCONSAIF_SYSCFG176_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_sysconsaif_syscfg176::W`](W) writer structure"]
impl crate::Writable for STG_SYSCONSAIF_SYSCFG176_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
