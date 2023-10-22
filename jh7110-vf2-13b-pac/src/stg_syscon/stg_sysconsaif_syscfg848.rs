#[doc = "Register `stg_sysconsaif_syscfg848` reader"]
pub type R = crate::R<STG_SYSCONSAIF_SYSCFG848_SPEC>;
#[doc = "Register `stg_sysconsaif_syscfg848` writer"]
pub type W = crate::W<STG_SYSCONSAIF_SYSCFG848_SPEC>;
#[doc = "Field `u1_plda_pcie_test_out_bridge_447_416` reader - u1_plda_pcie_test_out_bridge_447_416"]
pub type U1_PLDA_PCIE_TEST_OUT_BRIDGE_447_416_R = crate::FieldReader<u32>;
#[doc = "Field `u1_plda_pcie_test_out_bridge_447_416` writer - u1_plda_pcie_test_out_bridge_447_416"]
pub type U1_PLDA_PCIE_TEST_OUT_BRIDGE_447_416_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - u1_plda_pcie_test_out_bridge_447_416"]
    #[inline(always)]
    pub fn u1_plda_pcie_test_out_bridge_447_416(&self) -> U1_PLDA_PCIE_TEST_OUT_BRIDGE_447_416_R {
        U1_PLDA_PCIE_TEST_OUT_BRIDGE_447_416_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - u1_plda_pcie_test_out_bridge_447_416"]
    #[inline(always)]
    #[must_use]
    pub fn u1_plda_pcie_test_out_bridge_447_416(
        &mut self,
    ) -> U1_PLDA_PCIE_TEST_OUT_BRIDGE_447_416_W<STG_SYSCONSAIF_SYSCFG848_SPEC, 0> {
        U1_PLDA_PCIE_TEST_OUT_BRIDGE_447_416_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 848\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_sysconsaif_syscfg848::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_sysconsaif_syscfg848::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCONSAIF_SYSCFG848_SPEC;
impl crate::RegisterSpec for STG_SYSCONSAIF_SYSCFG848_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_sysconsaif_syscfg848::R`](R) reader structure"]
impl crate::Readable for STG_SYSCONSAIF_SYSCFG848_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_sysconsaif_syscfg848::W`](W) writer structure"]
impl crate::Writable for STG_SYSCONSAIF_SYSCFG848_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}