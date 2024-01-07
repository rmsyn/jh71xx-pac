#[doc = "Register `gmac1_mdc` reader"]
pub type R = crate::R<GMAC1_MDC_SPEC>;
#[doc = "Register `gmac1_mdc` writer"]
pub type W = crate::W<GMAC1_MDC_SPEC>;
#[doc = "Field `cfg` reader - cfg"]
pub type CFG_R = crate::FieldReader;
#[doc = "Field `cfg` writer - cfg"]
pub type CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - cfg"]
    #[inline(always)]
    pub fn cfg(&self) -> CFG_R {
        CFG_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - cfg"]
    #[inline(always)]
    #[must_use]
    pub fn cfg(&mut self) -> CFG_W<GMAC1_MDC_SPEC> {
        CFG_W::new(self, 0)
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
#[doc = "GPIO GMAC1 MDC Pad Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac1_mdc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac1_mdc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GMAC1_MDC_SPEC;
impl crate::RegisterSpec for GMAC1_MDC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac1_mdc::R`](R) reader structure"]
impl crate::Readable for GMAC1_MDC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gmac1_mdc::W`](W) writer structure"]
impl crate::Writable for GMAC1_MDC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gmac1_mdc to value 0x02"]
impl crate::Resettable for GMAC1_MDC_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
