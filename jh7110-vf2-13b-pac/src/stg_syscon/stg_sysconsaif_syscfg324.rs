#[doc = "Register `stg_sysconsaif_syscfg324` reader"]
pub type R = crate::R<STG_SYSCONSAIF_SYSCFG324_SPEC>;
#[doc = "Register `stg_sysconsaif_syscfg324` writer"]
pub type W = crate::W<STG_SYSCONSAIF_SYSCFG324_SPEC>;
#[doc = "Field `u0_plda_pcie_pf2_offset` reader - u0_plda_pcie_pf2_offset"]
pub type U0_PLDA_PCIE_PF2_OFFSET_R = crate::FieldReader<u32>;
#[doc = "Field `u0_plda_pcie_pf2_offset` writer - u0_plda_pcie_pf2_offset"]
pub type U0_PLDA_PCIE_PF2_OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - u0_plda_pcie_pf2_offset"]
    #[inline(always)]
    pub fn u0_plda_pcie_pf2_offset(&self) -> U0_PLDA_PCIE_PF2_OFFSET_R {
        U0_PLDA_PCIE_PF2_OFFSET_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - u0_plda_pcie_pf2_offset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_plda_pcie_pf2_offset(
        &mut self,
    ) -> U0_PLDA_PCIE_PF2_OFFSET_W<STG_SYSCONSAIF_SYSCFG324_SPEC> {
        U0_PLDA_PCIE_PF2_OFFSET_W::new(self, 0)
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
#[doc = "STG SYSCONSAIF SYSCFG 324\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_sysconsaif_syscfg324::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_sysconsaif_syscfg324::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCONSAIF_SYSCFG324_SPEC;
impl crate::RegisterSpec for STG_SYSCONSAIF_SYSCFG324_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_sysconsaif_syscfg324::R`](R) reader structure"]
impl crate::Readable for STG_SYSCONSAIF_SYSCFG324_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_sysconsaif_syscfg324::W`](W) writer structure"]
impl crate::Writable for STG_SYSCONSAIF_SYSCFG324_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
