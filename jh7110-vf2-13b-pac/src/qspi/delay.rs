#[doc = "Register `delay` reader"]
pub type R = crate::R<DELAY_SPEC>;
#[doc = "Register `delay` writer"]
pub type W = crate::W<DELAY_SPEC>;
#[doc = "Field `tslch` reader - TSLCH Delay Value"]
pub type TSLCH_R = crate::FieldReader;
#[doc = "Field `tslch` writer - TSLCH Delay Value"]
pub type TSLCH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `tchsh` reader - TCHSH Delay Value"]
pub type TCHSH_R = crate::FieldReader;
#[doc = "Field `tchsh` writer - TCHSH Delay Value"]
pub type TCHSH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `tsd2d` reader - TSD2D Delay Value"]
pub type TSD2D_R = crate::FieldReader;
#[doc = "Field `tsd2d` writer - TSD2D Delay Value"]
pub type TSD2D_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `tshsl` reader - TSHSL Delay Value"]
pub type TSHSL_R = crate::FieldReader;
#[doc = "Field `tshsl` writer - TSHSL Delay Value"]
pub type TSHSL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - TSLCH Delay Value"]
    #[inline(always)]
    pub fn tslch(&self) -> TSLCH_R {
        TSLCH_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - TCHSH Delay Value"]
    #[inline(always)]
    pub fn tchsh(&self) -> TCHSH_R {
        TCHSH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - TSD2D Delay Value"]
    #[inline(always)]
    pub fn tsd2d(&self) -> TSD2D_R {
        TSD2D_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - TSHSL Delay Value"]
    #[inline(always)]
    pub fn tshsl(&self) -> TSHSL_R {
        TSHSL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TSLCH Delay Value"]
    #[inline(always)]
    #[must_use]
    pub fn tslch(&mut self) -> TSLCH_W<DELAY_SPEC, 0> {
        TSLCH_W::new(self)
    }
    #[doc = "Bits 8:15 - TCHSH Delay Value"]
    #[inline(always)]
    #[must_use]
    pub fn tchsh(&mut self) -> TCHSH_W<DELAY_SPEC, 8> {
        TCHSH_W::new(self)
    }
    #[doc = "Bits 16:23 - TSD2D Delay Value"]
    #[inline(always)]
    #[must_use]
    pub fn tsd2d(&mut self) -> TSD2D_W<DELAY_SPEC, 16> {
        TSD2D_W::new(self)
    }
    #[doc = "Bits 24:31 - TSHSL Delay Value"]
    #[inline(always)]
    #[must_use]
    pub fn tshsl(&mut self) -> TSHSL_W<DELAY_SPEC, 24> {
        TSHSL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Cadence QSPI Delay\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`delay::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`delay::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DELAY_SPEC;
impl crate::RegisterSpec for DELAY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`delay::R`](R) reader structure"]
impl crate::Readable for DELAY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`delay::W`](W) writer structure"]
impl crate::Writable for DELAY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets delay to value 0"]
impl crate::Resettable for DELAY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
