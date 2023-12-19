#[doc = "Register `stg_syscfg_7` reader"]
pub type R = crate::R<STG_SYSCFG_7_SPEC>;
#[doc = "Register `stg_syscfg_7` writer"]
pub type W = crate::W<STG_SYSCFG_7_SPEC>;
#[doc = "Field `u0_e2_nmi_exception_vector` reader - u0_e2_nmi_exception_vector"]
pub type U0_E2_NMI_EXCEPTION_VECTOR_R = crate::FieldReader<u32>;
#[doc = "Field `u0_e2_nmi_exception_vector` writer - u0_e2_nmi_exception_vector"]
pub type U0_E2_NMI_EXCEPTION_VECTOR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - u0_e2_nmi_exception_vector"]
    #[inline(always)]
    pub fn u0_e2_nmi_exception_vector(&self) -> U0_E2_NMI_EXCEPTION_VECTOR_R {
        U0_E2_NMI_EXCEPTION_VECTOR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - u0_e2_nmi_exception_vector"]
    #[inline(always)]
    #[must_use]
    pub fn u0_e2_nmi_exception_vector(
        &mut self,
    ) -> U0_E2_NMI_EXCEPTION_VECTOR_W<STG_SYSCFG_7_SPEC> {
        U0_E2_NMI_EXCEPTION_VECTOR_W::new(self, 0)
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
#[doc = "STG SYSCONSAIF SYSCFG 28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCFG_7_SPEC;
impl crate::RegisterSpec for STG_SYSCFG_7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_7::R`](R) reader structure"]
impl crate::Readable for STG_SYSCFG_7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_7::W`](W) writer structure"]
impl crate::Writable for STG_SYSCFG_7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets stg_syscfg_7 to value 0"]
impl crate::Resettable for STG_SYSCFG_7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
