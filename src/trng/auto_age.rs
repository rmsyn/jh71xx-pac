#[doc = "Register `auto_age` reader"]
pub type R = crate::R<AUTO_AGE_SPEC>;
#[doc = "Register `auto_age` writer"]
pub type W = crate::W<AUTO_AGE_SPEC>;
#[doc = "Field `age` reader - Countdown value for auto-reseed timer"]
pub type AGE_R = crate::FieldReader<u32>;
#[doc = "Field `age` writer - Countdown value for auto-reseed timer"]
pub type AGE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Countdown value for auto-reseed timer"]
    #[inline(always)]
    pub fn age(&self) -> AGE_R {
        AGE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Countdown value for auto-reseed timer"]
    #[inline(always)]
    #[must_use]
    pub fn age(&mut self) -> AGE_W<AUTO_AGE_SPEC> {
        AGE_W::new(self, 0)
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
#[doc = "Auto-reseeding after specified timer countdowns to 0: 0 - disable timer, other - reload value for internal timer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`auto_age::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`auto_age::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AUTO_AGE_SPEC;
impl crate::RegisterSpec for AUTO_AGE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`auto_age::R`](R) reader structure"]
impl crate::Readable for AUTO_AGE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`auto_age::W`](W) writer structure"]
impl crate::Writable for AUTO_AGE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets auto_age to value 0"]
impl crate::Resettable for AUTO_AGE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
