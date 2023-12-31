#[doc = "Register `hrc` reader"]
pub type R = crate::R<HRC_SPEC>;
#[doc = "Register `hrc` writer"]
pub type W = crate::W<HRC_SPEC>;
#[doc = "Field `hrc` reader - PWM PTC duty-cycle value"]
pub type HRC_R = crate::FieldReader<u32>;
#[doc = "Field `hrc` writer - PWM PTC duty-cycle value"]
pub type HRC_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PWM PTC duty-cycle value"]
    #[inline(always)]
    pub fn hrc(&self) -> HRC_R {
        HRC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PWM PTC duty-cycle value"]
    #[inline(always)]
    #[must_use]
    pub fn hrc(&mut self) -> HRC_W<HRC_SPEC> {
        HRC_W::new(self, 0)
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
#[doc = "PTC duty-cycle register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hrc::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hrc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HRC_SPEC;
impl crate::RegisterSpec for HRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hrc::R`](R) reader structure"]
impl crate::Readable for HRC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hrc::W`](W) writer structure"]
impl crate::Writable for HRC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
