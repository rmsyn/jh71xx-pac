#[doc = "Register `stg_syscfg_178` reader"]
pub type R = crate::R<STG_SYSCFG_178_SPEC>;
#[doc = "Register `stg_syscfg_178` writer"]
pub type W = crate::W<STG_SYSCFG_178_SPEC>;
#[doc = "Field `u1_plda_pcie_k_phyparam_671_640` reader - u1_plda_pcie_k_phyparam_671_640"]
pub type U1_PLDA_PCIE_K_PHYPARAM_671_640_R = crate::FieldReader<u32>;
#[doc = "Field `u1_plda_pcie_k_phyparam_671_640` writer - u1_plda_pcie_k_phyparam_671_640"]
pub type U1_PLDA_PCIE_K_PHYPARAM_671_640_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - u1_plda_pcie_k_phyparam_671_640"]
    #[inline(always)]
    pub fn u1_plda_pcie_k_phyparam_671_640(&self) -> U1_PLDA_PCIE_K_PHYPARAM_671_640_R {
        U1_PLDA_PCIE_K_PHYPARAM_671_640_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - u1_plda_pcie_k_phyparam_671_640"]
    #[inline(always)]
    #[must_use]
    pub fn u1_plda_pcie_k_phyparam_671_640(
        &mut self,
    ) -> U1_PLDA_PCIE_K_PHYPARAM_671_640_W<STG_SYSCFG_178_SPEC> {
        U1_PLDA_PCIE_K_PHYPARAM_671_640_W::new(self, 0)
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
#[doc = "STG SYSCONSAIF SYSCFG 712\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_178::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_178::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCFG_178_SPEC;
impl crate::RegisterSpec for STG_SYSCFG_178_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_178::R`](R) reader structure"]
impl crate::Readable for STG_SYSCFG_178_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_178::W`](W) writer structure"]
impl crate::Writable for STG_SYSCFG_178_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets stg_syscfg_178 to value 0"]
impl crate::Resettable for STG_SYSCFG_178_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
