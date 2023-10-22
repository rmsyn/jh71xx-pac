#[doc = "Register `stg_sysconsaif_syscfg220` reader"]
pub type R = crate::R<STG_SYSCONSAIF_SYSCFG220_SPEC>;
#[doc = "Register `stg_sysconsaif_syscfg220` writer"]
pub type W = crate::W<STG_SYSCONSAIF_SYSCFG220_SPEC>;
#[doc = "Field `u0_plda_pcie_k_phyparam_191_160` reader - u0_plda_pcie_k_phyparam_191_160"]
pub type U0_PLDA_PCIE_K_PHYPARAM_191_160_R = crate::FieldReader<u32>;
#[doc = "Field `u0_plda_pcie_k_phyparam_191_160` writer - u0_plda_pcie_k_phyparam_191_160"]
pub type U0_PLDA_PCIE_K_PHYPARAM_191_160_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - u0_plda_pcie_k_phyparam_191_160"]
    #[inline(always)]
    pub fn u0_plda_pcie_k_phyparam_191_160(&self) -> U0_PLDA_PCIE_K_PHYPARAM_191_160_R {
        U0_PLDA_PCIE_K_PHYPARAM_191_160_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - u0_plda_pcie_k_phyparam_191_160"]
    #[inline(always)]
    #[must_use]
    pub fn u0_plda_pcie_k_phyparam_191_160(
        &mut self,
    ) -> U0_PLDA_PCIE_K_PHYPARAM_191_160_W<STG_SYSCONSAIF_SYSCFG220_SPEC, 0> {
        U0_PLDA_PCIE_K_PHYPARAM_191_160_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 220\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_sysconsaif_syscfg220::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_sysconsaif_syscfg220::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCONSAIF_SYSCFG220_SPEC;
impl crate::RegisterSpec for STG_SYSCONSAIF_SYSCFG220_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_sysconsaif_syscfg220::R`](R) reader structure"]
impl crate::Readable for STG_SYSCONSAIF_SYSCFG220_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_sysconsaif_syscfg220::W`](W) writer structure"]
impl crate::Writable for STG_SYSCONSAIF_SYSCFG220_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
