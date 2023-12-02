#[doc = "Register `aon_sysconsaif_syscfg4` reader"]
pub type R = crate::R<AON_SYSCONSAIF_SYSCFG4_SPEC>;
#[doc = "Register `aon_sysconsaif_syscfg4` writer"]
pub type W = crate::W<AON_SYSCONSAIF_SYSCFG4_SPEC>;
#[doc = "Field `u0_boot_ctrl_boot_status` reader - u0_boot_ctrl_boot_status"]
pub type U0_BOOT_CTRL_BOOT_STATUS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - u0_boot_ctrl_boot_status"]
    #[inline(always)]
    pub fn u0_boot_ctrl_boot_status(&self) -> U0_BOOT_CTRL_BOOT_STATUS_R {
        U0_BOOT_CTRL_BOOT_STATUS_R::new((self.bits & 0x0f) as u8)
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
#[doc = "AON SYSCONSAIF SYSCFG 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_sysconsaif_syscfg4::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_sysconsaif_syscfg4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AON_SYSCONSAIF_SYSCFG4_SPEC;
impl crate::RegisterSpec for AON_SYSCONSAIF_SYSCFG4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_sysconsaif_syscfg4::R`](R) reader structure"]
impl crate::Readable for AON_SYSCONSAIF_SYSCFG4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aon_sysconsaif_syscfg4::W`](W) writer structure"]
impl crate::Writable for AON_SYSCONSAIF_SYSCFG4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
