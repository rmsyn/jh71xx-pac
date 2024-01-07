#[doc = "Register `current_seq_state` reader"]
pub type R = crate::R<CURRENT_SEQ_STATE_SPEC>;
#[doc = "Register `current_seq_state` writer"]
pub type W = crate::W<CURRENT_SEQ_STATE_SPEC>;
#[doc = "Field `power_mode_cur` reader - Current sequence state."]
pub type POWER_MODE_CUR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Current sequence state."]
    #[inline(always)]
    pub fn power_mode_cur(&self) -> POWER_MODE_CUR_R {
        POWER_MODE_CUR_R::new((self.bits & 3) as u8)
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
#[doc = "Current Sequence State\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`current_seq_state::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`current_seq_state::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CURRENT_SEQ_STATE_SPEC;
impl crate::RegisterSpec for CURRENT_SEQ_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`current_seq_state::R`](R) reader structure"]
impl crate::Readable for CURRENT_SEQ_STATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`current_seq_state::W`](W) writer structure"]
impl crate::Writable for CURRENT_SEQ_STATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets current_seq_state to value 0"]
impl crate::Resettable for CURRENT_SEQ_STATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
