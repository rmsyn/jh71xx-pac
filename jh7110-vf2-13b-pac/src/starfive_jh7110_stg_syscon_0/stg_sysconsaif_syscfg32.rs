#[doc = "Register `stg_sysconsaif_syscfg32` reader"]
pub type R = crate::R<STG_SYSCONSAIF_SYSCFG32_SPEC>;
#[doc = "Register `stg_sysconsaif_syscfg32` writer"]
pub type W = crate::W<STG_SYSCONSAIF_SYSCFG32_SPEC>;
#[doc = "Field `u0_e2_sft7110_nmi_0_rnmi_interrupt_vector` reader - u0_e2_sft7110_nmi_0_rnmi_interrupt_vector"]
pub type U0_E2_SFT7110_NMI_0_RNMI_INTERRUPT_VECTOR_R = crate::FieldReader<u32>;
#[doc = "Field `u0_e2_sft7110_nmi_0_rnmi_interrupt_vector` writer - u0_e2_sft7110_nmi_0_rnmi_interrupt_vector"]
pub type U0_E2_SFT7110_NMI_0_RNMI_INTERRUPT_VECTOR_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - u0_e2_sft7110_nmi_0_rnmi_interrupt_vector"]
    #[inline(always)]
    pub fn u0_e2_sft7110_nmi_0_rnmi_interrupt_vector(
        &self,
    ) -> U0_E2_SFT7110_NMI_0_RNMI_INTERRUPT_VECTOR_R {
        U0_E2_SFT7110_NMI_0_RNMI_INTERRUPT_VECTOR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - u0_e2_sft7110_nmi_0_rnmi_interrupt_vector"]
    #[inline(always)]
    #[must_use]
    pub fn u0_e2_sft7110_nmi_0_rnmi_interrupt_vector(
        &mut self,
    ) -> U0_E2_SFT7110_NMI_0_RNMI_INTERRUPT_VECTOR_W<STG_SYSCONSAIF_SYSCFG32_SPEC, 0> {
        U0_E2_SFT7110_NMI_0_RNMI_INTERRUPT_VECTOR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_sysconsaif_syscfg32::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_sysconsaif_syscfg32::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCONSAIF_SYSCFG32_SPEC;
impl crate::RegisterSpec for STG_SYSCONSAIF_SYSCFG32_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_sysconsaif_syscfg32::R`](R) reader structure"]
impl crate::Readable for STG_SYSCONSAIF_SYSCFG32_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_sysconsaif_syscfg32::W`](W) writer structure"]
impl crate::Writable for STG_SYSCONSAIF_SYSCFG32_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
