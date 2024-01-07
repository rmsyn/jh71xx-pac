#[doc = "Register `stg_syscfg_162` reader"]
pub type R = crate::R<STG_SYSCFG_162_SPEC>;
#[doc = "Register `stg_syscfg_162` writer"]
pub type W = crate::W<STG_SYSCFG_162_SPEC>;
#[doc = "Field `u1_plda_pcie_k_phyparam_159_128` reader - u1_plda_pcie_k_phyparam_159_128"]
pub type U1_PLDA_PCIE_K_PHYPARAM_159_128_R = crate::FieldReader<u32>;
#[doc = "Field `u1_plda_pcie_k_phyparam_159_128` writer - u1_plda_pcie_k_phyparam_159_128"]
pub type U1_PLDA_PCIE_K_PHYPARAM_159_128_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - u1_plda_pcie_k_phyparam_159_128"]
    #[inline(always)]
    pub fn u1_plda_pcie_k_phyparam_159_128(&self) -> U1_PLDA_PCIE_K_PHYPARAM_159_128_R {
        U1_PLDA_PCIE_K_PHYPARAM_159_128_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - u1_plda_pcie_k_phyparam_159_128"]
    #[inline(always)]
    #[must_use]
    pub fn u1_plda_pcie_k_phyparam_159_128(
        &mut self,
    ) -> U1_PLDA_PCIE_K_PHYPARAM_159_128_W<STG_SYSCFG_162_SPEC> {
        U1_PLDA_PCIE_K_PHYPARAM_159_128_W::new(self, 0)
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
#[doc = "STG SYSCONSAIF SYSCFG 648\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_162::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_162::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCFG_162_SPEC;
impl crate::RegisterSpec for STG_SYSCFG_162_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_162::R`](R) reader structure"]
impl crate::Readable for STG_SYSCFG_162_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_162::W`](W) writer structure"]
impl crate::Writable for STG_SYSCFG_162_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets stg_syscfg_162 to value 0"]
impl crate::Resettable for STG_SYSCFG_162_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
