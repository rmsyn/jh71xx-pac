#[doc = "Register `hard_event_turn_on_mask` reader"]
pub type R = crate::R<HARD_EVENT_TURN_ON_MASK_SPEC>;
#[doc = "Register `hard_event_turn_on_mask` writer"]
pub type W = crate::W<HARD_EVENT_TURN_ON_MASK_SPEC>;
#[doc = "Field `hard_event_0_on_mask` reader - RTC event encourage turn-on sequence, 1: mask hardware event, 0: enable hardware event"]
pub type HARD_EVENT_0_ON_MASK_R = crate::BitReader;
#[doc = "Field `hard_event_0_on_mask` writer - RTC event encourage turn-on sequence, 1: mask hardware event, 0: enable hardware event"]
pub type HARD_EVENT_0_ON_MASK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `hard_event_1_on_mask` reader - GMAC event encourage turn-on sequence, 1: mask hardware event, 0: enable hardware event"]
pub type HARD_EVENT_1_ON_MASK_R = crate::BitReader;
#[doc = "Field `hard_event_1_on_mask` writer - GMAC event encourage turn-on sequence, 1: mask hardware event, 0: enable hardware event"]
pub type HARD_EVENT_1_ON_MASK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `hard_event_2_on_mask` reader - RFU, 1: mask hardware event, 0: enable hardware event"]
pub type HARD_EVENT_2_ON_MASK_R = crate::BitReader;
#[doc = "Field `hard_event_2_on_mask` writer - RFU, 1: mask hardware event, 0: enable hardware event"]
pub type HARD_EVENT_2_ON_MASK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `hard_event_3_on_mask` reader - RGPIO0 event encourage turn-on sequence, 1: mask hardware event, 0: enable hardware event"]
pub type HARD_EVENT_3_ON_MASK_R = crate::BitReader;
#[doc = "Field `hard_event_3_on_mask` writer - RGPIO0 event encourage turn-on sequence, 1: mask hardware event, 0: enable hardware event"]
pub type HARD_EVENT_3_ON_MASK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `hard_event_4_on_mask` reader - RGPIO1 event encourage turn-on sequence, 1: mask hardware event, 0: enable hardware event"]
pub type HARD_EVENT_4_ON_MASK_R = crate::BitReader;
#[doc = "Field `hard_event_4_on_mask` writer - RGPIO1 event encourage turn-on sequence, 1: mask hardware event, 0: enable hardware event"]
pub type HARD_EVENT_4_ON_MASK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `hard_event_5_on_mask` reader - RGPIO2 event encourage turn-on sequence, 1: mask hardware event, 0: enable hardware event"]
pub type HARD_EVENT_5_ON_MASK_R = crate::BitReader;
#[doc = "Field `hard_event_5_on_mask` writer - RGPIO2 event encourage turn-on sequence, 1: mask hardware event, 0: enable hardware event"]
pub type HARD_EVENT_5_ON_MASK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `hard_event_6_on_mask` reader - RGPIO3 event encourage turn-on sequence, 1: mask hardware event, 0: enable hardware event"]
pub type HARD_EVENT_6_ON_MASK_R = crate::BitReader;
#[doc = "Field `hard_event_6_on_mask` writer - RGPIO3 event encourage turn-on sequence, 1: mask hardware event, 0: enable hardware event"]
pub type HARD_EVENT_6_ON_MASK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `hard_event_7_on_mask` reader - GPU event, 1: mask hardware event, 0: enable hardware event"]
pub type HARD_EVENT_7_ON_MASK_R = crate::BitReader;
#[doc = "Field `hard_event_7_on_mask` writer - GPU event, 1: mask hardware event, 0: enable hardware event"]
pub type HARD_EVENT_7_ON_MASK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - RTC event encourage turn-on sequence, 1: mask hardware event, 0: enable hardware event"]
    #[inline(always)]
    pub fn hard_event_0_on_mask(&self) -> HARD_EVENT_0_ON_MASK_R {
        HARD_EVENT_0_ON_MASK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GMAC event encourage turn-on sequence, 1: mask hardware event, 0: enable hardware event"]
    #[inline(always)]
    pub fn hard_event_1_on_mask(&self) -> HARD_EVENT_1_ON_MASK_R {
        HARD_EVENT_1_ON_MASK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RFU, 1: mask hardware event, 0: enable hardware event"]
    #[inline(always)]
    pub fn hard_event_2_on_mask(&self) -> HARD_EVENT_2_ON_MASK_R {
        HARD_EVENT_2_ON_MASK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RGPIO0 event encourage turn-on sequence, 1: mask hardware event, 0: enable hardware event"]
    #[inline(always)]
    pub fn hard_event_3_on_mask(&self) -> HARD_EVENT_3_ON_MASK_R {
        HARD_EVENT_3_ON_MASK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RGPIO1 event encourage turn-on sequence, 1: mask hardware event, 0: enable hardware event"]
    #[inline(always)]
    pub fn hard_event_4_on_mask(&self) -> HARD_EVENT_4_ON_MASK_R {
        HARD_EVENT_4_ON_MASK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RGPIO2 event encourage turn-on sequence, 1: mask hardware event, 0: enable hardware event"]
    #[inline(always)]
    pub fn hard_event_5_on_mask(&self) -> HARD_EVENT_5_ON_MASK_R {
        HARD_EVENT_5_ON_MASK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RGPIO3 event encourage turn-on sequence, 1: mask hardware event, 0: enable hardware event"]
    #[inline(always)]
    pub fn hard_event_6_on_mask(&self) -> HARD_EVENT_6_ON_MASK_R {
        HARD_EVENT_6_ON_MASK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPU event, 1: mask hardware event, 0: enable hardware event"]
    #[inline(always)]
    pub fn hard_event_7_on_mask(&self) -> HARD_EVENT_7_ON_MASK_R {
        HARD_EVENT_7_ON_MASK_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC event encourage turn-on sequence, 1: mask hardware event, 0: enable hardware event"]
    #[inline(always)]
    #[must_use]
    pub fn hard_event_0_on_mask(
        &mut self,
    ) -> HARD_EVENT_0_ON_MASK_W<HARD_EVENT_TURN_ON_MASK_SPEC, 0> {
        HARD_EVENT_0_ON_MASK_W::new(self)
    }
    #[doc = "Bit 1 - GMAC event encourage turn-on sequence, 1: mask hardware event, 0: enable hardware event"]
    #[inline(always)]
    #[must_use]
    pub fn hard_event_1_on_mask(
        &mut self,
    ) -> HARD_EVENT_1_ON_MASK_W<HARD_EVENT_TURN_ON_MASK_SPEC, 1> {
        HARD_EVENT_1_ON_MASK_W::new(self)
    }
    #[doc = "Bit 2 - RFU, 1: mask hardware event, 0: enable hardware event"]
    #[inline(always)]
    #[must_use]
    pub fn hard_event_2_on_mask(
        &mut self,
    ) -> HARD_EVENT_2_ON_MASK_W<HARD_EVENT_TURN_ON_MASK_SPEC, 2> {
        HARD_EVENT_2_ON_MASK_W::new(self)
    }
    #[doc = "Bit 3 - RGPIO0 event encourage turn-on sequence, 1: mask hardware event, 0: enable hardware event"]
    #[inline(always)]
    #[must_use]
    pub fn hard_event_3_on_mask(
        &mut self,
    ) -> HARD_EVENT_3_ON_MASK_W<HARD_EVENT_TURN_ON_MASK_SPEC, 3> {
        HARD_EVENT_3_ON_MASK_W::new(self)
    }
    #[doc = "Bit 4 - RGPIO1 event encourage turn-on sequence, 1: mask hardware event, 0: enable hardware event"]
    #[inline(always)]
    #[must_use]
    pub fn hard_event_4_on_mask(
        &mut self,
    ) -> HARD_EVENT_4_ON_MASK_W<HARD_EVENT_TURN_ON_MASK_SPEC, 4> {
        HARD_EVENT_4_ON_MASK_W::new(self)
    }
    #[doc = "Bit 5 - RGPIO2 event encourage turn-on sequence, 1: mask hardware event, 0: enable hardware event"]
    #[inline(always)]
    #[must_use]
    pub fn hard_event_5_on_mask(
        &mut self,
    ) -> HARD_EVENT_5_ON_MASK_W<HARD_EVENT_TURN_ON_MASK_SPEC, 5> {
        HARD_EVENT_5_ON_MASK_W::new(self)
    }
    #[doc = "Bit 6 - RGPIO3 event encourage turn-on sequence, 1: mask hardware event, 0: enable hardware event"]
    #[inline(always)]
    #[must_use]
    pub fn hard_event_6_on_mask(
        &mut self,
    ) -> HARD_EVENT_6_ON_MASK_W<HARD_EVENT_TURN_ON_MASK_SPEC, 6> {
        HARD_EVENT_6_ON_MASK_W::new(self)
    }
    #[doc = "Bit 7 - GPU event, 1: mask hardware event, 0: enable hardware event"]
    #[inline(always)]
    #[must_use]
    pub fn hard_event_7_on_mask(
        &mut self,
    ) -> HARD_EVENT_7_ON_MASK_W<HARD_EVENT_TURN_ON_MASK_SPEC, 7> {
        HARD_EVENT_7_ON_MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Hardware Event Turn-On Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hard_event_turn_on_mask::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hard_event_turn_on_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HARD_EVENT_TURN_ON_MASK_SPEC;
impl crate::RegisterSpec for HARD_EVENT_TURN_ON_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hard_event_turn_on_mask::R`](R) reader structure"]
impl crate::Readable for HARD_EVENT_TURN_ON_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hard_event_turn_on_mask::W`](W) writer structure"]
impl crate::Writable for HARD_EVENT_TURN_ON_MASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
