#[doc = "Register `stg_sysconsaif_syscfg912` reader"]
pub type R = crate::R<STG_SYSCONSAIF_SYSCFG912_SPEC>;
#[doc = "Register `stg_sysconsaif_syscfg912` writer"]
pub type W = crate::W<STG_SYSCONSAIF_SYSCFG912_SPEC>;
#[doc = "Field `u1_plda_pcie_test_out_pcie_447_416` reader - u1_plda_pcie_test_out_pcie_447_416"]
pub type U1_PLDA_PCIE_TEST_OUT_PCIE_447_416_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - u1_plda_pcie_test_out_pcie_447_416"]
    #[inline(always)]
    pub fn u1_plda_pcie_test_out_pcie_447_416(&self) -> U1_PLDA_PCIE_TEST_OUT_PCIE_447_416_R {
        U1_PLDA_PCIE_TEST_OUT_PCIE_447_416_R::new(self.bits)
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
#[doc = "STG SYSCONSAIF SYSCFG 912\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_sysconsaif_syscfg912::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_sysconsaif_syscfg912::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCONSAIF_SYSCFG912_SPEC;
impl crate::RegisterSpec for STG_SYSCONSAIF_SYSCFG912_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_sysconsaif_syscfg912::R`](R) reader structure"]
impl crate::Readable for STG_SYSCONSAIF_SYSCFG912_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_sysconsaif_syscfg912::W`](W) writer structure"]
impl crate::Writable for STG_SYSCONSAIF_SYSCFG912_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
