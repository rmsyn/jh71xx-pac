#[doc = "Register `stg_sysconsaif_syscfg392` reader"]
pub type R = crate::R<STG_SYSCONSAIF_SYSCFG392_SPEC>;
#[doc = "Register `stg_sysconsaif_syscfg392` writer"]
pub type W = crate::W<STG_SYSCONSAIF_SYSCFG392_SPEC>;
#[doc = "Field `u0_plda_pcie_test_out_bridge_255_224` reader - u0_plda_pcie_test_out_bridge_255_224"]
pub type U0_PLDA_PCIE_TEST_OUT_BRIDGE_255_224_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - u0_plda_pcie_test_out_bridge_255_224"]
    #[inline(always)]
    pub fn u0_plda_pcie_test_out_bridge_255_224(&self) -> U0_PLDA_PCIE_TEST_OUT_BRIDGE_255_224_R {
        U0_PLDA_PCIE_TEST_OUT_BRIDGE_255_224_R::new(self.bits)
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
#[doc = "STG SYSCONSAIF SYSCFG 392\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_sysconsaif_syscfg392::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_sysconsaif_syscfg392::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCONSAIF_SYSCFG392_SPEC;
impl crate::RegisterSpec for STG_SYSCONSAIF_SYSCFG392_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_sysconsaif_syscfg392::R`](R) reader structure"]
impl crate::Readable for STG_SYSCONSAIF_SYSCFG392_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_sysconsaif_syscfg392::W`](W) writer structure"]
impl crate::Writable for STG_SYSCONSAIF_SYSCFG392_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
