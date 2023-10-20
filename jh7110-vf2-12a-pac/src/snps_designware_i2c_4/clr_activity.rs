#[doc = "Register `clr_activity` reader"]
pub type R = crate::R<CLR_ACTIVITY_SPEC>;
#[doc = "Register `clr_activity` writer"]
pub type W = crate::W<CLR_ACTIVITY_SPEC>;
#[doc = "Field `clr_activity` reader - clr_activity"]
pub type CLR_ACTIVITY_R = crate::FieldReader<u32>;
#[doc = "Field `clr_activity` writer - clr_activity"]
pub type CLR_ACTIVITY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - clr_activity"]
    #[inline(always)]
    pub fn clr_activity(&self) -> CLR_ACTIVITY_R {
        CLR_ACTIVITY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - clr_activity"]
    #[inline(always)]
    #[must_use]
    pub fn clr_activity(&mut self) -> CLR_ACTIVITY_W<CLR_ACTIVITY_SPEC, 0> {
        CLR_ACTIVITY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DesignWare I2C Clear Activity\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr_activity::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr_activity::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLR_ACTIVITY_SPEC;
impl crate::RegisterSpec for CLR_ACTIVITY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr_activity::R`](R) reader structure"]
impl crate::Readable for CLR_ACTIVITY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clr_activity::W`](W) writer structure"]
impl crate::Writable for CLR_ACTIVITY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets clr_activity to value 0"]
impl crate::Resettable for CLR_ACTIVITY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
