#[doc = "Register `stg_syscfg_29` reader"]
pub type R = crate::R<STG_SYSCFG_29_SPEC>;
#[doc = "Register `stg_syscfg_29` writer"]
pub type W = crate::W<STG_SYSCFG_29_SPEC>;
#[doc = "Field `u0_pcie_axi4_mst0_awfunc` reader - u0_pcie_axi4_mst0_awfunc"]
pub type U0_PCIE_AXI4_MST0_AWFUNC_R = crate::FieldReader<u16>;
#[doc = "Field `u0_pcie_axi4_mst0_awregion` reader - u0_pcie_axi4_mst0_awregion"]
pub type U0_PCIE_AXI4_MST0_AWREGION_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:14 - u0_pcie_axi4_mst0_awfunc"]
    #[inline(always)]
    pub fn u0_pcie_axi4_mst0_awfunc(&self) -> U0_PCIE_AXI4_MST0_AWFUNC_R {
        U0_PCIE_AXI4_MST0_AWFUNC_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 15:18 - u0_pcie_axi4_mst0_awregion"]
    #[inline(always)]
    pub fn u0_pcie_axi4_mst0_awregion(&self) -> U0_PCIE_AXI4_MST0_AWREGION_R {
        U0_PCIE_AXI4_MST0_AWREGION_R::new(((self.bits >> 15) & 0x0f) as u8)
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
#[doc = "STG SYSCONSAIF SYSCFG 116\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_29::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_29::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCFG_29_SPEC;
impl crate::RegisterSpec for STG_SYSCFG_29_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_29::R`](R) reader structure"]
impl crate::Readable for STG_SYSCFG_29_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_29::W`](W) writer structure"]
impl crate::Writable for STG_SYSCFG_29_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets stg_syscfg_29 to value 0"]
impl crate::Resettable for STG_SYSCFG_29_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
