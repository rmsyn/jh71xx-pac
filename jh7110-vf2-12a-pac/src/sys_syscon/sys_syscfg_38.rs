#[doc = "Register `sys_syscfg_38` reader"]
pub type R = crate::R<SYS_SYSCFG_38_SPEC>;
#[doc = "Register `sys_syscfg_38` writer"]
pub type W = crate::W<SYS_SYSCFG_38_SPEC>;
#[doc = "Field `u1_gmac5_axi64_ptp_timestamp_o_63_32` reader - u1_gmac5_axi64_ptp_timestamp_o_63_32"]
pub type U1_GMAC5_AXI64_PTP_TIMESTAMP_O_63_32_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - u1_gmac5_axi64_ptp_timestamp_o_63_32"]
    #[inline(always)]
    pub fn u1_gmac5_axi64_ptp_timestamp_o_63_32(&self) -> U1_GMAC5_AXI64_PTP_TIMESTAMP_O_63_32_R {
        U1_GMAC5_AXI64_PTP_TIMESTAMP_O_63_32_R::new(self.bits)
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
#[doc = "SYS SYSCONSAIF SYSCFG 152\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_38::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_38::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_SYSCFG_38_SPEC;
impl crate::RegisterSpec for SYS_SYSCFG_38_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg_38::R`](R) reader structure"]
impl crate::Readable for SYS_SYSCFG_38_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_syscfg_38::W`](W) writer structure"]
impl crate::Writable for SYS_SYSCFG_38_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sys_syscfg_38 to value 0"]
impl crate::Resettable for SYS_SYSCFG_38_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
