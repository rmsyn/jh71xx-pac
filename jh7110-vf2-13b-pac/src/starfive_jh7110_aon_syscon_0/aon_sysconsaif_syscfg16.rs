#[doc = "Register `aon_sysconsaif_syscfg16` reader"]
pub type R = crate::R<AON_SYSCONSAIF_SYSCFG16_SPEC>;
#[doc = "Register `aon_sysconsaif_syscfg16` writer"]
pub type W = crate::W<AON_SYSCONSAIF_SYSCFG16_SPEC>;
#[doc = "Field `gmac5_axi64_ptp_timestamp_o_31_0` reader - gmac5_axi64_ptp_timestamp_o_31_0"]
pub type GMAC5_AXI64_PTP_TIMESTAMP_O_31_0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - gmac5_axi64_ptp_timestamp_o_31_0"]
    #[inline(always)]
    pub fn gmac5_axi64_ptp_timestamp_o_31_0(&self) -> GMAC5_AXI64_PTP_TIMESTAMP_O_31_0_R {
        GMAC5_AXI64_PTP_TIMESTAMP_O_31_0_R::new(self.bits)
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
#[doc = "AON SYSCONSAIF SYSCFG 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_sysconsaif_syscfg16::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_sysconsaif_syscfg16::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AON_SYSCONSAIF_SYSCFG16_SPEC;
impl crate::RegisterSpec for AON_SYSCONSAIF_SYSCFG16_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_sysconsaif_syscfg16::R`](R) reader structure"]
impl crate::Readable for AON_SYSCONSAIF_SYSCFG16_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aon_sysconsaif_syscfg16::W`](W) writer structure"]
impl crate::Writable for AON_SYSCONSAIF_SYSCFG16_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
