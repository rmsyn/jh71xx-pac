#[doc = "Register `tim` reader"]
pub type R = crate::R<TIM_SPEC>;
#[doc = "Register `tim` writer"]
pub type W = crate::W<TIM_SPEC>;
#[doc = "Field `seq_done_mask` reader - Mask the sequence complete event. 0: mask, 1: unmask"]
pub type SEQ_DONE_MASK_R = crate::BitReader;
#[doc = "Field `seq_done_mask` writer - Mask the sequence complete event. 0: mask, 1: unmask"]
pub type SEQ_DONE_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hw_req_mask` reader - Mask the hardware encouragement request. 0: mask, 1: unmask"]
pub type HW_REQ_MASK_R = crate::BitReader;
#[doc = "Field `hw_req_mask` writer - Mask the hardware encouragement request. 0: mask, 1: unmask"]
pub type HW_REQ_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sw_fail_mask` reader - Mask the software encouragement failure event. 0: mask, 1: unmask"]
pub type SW_FAIL_MASK_R = crate::FieldReader;
#[doc = "Field `sw_fail_mask` writer - Mask the software encouragement failure event. 0: mask, 1: unmask"]
pub type SW_FAIL_MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `hw_fail_mask` reader - Mask the hardware encouragement failure event. 0: mask, 1: unmask"]
pub type HW_FAIL_MASK_R = crate::FieldReader;
#[doc = "Field `hw_fail_mask` writer - Mask the hardware encouragement failure event. 0: mask, 1: unmask"]
pub type HW_FAIL_MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pch_fail_mask` reader - Mask the P-channel encouragement failure event. 0: mask, 1: unmask"]
pub type PCH_FAIL_MASK_R = crate::FieldReader;
#[doc = "Field `pch_fail_mask` writer - Mask the P-channel encouragement failure event. 0: mask, 1: unmask"]
pub type PCH_FAIL_MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Mask the sequence complete event. 0: mask, 1: unmask"]
    #[inline(always)]
    pub fn seq_done_mask(&self) -> SEQ_DONE_MASK_R {
        SEQ_DONE_MASK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask the hardware encouragement request. 0: mask, 1: unmask"]
    #[inline(always)]
    pub fn hw_req_mask(&self) -> HW_REQ_MASK_R {
        HW_REQ_MASK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Mask the software encouragement failure event. 0: mask, 1: unmask"]
    #[inline(always)]
    pub fn sw_fail_mask(&self) -> SW_FAIL_MASK_R {
        SW_FAIL_MASK_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Mask the hardware encouragement failure event. 0: mask, 1: unmask"]
    #[inline(always)]
    pub fn hw_fail_mask(&self) -> HW_FAIL_MASK_R {
        HW_FAIL_MASK_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:8 - Mask the P-channel encouragement failure event. 0: mask, 1: unmask"]
    #[inline(always)]
    pub fn pch_fail_mask(&self) -> PCH_FAIL_MASK_R {
        PCH_FAIL_MASK_R::new(((self.bits >> 6) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Mask the sequence complete event. 0: mask, 1: unmask"]
    #[inline(always)]
    #[must_use]
    pub fn seq_done_mask(&mut self) -> SEQ_DONE_MASK_W<TIM_SPEC> {
        SEQ_DONE_MASK_W::new(self, 0)
    }
    #[doc = "Bit 1 - Mask the hardware encouragement request. 0: mask, 1: unmask"]
    #[inline(always)]
    #[must_use]
    pub fn hw_req_mask(&mut self) -> HW_REQ_MASK_W<TIM_SPEC> {
        HW_REQ_MASK_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Mask the software encouragement failure event. 0: mask, 1: unmask"]
    #[inline(always)]
    #[must_use]
    pub fn sw_fail_mask(&mut self) -> SW_FAIL_MASK_W<TIM_SPEC> {
        SW_FAIL_MASK_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Mask the hardware encouragement failure event. 0: mask, 1: unmask"]
    #[inline(always)]
    #[must_use]
    pub fn hw_fail_mask(&mut self) -> HW_FAIL_MASK_W<TIM_SPEC> {
        HW_FAIL_MASK_W::new(self, 4)
    }
    #[doc = "Bits 6:8 - Mask the P-channel encouragement failure event. 0: mask, 1: unmask"]
    #[inline(always)]
    #[must_use]
    pub fn pch_fail_mask(&mut self) -> PCH_FAIL_MASK_W<TIM_SPEC> {
        PCH_FAIL_MASK_W::new(self, 6)
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
#[doc = "TIMER Interrupt Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM_SPEC;
impl crate::RegisterSpec for TIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim::R`](R) reader structure"]
impl crate::Readable for TIM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tim::W`](W) writer structure"]
impl crate::Writable for TIM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tim to value 0"]
impl crate::Resettable for TIM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
