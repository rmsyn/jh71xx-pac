#[doc = "Register `stg_sysconsaif_syscfg624` reader"]
pub type R = crate::R<STG_SYSCONSAIF_SYSCFG624_SPEC>;
#[doc = "Register `stg_sysconsaif_syscfg624` writer"]
pub type W = crate::W<STG_SYSCONSAIF_SYSCFG624_SPEC>;
#[doc = "Field `u1_plda_pcie_axi4_slv0_wderr` reader - u1_plda_pcie_axi4_slv0_wderr"]
pub type U1_PLDA_PCIE_AXI4_SLV0_WDERR_R = crate::FieldReader;
#[doc = "Field `u1_plda_pcie_axi4_slv0_wderr` writer - u1_plda_pcie_axi4_slv0_wderr"]
pub type U1_PLDA_PCIE_AXI4_SLV0_WDERR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `u1_plda_pcie_axi4_slvl_arfunc` reader - u1_plda_pcie_axi4_slvl_arfunc"]
pub type U1_PLDA_PCIE_AXI4_SLVL_ARFUNC_R = crate::FieldReader<u16>;
#[doc = "Field `u1_plda_pcie_axi4_slvl_arfunc` writer - u1_plda_pcie_axi4_slvl_arfunc"]
pub type U1_PLDA_PCIE_AXI4_SLVL_ARFUNC_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:7 - u1_plda_pcie_axi4_slv0_wderr"]
    #[inline(always)]
    pub fn u1_plda_pcie_axi4_slv0_wderr(&self) -> U1_PLDA_PCIE_AXI4_SLV0_WDERR_R {
        U1_PLDA_PCIE_AXI4_SLV0_WDERR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:22 - u1_plda_pcie_axi4_slvl_arfunc"]
    #[inline(always)]
    pub fn u1_plda_pcie_axi4_slvl_arfunc(&self) -> U1_PLDA_PCIE_AXI4_SLVL_ARFUNC_R {
        U1_PLDA_PCIE_AXI4_SLVL_ARFUNC_R::new(((self.bits >> 8) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - u1_plda_pcie_axi4_slv0_wderr"]
    #[inline(always)]
    #[must_use]
    pub fn u1_plda_pcie_axi4_slv0_wderr(
        &mut self,
    ) -> U1_PLDA_PCIE_AXI4_SLV0_WDERR_W<STG_SYSCONSAIF_SYSCFG624_SPEC> {
        U1_PLDA_PCIE_AXI4_SLV0_WDERR_W::new(self, 0)
    }
    #[doc = "Bits 8:22 - u1_plda_pcie_axi4_slvl_arfunc"]
    #[inline(always)]
    #[must_use]
    pub fn u1_plda_pcie_axi4_slvl_arfunc(
        &mut self,
    ) -> U1_PLDA_PCIE_AXI4_SLVL_ARFUNC_W<STG_SYSCONSAIF_SYSCFG624_SPEC> {
        U1_PLDA_PCIE_AXI4_SLVL_ARFUNC_W::new(self, 8)
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
#[doc = "STG SYSCONSAIF SYSCFG 624\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_sysconsaif_syscfg624::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_sysconsaif_syscfg624::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCONSAIF_SYSCFG624_SPEC;
impl crate::RegisterSpec for STG_SYSCONSAIF_SYSCFG624_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_sysconsaif_syscfg624::R`](R) reader structure"]
impl crate::Readable for STG_SYSCONSAIF_SYSCFG624_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_sysconsaif_syscfg624::W`](W) writer structure"]
impl crate::Writable for STG_SYSCONSAIF_SYSCFG624_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
