#[doc = "Register `comp_version` reader"]
pub type R = crate::R<COMP_VERSION_SPEC>;
#[doc = "Register `comp_version` writer"]
pub type W = crate::W<COMP_VERSION_SPEC>;
#[doc = "Field `comp_version` reader - comp_version"]
pub type COMP_VERSION_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - comp_version"]
    #[inline(always)]
    pub fn comp_version(&self) -> COMP_VERSION_R {
        COMP_VERSION_R::new(self.bits)
    }
}
impl W {
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
#[doc = "DesignWare I2C Compatibility Version\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp_version::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp_version::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMP_VERSION_SPEC;
impl crate::RegisterSpec for COMP_VERSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp_version::R`](R) reader structure"]
impl crate::Readable for COMP_VERSION_SPEC {}
#[doc = "`write(|w| ..)` method takes [`comp_version::W`](W) writer structure"]
impl crate::Writable for COMP_VERSION_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets comp_version to value 0"]
impl crate::Resettable for COMP_VERSION_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
