#[doc = "Register `auto_rqsts` reader"]
pub type R = crate::R<AUTO_RQSTS_SPEC>;
#[doc = "Register `auto_rqsts` writer"]
pub type W = crate::W<AUTO_RQSTS_SPEC>;
#[doc = "Field `rqsts` reader - Threshold number of reseed requests for auto-reseed counter"]
pub type RQSTS_R = crate::FieldReader<u32>;
#[doc = "Field `rqsts` writer - Threshold number of reseed requests for auto-reseed counter"]
pub type RQSTS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Threshold number of reseed requests for auto-reseed counter"]
    #[inline(always)]
    pub fn rqsts(&self) -> RQSTS_R {
        RQSTS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Threshold number of reseed requests for auto-reseed counter"]
    #[inline(always)]
    #[must_use]
    pub fn rqsts(&mut self) -> RQSTS_W<AUTO_RQSTS_SPEC, 0> {
        RQSTS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Auto-reseeding after random number requests by host reaches specified counter: 0 - disable counter, other - reload value for internal counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`auto_rqsts::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`auto_rqsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AUTO_RQSTS_SPEC;
impl crate::RegisterSpec for AUTO_RQSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`auto_rqsts::R`](R) reader structure"]
impl crate::Readable for AUTO_RQSTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`auto_rqsts::W`](W) writer structure"]
impl crate::Writable for AUTO_RQSTS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
