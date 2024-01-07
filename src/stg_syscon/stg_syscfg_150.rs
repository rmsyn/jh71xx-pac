#[doc = "Register `stg_syscfg_150` reader"]
pub type R = crate::R<STG_SYSCFG_150_SPEC>;
#[doc = "Register `stg_syscfg_150` writer"]
pub type W = crate::W<STG_SYSCFG_150_SPEC>;
#[doc = "Field `u1_pcie_axi4_mst0_aratomop_257_256` reader - u1_pcie_axi4_mst0_aratomop_257_256"]
pub type U1_PCIE_AXI4_MST0_ARATOMOP_257_256_R = crate::FieldReader;
#[doc = "Field `u1_pcie_axi4_mst0_aratomop_257_256` writer - u1_pcie_axi4_mst0_aratomop_257_256"]
pub type U1_PCIE_AXI4_MST0_ARATOMOP_257_256_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u1_pcie_axi4_slv0_arfunc` reader - u1_pcie_axi4_slv0_arfunc"]
pub type U1_PCIE_AXI4_SLV0_ARFUNC_R = crate::FieldReader<u16>;
#[doc = "Field `u1_pcie_axi4_slv0_arfunc` writer - u1_pcie_axi4_slv0_arfunc"]
pub type U1_PCIE_AXI4_SLV0_ARFUNC_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `u1_pcie_axi4_slv0_arregion` reader - u1_pcie_axi4_slv0_arregion"]
pub type U1_PCIE_AXI4_SLV0_ARREGION_R = crate::FieldReader;
#[doc = "Field `u1_pcie_axi4_slv0_arregion` writer - u1_pcie_axi4_slv0_arregion"]
pub type U1_PCIE_AXI4_SLV0_ARREGION_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - u1_pcie_axi4_mst0_aratomop_257_256"]
    #[inline(always)]
    pub fn u1_pcie_axi4_mst0_aratomop_257_256(&self) -> U1_PCIE_AXI4_MST0_ARATOMOP_257_256_R {
        U1_PCIE_AXI4_MST0_ARATOMOP_257_256_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:16 - u1_pcie_axi4_slv0_arfunc"]
    #[inline(always)]
    pub fn u1_pcie_axi4_slv0_arfunc(&self) -> U1_PCIE_AXI4_SLV0_ARFUNC_R {
        U1_PCIE_AXI4_SLV0_ARFUNC_R::new(((self.bits >> 2) & 0x7fff) as u16)
    }
    #[doc = "Bits 17:20 - u1_pcie_axi4_slv0_arregion"]
    #[inline(always)]
    pub fn u1_pcie_axi4_slv0_arregion(&self) -> U1_PCIE_AXI4_SLV0_ARREGION_R {
        U1_PCIE_AXI4_SLV0_ARREGION_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - u1_pcie_axi4_mst0_aratomop_257_256"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_axi4_mst0_aratomop_257_256(
        &mut self,
    ) -> U1_PCIE_AXI4_MST0_ARATOMOP_257_256_W<STG_SYSCFG_150_SPEC> {
        U1_PCIE_AXI4_MST0_ARATOMOP_257_256_W::new(self, 0)
    }
    #[doc = "Bits 2:16 - u1_pcie_axi4_slv0_arfunc"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_axi4_slv0_arfunc(&mut self) -> U1_PCIE_AXI4_SLV0_ARFUNC_W<STG_SYSCFG_150_SPEC> {
        U1_PCIE_AXI4_SLV0_ARFUNC_W::new(self, 2)
    }
    #[doc = "Bits 17:20 - u1_pcie_axi4_slv0_arregion"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_axi4_slv0_arregion(
        &mut self,
    ) -> U1_PCIE_AXI4_SLV0_ARREGION_W<STG_SYSCFG_150_SPEC> {
        U1_PCIE_AXI4_SLV0_ARREGION_W::new(self, 17)
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
#[doc = "STG SYSCONSAIF SYSCFG 600\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_150::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_150::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCFG_150_SPEC;
impl crate::RegisterSpec for STG_SYSCFG_150_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_150::R`](R) reader structure"]
impl crate::Readable for STG_SYSCFG_150_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_150::W`](W) writer structure"]
impl crate::Writable for STG_SYSCFG_150_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets stg_syscfg_150 to value 0"]
impl crate::Resettable for STG_SYSCFG_150_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
