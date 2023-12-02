#[doc = "Register `aon_sysconsaif_syscfg8` reader"]
pub type R = crate::R<AON_SYSCONSAIF_SYSCFG8_SPEC>;
#[doc = "Register `aon_sysconsaif_syscfg8` writer"]
pub type W = crate::W<AON_SYSCONSAIF_SYSCFG8_SPEC>;
#[doc = "Field `u0_boot_ctrl_boot_vector_31_0` reader - u0_boot_ctrl_boot_vector_31_0"]
pub type U0_BOOT_CTRL_BOOT_VECTOR_31_0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - u0_boot_ctrl_boot_vector_31_0"]
    #[inline(always)]
    pub fn u0_boot_ctrl_boot_vector_31_0(&self) -> U0_BOOT_CTRL_BOOT_VECTOR_31_0_R {
        U0_BOOT_CTRL_BOOT_VECTOR_31_0_R::new(self.bits)
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
#[doc = "AON SYSCONSAIF SYSCFG 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_sysconsaif_syscfg8::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_sysconsaif_syscfg8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AON_SYSCONSAIF_SYSCFG8_SPEC;
impl crate::RegisterSpec for AON_SYSCONSAIF_SYSCFG8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_sysconsaif_syscfg8::R`](R) reader structure"]
impl crate::Readable for AON_SYSCONSAIF_SYSCFG8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aon_sysconsaif_syscfg8::W`](W) writer structure"]
impl crate::Writable for AON_SYSCONSAIF_SYSCFG8_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
