#[doc = "Register `int_status` reader"]
pub type R = crate::R<INT_STATUS_SPEC>;
#[doc = "Register `int_status` writer"]
pub type W = crate::W<INT_STATUS_SPEC>;
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
#[doc = "PMU Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_STATUS_SPEC;
impl crate::RegisterSpec for INT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_status::R`](R) reader structure"]
impl crate::Readable for INT_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_status::W`](W) writer structure"]
impl crate::Writable for INT_STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets int_status to value 0"]
impl crate::Resettable for INT_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
