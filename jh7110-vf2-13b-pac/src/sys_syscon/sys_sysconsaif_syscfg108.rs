#[doc = "Register `sys_sysconsaif_syscfg108` reader"]
pub type R = crate::R<SYS_SYSCONSAIF_SYSCFG108_SPEC>;
#[doc = "Register `sys_sysconsaif_syscfg108` writer"]
pub type W = crate::W<SYS_SYSCONSAIF_SYSCFG108_SPEC>;
#[doc = "Field `u0_u7mc_sft7110_reset_vector_1_35_32` reader - u0_u7mc_sft7110_reset_vector_1_35_32"]
pub type U0_U7MC_SFT7110_RESET_VECTOR_1_35_32_R = crate::FieldReader;
#[doc = "Field `u0_u7mc_sft7110_reset_vector_1_35_32` writer - u0_u7mc_sft7110_reset_vector_1_35_32"]
pub type U0_U7MC_SFT7110_RESET_VECTOR_1_35_32_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - u0_u7mc_sft7110_reset_vector_1_35_32"]
    #[inline(always)]
    pub fn u0_u7mc_sft7110_reset_vector_1_35_32(&self) -> U0_U7MC_SFT7110_RESET_VECTOR_1_35_32_R {
        U0_U7MC_SFT7110_RESET_VECTOR_1_35_32_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - u0_u7mc_sft7110_reset_vector_1_35_32"]
    #[inline(always)]
    #[must_use]
    pub fn u0_u7mc_sft7110_reset_vector_1_35_32(
        &mut self,
    ) -> U0_U7MC_SFT7110_RESET_VECTOR_1_35_32_W<SYS_SYSCONSAIF_SYSCFG108_SPEC> {
        U0_U7MC_SFT7110_RESET_VECTOR_1_35_32_W::new(self, 0)
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
#[doc = "SYS SYSCONSAIF SYSCFG 108\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg108::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg108::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_SYSCONSAIF_SYSCFG108_SPEC;
impl crate::RegisterSpec for SYS_SYSCONSAIF_SYSCFG108_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_sysconsaif_syscfg108::R`](R) reader structure"]
impl crate::Readable for SYS_SYSCONSAIF_SYSCFG108_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_sysconsaif_syscfg108::W`](W) writer structure"]
impl crate::Writable for SYS_SYSCONSAIF_SYSCFG108_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
