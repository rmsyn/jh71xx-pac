#[doc = "Register `comp_type` reader"]
pub type R = crate::R<COMP_TYPE_SPEC>;
#[doc = "Register `comp_type` writer"]
pub type W = crate::W<COMP_TYPE_SPEC>;
#[doc = "Field `comp_type` reader - comp_type"]
pub type COMP_TYPE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - comp_type"]
    #[inline(always)]
    pub fn comp_type(&self) -> COMP_TYPE_R {
        COMP_TYPE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DesignWare I2C Compatibility Type\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp_type::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp_type::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMP_TYPE_SPEC;
impl crate::RegisterSpec for COMP_TYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp_type::R`](R) reader structure"]
impl crate::Readable for COMP_TYPE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`comp_type::W`](W) writer structure"]
impl crate::Writable for COMP_TYPE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets comp_type to value 0"]
impl crate::Resettable for COMP_TYPE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
