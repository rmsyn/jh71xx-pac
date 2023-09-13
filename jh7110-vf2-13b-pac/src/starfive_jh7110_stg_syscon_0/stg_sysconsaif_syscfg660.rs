#[doc = "Register `stg_sysconsaif_syscfg660` reader"]
pub type R = crate::R<STG_SYSCONSAIF_SYSCFG660_SPEC>;
#[doc = "Register `stg_sysconsaif_syscfg660` writer"]
pub type W = crate::W<STG_SYSCONSAIF_SYSCFG660_SPEC>;
#[doc = "Field `u1_plda_pcie_k_phyparam_255_224` reader - u1_plda_pcie_k_phyparam_255_224"]
pub type U1_PLDA_PCIE_K_PHYPARAM_255_224_R = crate::FieldReader<u32>;
#[doc = "Field `u1_plda_pcie_k_phyparam_255_224` writer - u1_plda_pcie_k_phyparam_255_224"]
pub type U1_PLDA_PCIE_K_PHYPARAM_255_224_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - u1_plda_pcie_k_phyparam_255_224"]
    #[inline(always)]
    pub fn u1_plda_pcie_k_phyparam_255_224(&self) -> U1_PLDA_PCIE_K_PHYPARAM_255_224_R {
        U1_PLDA_PCIE_K_PHYPARAM_255_224_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - u1_plda_pcie_k_phyparam_255_224"]
    #[inline(always)]
    #[must_use]
    pub fn u1_plda_pcie_k_phyparam_255_224(
        &mut self,
    ) -> U1_PLDA_PCIE_K_PHYPARAM_255_224_W<STG_SYSCONSAIF_SYSCFG660_SPEC, 0> {
        U1_PLDA_PCIE_K_PHYPARAM_255_224_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 660\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_sysconsaif_syscfg660::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_sysconsaif_syscfg660::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCONSAIF_SYSCFG660_SPEC;
impl crate::RegisterSpec for STG_SYSCONSAIF_SYSCFG660_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_sysconsaif_syscfg660::R`](R) reader structure"]
impl crate::Readable for STG_SYSCONSAIF_SYSCFG660_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_sysconsaif_syscfg660::W`](W) writer structure"]
impl crate::Writable for STG_SYSCONSAIF_SYSCFG660_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
