#[doc = "Register `sw_encourage` reader"]
pub type R = crate::R<SW_ENCOURAGE_SPEC>;
#[doc = "Register `sw_encourage` writer"]
pub type W = crate::W<SW_ENCOURAGE_SPEC>;
#[doc = "Field `sw_encourage` reader - Software Encouragement"]
pub type SW_ENCOURAGE_R = crate::FieldReader;
#[doc = "Field `sw_encourage` writer - Software Encouragement"]
pub type SW_ENCOURAGE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Software Encouragement"]
    #[inline(always)]
    pub fn sw_encourage(&self) -> SW_ENCOURAGE_R {
        SW_ENCOURAGE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Software Encouragement"]
    #[inline(always)]
    #[must_use]
    pub fn sw_encourage(&mut self) -> SW_ENCOURAGE_W<SW_ENCOURAGE_SPEC, 0> {
        SW_ENCOURAGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Software Encouragement\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sw_encourage::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_encourage::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SW_ENCOURAGE_SPEC;
impl crate::RegisterSpec for SW_ENCOURAGE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sw_encourage::R`](R) reader structure"]
impl crate::Readable for SW_ENCOURAGE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sw_encourage::W`](W) writer structure"]
impl crate::Writable for SW_ENCOURAGE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
