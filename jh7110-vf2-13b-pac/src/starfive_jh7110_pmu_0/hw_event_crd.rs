#[doc = "Register `hw_event_crd` reader"]
pub type R = crate::R<HW_EVENT_CRD_SPEC>;
#[doc = "Register `hw_event_crd` writer"]
pub type W = crate::W<HW_EVENT_CRD_SPEC>;
#[doc = "Field `hw_event_crd` reader - Hardware Event Record."]
pub type HW_EVENT_CRD_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Hardware Event Record."]
    #[inline(always)]
    pub fn hw_event_crd(&self) -> HW_EVENT_CRD_R {
        HW_EVENT_CRD_R::new((self.bits & 0xff) as u8)
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
#[doc = "Hardware Event Record\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hw_event_crd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hw_event_crd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HW_EVENT_CRD_SPEC;
impl crate::RegisterSpec for HW_EVENT_CRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hw_event_crd::R`](R) reader structure"]
impl crate::Readable for HW_EVENT_CRD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hw_event_crd::W`](W) writer structure"]
impl crate::Writable for HW_EVENT_CRD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hw_event_crd to value 0"]
impl crate::Resettable for HW_EVENT_CRD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
