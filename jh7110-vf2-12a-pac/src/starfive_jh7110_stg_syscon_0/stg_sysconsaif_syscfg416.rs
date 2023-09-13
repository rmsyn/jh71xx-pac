#[doc = "Register `stg_sysconsaif_syscfg416` reader"]
pub type R = crate::R<STG_SYSCONSAIF_SYSCFG416_SPEC>;
#[doc = "Register `stg_sysconsaif_syscfg416` writer"]
pub type W = crate::W<STG_SYSCONSAIF_SYSCFG416_SPEC>;
#[doc = "Field `u0_plda_pcie_test_out_bridge_447_416` reader - u0_plda_pcie_test_out_bridge_447_416"]
pub type U0_PLDA_PCIE_TEST_OUT_BRIDGE_447_416_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - u0_plda_pcie_test_out_bridge_447_416"]
    #[inline(always)]
    pub fn u0_plda_pcie_test_out_bridge_447_416(&self) -> U0_PLDA_PCIE_TEST_OUT_BRIDGE_447_416_R {
        U0_PLDA_PCIE_TEST_OUT_BRIDGE_447_416_R::new(self.bits)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 416\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_sysconsaif_syscfg416::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_sysconsaif_syscfg416::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCONSAIF_SYSCFG416_SPEC;
impl crate::RegisterSpec for STG_SYSCONSAIF_SYSCFG416_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_sysconsaif_syscfg416::R`](R) reader structure"]
impl crate::Readable for STG_SYSCONSAIF_SYSCFG416_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_sysconsaif_syscfg416::W`](W) writer structure"]
impl crate::Writable for STG_SYSCONSAIF_SYSCFG416_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
