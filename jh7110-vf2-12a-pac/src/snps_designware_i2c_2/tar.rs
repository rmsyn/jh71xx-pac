#[doc = "Register `tar` reader"]
pub type R = crate::R<TAR_SPEC>;
#[doc = "Register `tar` writer"]
pub type W = crate::W<TAR_SPEC>;
#[doc = "Field `tar` reader - tar"]
pub type TAR_R = crate::FieldReader<u32>;
#[doc = "Field `tar` writer - tar"]
pub type TAR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - tar"]
    #[inline(always)]
    pub fn tar(&self) -> TAR_R {
        TAR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - tar"]
    #[inline(always)]
    #[must_use]
    pub fn tar(&mut self) -> TAR_W<TAR_SPEC, 0> {
        TAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DesignWare I2C TAR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tar::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TAR_SPEC;
impl crate::RegisterSpec for TAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tar::R`](R) reader structure"]
impl crate::Readable for TAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tar::W`](W) writer structure"]
impl crate::Writable for TAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
