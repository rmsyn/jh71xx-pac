#[doc = "Register `padcfg_gmac1_mdc_syscon` reader"]
pub type R = crate::R<PADCFG_GMAC1_MDC_SYSCON_SPEC>;
#[doc = "Register `padcfg_gmac1_mdc_syscon` writer"]
pub type W = crate::W<PADCFG_GMAC1_MDC_SYSCON_SPEC>;
#[doc = "Field `padcfg_pad_gmac1_mdc_syscon` reader - padcfg_pad_gmac1_mdc_syscon"]
pub type PADCFG_PAD_GMAC1_MDC_SYSCON_R = crate::FieldReader;
#[doc = "Field `padcfg_pad_gmac1_mdc_syscon` writer - padcfg_pad_gmac1_mdc_syscon"]
pub type PADCFG_PAD_GMAC1_MDC_SYSCON_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - padcfg_pad_gmac1_mdc_syscon"]
    #[inline(always)]
    pub fn padcfg_pad_gmac1_mdc_syscon(&self) -> PADCFG_PAD_GMAC1_MDC_SYSCON_R {
        PADCFG_PAD_GMAC1_MDC_SYSCON_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - padcfg_pad_gmac1_mdc_syscon"]
    #[inline(always)]
    #[must_use]
    pub fn padcfg_pad_gmac1_mdc_syscon(
        &mut self,
    ) -> PADCFG_PAD_GMAC1_MDC_SYSCON_W<PADCFG_GMAC1_MDC_SYSCON_SPEC> {
        PADCFG_PAD_GMAC1_MDC_SYSCON_W::new(self, 0)
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
#[doc = "GPIO GMAC1 MDC Pad Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gmac1_mdc_syscon::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gmac1_mdc_syscon::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PADCFG_GMAC1_MDC_SYSCON_SPEC;
impl crate::RegisterSpec for PADCFG_GMAC1_MDC_SYSCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padcfg_gmac1_mdc_syscon::R`](R) reader structure"]
impl crate::Readable for PADCFG_GMAC1_MDC_SYSCON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`padcfg_gmac1_mdc_syscon::W`](W) writer structure"]
impl crate::Writable for PADCFG_GMAC1_MDC_SYSCON_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets padcfg_gmac1_mdc_syscon to value 0x02"]
impl crate::Resettable for PADCFG_GMAC1_MDC_SYSCON_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
