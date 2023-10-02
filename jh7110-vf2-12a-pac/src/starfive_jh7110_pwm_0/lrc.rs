#[doc = "Register `lrc` reader"]
pub type R = crate::R<LRC_SPEC>;
#[doc = "Register `lrc` writer"]
pub type W = crate::W<LRC_SPEC>;
#[doc = "Field `lrc` reader - PWM PTC period value"]
pub type LRC_R = crate::FieldReader<u32>;
#[doc = "Field `lrc` writer - PWM PTC period value"]
pub type LRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - PWM PTC period value"]
    #[inline(always)]
    pub fn lrc(&self) -> LRC_R {
        LRC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PWM PTC period value"]
    #[inline(always)]
    #[must_use]
    pub fn lrc(&mut self) -> LRC_W<LRC_SPEC, 0> {
        LRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PTC period register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lrc::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lrc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LRC_SPEC;
impl crate::RegisterSpec for LRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lrc::R`](R) reader structure"]
impl crate::Readable for LRC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lrc::W`](W) writer structure"]
impl crate::Writable for LRC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
