#[doc = "Register `stg_syscfg_135` reader"]
pub type R = crate::R<STG_SYSCFG_135_SPEC>;
#[doc = "Register `stg_syscfg_135` writer"]
pub type W = crate::W<STG_SYSCFG_135_SPEC>;
#[doc = "Field `u1_pcie_axi4_mst0_aruser_31_0` reader - u1_pcie_axi4_mst0_aruser_31_0"]
pub type U1_PCIE_AXI4_MST0_ARUSER_31_0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - u1_pcie_axi4_mst0_aruser_31_0"]
    #[inline(always)]
    pub fn u1_pcie_axi4_mst0_aruser_31_0(&self) -> U1_PCIE_AXI4_MST0_ARUSER_31_0_R {
        U1_PCIE_AXI4_MST0_ARUSER_31_0_R::new(self.bits)
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
#[doc = "STG SYSCONSAIF SYSCFG 540\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_135::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_135::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCFG_135_SPEC;
impl crate::RegisterSpec for STG_SYSCFG_135_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_135::R`](R) reader structure"]
impl crate::Readable for STG_SYSCFG_135_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_135::W`](W) writer structure"]
impl crate::Writable for STG_SYSCFG_135_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets stg_syscfg_135 to value 0"]
impl crate::Resettable for STG_SYSCFG_135_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
