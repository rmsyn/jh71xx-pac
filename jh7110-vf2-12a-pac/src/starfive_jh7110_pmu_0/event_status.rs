#[doc = "Register `event_status` reader"]
pub type R = crate::R<EVENT_STATUS_SPEC>;
#[doc = "Register `event_status` writer"]
pub type W = crate::W<EVENT_STATUS_SPEC>;
#[doc = "Field `seq_done_event` reader - Sequence complete."]
pub type SEQ_DONE_EVENT_R = crate::BitReader;
#[doc = "Field `hw_req_event` reader - Hardware encouragement request."]
pub type HW_REQ_EVENT_R = crate::BitReader;
#[doc = "Field `sw_fail_event` reader - Software encouragement failure."]
pub type SW_FAIL_EVENT_R = crate::FieldReader;
#[doc = "Field `hw_fail_event` reader - Hardware encouragement failure."]
pub type HW_FAIL_EVENT_R = crate::FieldReader;
#[doc = "Field `pch_fail_event` reader - P-channel failure."]
pub type PCH_FAIL_EVENT_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Sequence complete."]
    #[inline(always)]
    pub fn seq_done_event(&self) -> SEQ_DONE_EVENT_R {
        SEQ_DONE_EVENT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Hardware encouragement request."]
    #[inline(always)]
    pub fn hw_req_event(&self) -> HW_REQ_EVENT_R {
        HW_REQ_EVENT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Software encouragement failure."]
    #[inline(always)]
    pub fn sw_fail_event(&self) -> SW_FAIL_EVENT_R {
        SW_FAIL_EVENT_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Hardware encouragement failure."]
    #[inline(always)]
    pub fn hw_fail_event(&self) -> HW_FAIL_EVENT_R {
        HW_FAIL_EVENT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:8 - P-channel failure."]
    #[inline(always)]
    pub fn pch_fail_event(&self) -> PCH_FAIL_EVENT_R {
        PCH_FAIL_EVENT_R::new(((self.bits >> 6) & 7) as u8)
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
#[doc = "PMU Event Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`event_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`event_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EVENT_STATUS_SPEC;
impl crate::RegisterSpec for EVENT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`event_status::R`](R) reader structure"]
impl crate::Readable for EVENT_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`event_status::W`](W) writer structure"]
impl crate::Writable for EVENT_STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets event_status to value 0"]
impl crate::Resettable for EVENT_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
