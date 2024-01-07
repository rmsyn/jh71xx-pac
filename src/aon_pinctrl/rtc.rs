#[doc = "Register `rtc` reader"]
pub type R = crate::R<RTC_SPEC>;
#[doc = "Register `rtc` writer"]
pub type W = crate::W<RTC_SPEC>;
#[doc = "Field `ds` reader - Output Drive Strength (DS): * 00: The rated drive strength is 2 mA. * 01: The rated drive strength is 4 mA. * 10: The rated drive strength is 8 mA. * 11: The rated drive strength is 12 mA."]
pub type DS_R = crate::FieldReader;
#[doc = "Field `ds` writer - Output Drive Strength (DS): * 00: The rated drive strength is 2 mA. * 01: The rated drive strength is 4 mA. * 10: The rated drive strength is 8 mA. * 11: The rated drive strength is 12 mA."]
pub type DS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Output Drive Strength (DS): * 00: The rated drive strength is 2 mA. * 01: The rated drive strength is 4 mA. * 10: The rated drive strength is 8 mA. * 11: The rated drive strength is 12 mA."]
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Output Drive Strength (DS): * 00: The rated drive strength is 2 mA. * 01: The rated drive strength is 4 mA. * 10: The rated drive strength is 8 mA. * 11: The rated drive strength is 12 mA."]
    #[inline(always)]
    #[must_use]
    pub fn ds(&mut self) -> DS_W<RTC_SPEC> {
        DS_W::new(self, 0)
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
#[doc = "AON IOMUX CFG SAIF SYSCFG 76\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_SPEC;
impl crate::RegisterSpec for RTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc::R`](R) reader structure"]
impl crate::Readable for RTC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtc::W`](W) writer structure"]
impl crate::Writable for RTC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rtc to value 0"]
impl crate::Resettable for RTC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
