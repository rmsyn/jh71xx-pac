#[doc = "Register `polling_status` reader"]
pub type R = crate::R<POLLING_STATUS_SPEC>;
#[doc = "Register `polling_status` writer"]
pub type W = crate::W<POLLING_STATUS_SPEC>;
#[doc = "Field `status` reader - status"]
pub type STATUS_R = crate::FieldReader<u16>;
#[doc = "Field `status` writer - status"]
pub type STATUS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `dummy` reader - dummy"]
pub type DUMMY_R = crate::FieldReader;
#[doc = "Field `dummy` writer - dummy"]
pub type DUMMY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:15 - status"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:20 - dummy"]
    #[inline(always)]
    pub fn dummy(&self) -> DUMMY_R {
        DUMMY_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - status"]
    #[inline(always)]
    #[must_use]
    pub fn status(&mut self) -> STATUS_W<POLLING_STATUS_SPEC, 0> {
        STATUS_W::new(self)
    }
    #[doc = "Bits 16:20 - dummy"]
    #[inline(always)]
    #[must_use]
    pub fn dummy(&mut self) -> DUMMY_W<POLLING_STATUS_SPEC, 16> {
        DUMMY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Cadence QSPI Polling Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`polling_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`polling_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POLLING_STATUS_SPEC;
impl crate::RegisterSpec for POLLING_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`polling_status::R`](R) reader structure"]
impl crate::Readable for POLLING_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`polling_status::W`](W) writer structure"]
impl crate::Writable for POLLING_STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets polling_status to value 0"]
impl crate::Resettable for POLLING_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
