#[doc = "Register `sys_syscfg_37` reader"]
pub type R = crate::R<SYS_SYSCFG_37_SPEC>;
#[doc = "Register `sys_syscfg_37` writer"]
pub type W = crate::W<SYS_SYSCFG_37_SPEC>;
#[doc = "Field `gmac5_axi64_ptp_timestamp_0_31` reader - gmac5_axi64_ptp_timestamp_0_31"]
pub type GMAC5_AXI64_PTP_TIMESTAMP_0_31_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - gmac5_axi64_ptp_timestamp_0_31"]
    #[inline(always)]
    pub fn gmac5_axi64_ptp_timestamp_0_31(&self) -> GMAC5_AXI64_PTP_TIMESTAMP_0_31_R {
        GMAC5_AXI64_PTP_TIMESTAMP_0_31_R::new(self.bits)
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
#[doc = "SYS SYSCONSAIF SYSCFG 148\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_37::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_37::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_SYSCFG_37_SPEC;
impl crate::RegisterSpec for SYS_SYSCFG_37_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg_37::R`](R) reader structure"]
impl crate::Readable for SYS_SYSCFG_37_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_syscfg_37::W`](W) writer structure"]
impl crate::Writable for SYS_SYSCFG_37_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sys_syscfg_37 to value 0"]
impl crate::Resettable for SYS_SYSCFG_37_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
