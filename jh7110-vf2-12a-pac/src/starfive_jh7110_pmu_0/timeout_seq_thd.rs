#[doc = "Register `timeout_seq_thd` reader"]
pub type R = crate::R<TIMEOUT_SEQ_THD_SPEC>;
#[doc = "Register `timeout_seq_thd` writer"]
pub type W = crate::W<TIMEOUT_SEQ_THD_SPEC>;
#[doc = "Field `timeout_seq_thd` reader - Threshold Sequence Timeout"]
pub type TIMEOUT_SEQ_THD_R = crate::FieldReader<u16>;
#[doc = "Field `timeout_seq_thd` writer - Threshold Sequence Timeout"]
pub type TIMEOUT_SEQ_THD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Threshold Sequence Timeout"]
    #[inline(always)]
    pub fn timeout_seq_thd(&self) -> TIMEOUT_SEQ_THD_R {
        TIMEOUT_SEQ_THD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Threshold Sequence Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn timeout_seq_thd(&mut self) -> TIMEOUT_SEQ_THD_W<TIMEOUT_SEQ_THD_SPEC, 0> {
        TIMEOUT_SEQ_THD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Threshold Sequence Timeout\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timeout_seq_thd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timeout_seq_thd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMEOUT_SEQ_THD_SPEC;
impl crate::RegisterSpec for TIMEOUT_SEQ_THD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timeout_seq_thd::R`](R) reader structure"]
impl crate::Readable for TIMEOUT_SEQ_THD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timeout_seq_thd::W`](W) writer structure"]
impl crate::Writable for TIMEOUT_SEQ_THD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets timeout_seq_thd to value 0"]
impl crate::Resettable for TIMEOUT_SEQ_THD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
