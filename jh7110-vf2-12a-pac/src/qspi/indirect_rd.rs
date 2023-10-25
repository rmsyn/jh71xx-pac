#[doc = "Register `indirect_rd` reader"]
pub type R = crate::R<INDIRECT_RD_SPEC>;
#[doc = "Register `indirect_rd` writer"]
pub type W = crate::W<INDIRECT_RD_SPEC>;
#[doc = "Field `start` reader - Start indirect read"]
pub type START_R = crate::BitReader;
#[doc = "Field `start` writer - Start indirect read"]
pub type START_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `cancel` reader - Cancel indirect read"]
pub type CANCEL_R = crate::BitReader;
#[doc = "Field `cancel` writer - Cancel indirect read"]
pub type CANCEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `done` reader - Indirect read done"]
pub type DONE_R = crate::BitReader;
#[doc = "Field `done` writer - Indirect read done"]
pub type DONE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Start indirect read"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Cancel indirect read"]
    #[inline(always)]
    pub fn cancel(&self) -> CANCEL_R {
        CANCEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Indirect read done"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start indirect read"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<INDIRECT_RD_SPEC, 0> {
        START_W::new(self)
    }
    #[doc = "Bit 1 - Cancel indirect read"]
    #[inline(always)]
    #[must_use]
    pub fn cancel(&mut self) -> CANCEL_W<INDIRECT_RD_SPEC, 1> {
        CANCEL_W::new(self)
    }
    #[doc = "Bit 5 - Indirect read done"]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DONE_W<INDIRECT_RD_SPEC, 5> {
        DONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Cadence QSPI Indirect Read\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`indirect_rd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`indirect_rd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INDIRECT_RD_SPEC;
impl crate::RegisterSpec for INDIRECT_RD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`indirect_rd::R`](R) reader structure"]
impl crate::Readable for INDIRECT_RD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`indirect_rd::W`](W) writer structure"]
impl crate::Writable for INDIRECT_RD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets indirect_rd to value 0"]
impl crate::Resettable for INDIRECT_RD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
