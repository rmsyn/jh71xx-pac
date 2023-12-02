#[doc = "Register `stg_sysconsaif_syscfg724` reader"]
pub type R = crate::R<STG_SYSCONSAIF_SYSCFG724_SPEC>;
#[doc = "Register `stg_sysconsaif_syscfg724` writer"]
pub type W = crate::W<STG_SYSCONSAIF_SYSCFG724_SPEC>;
#[doc = "Field `u1_plda_pcie_k_phyparam_767_736` reader - u1_plda_pcie_k_phyparam_767_736"]
pub type U1_PLDA_PCIE_K_PHYPARAM_767_736_R = crate::FieldReader<u32>;
#[doc = "Field `u1_plda_pcie_k_phyparam_767_736` writer - u1_plda_pcie_k_phyparam_767_736"]
pub type U1_PLDA_PCIE_K_PHYPARAM_767_736_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - u1_plda_pcie_k_phyparam_767_736"]
    #[inline(always)]
    pub fn u1_plda_pcie_k_phyparam_767_736(&self) -> U1_PLDA_PCIE_K_PHYPARAM_767_736_R {
        U1_PLDA_PCIE_K_PHYPARAM_767_736_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - u1_plda_pcie_k_phyparam_767_736"]
    #[inline(always)]
    #[must_use]
    pub fn u1_plda_pcie_k_phyparam_767_736(
        &mut self,
    ) -> U1_PLDA_PCIE_K_PHYPARAM_767_736_W<STG_SYSCONSAIF_SYSCFG724_SPEC> {
        U1_PLDA_PCIE_K_PHYPARAM_767_736_W::new(self, 0)
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
#[doc = "STG SYSCONSAIF SYSCFG 724\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_sysconsaif_syscfg724::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_sysconsaif_syscfg724::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCONSAIF_SYSCFG724_SPEC;
impl crate::RegisterSpec for STG_SYSCONSAIF_SYSCFG724_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_sysconsaif_syscfg724::R`](R) reader structure"]
impl crate::Readable for STG_SYSCONSAIF_SYSCFG724_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_sysconsaif_syscfg724::W`](W) writer structure"]
impl crate::Writable for STG_SYSCONSAIF_SYSCFG724_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
