#[doc = "Register `aon_syscfg_4` reader"]
pub type R = crate::R<AON_SYSCFG_4_SPEC>;
#[doc = "Register `aon_syscfg_4` writer"]
pub type W = crate::W<AON_SYSCFG_4_SPEC>;
#[doc = "Field `gmac5_axi64_ptp_timestamp_o_0_31` reader - gmac5_axi64_ptp_timestamp_o_0_31"]
pub type GMAC5_AXI64_PTP_TIMESTAMP_O_0_31_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - gmac5_axi64_ptp_timestamp_o_0_31"]
    #[inline(always)]
    pub fn gmac5_axi64_ptp_timestamp_o_0_31(&self) -> GMAC5_AXI64_PTP_TIMESTAMP_O_0_31_R {
        GMAC5_AXI64_PTP_TIMESTAMP_O_0_31_R::new(self.bits)
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
#[doc = "AON SYSCONSAIF SYSCFG 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_syscfg_4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_syscfg_4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AON_SYSCFG_4_SPEC;
impl crate::RegisterSpec for AON_SYSCFG_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_syscfg_4::R`](R) reader structure"]
impl crate::Readable for AON_SYSCFG_4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aon_syscfg_4::W`](W) writer structure"]
impl crate::Writable for AON_SYSCFG_4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets aon_syscfg_4 to value 0"]
impl crate::Resettable for AON_SYSCFG_4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
