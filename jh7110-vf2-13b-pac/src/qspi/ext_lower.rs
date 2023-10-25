#[doc = "Register `ext_lower` reader"]
pub type R = crate::R<EXT_LOWER_SPEC>;
#[doc = "Register `ext_lower` writer"]
pub type W = crate::W<EXT_LOWER_SPEC>;
#[doc = "Field `stig` reader - stig"]
pub type STIG_R = crate::FieldReader<u16>;
#[doc = "Field `stig` writer - stig"]
pub type STIG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `write` reader - write"]
pub type WRITE_R = crate::FieldReader;
#[doc = "Field `write` writer - write"]
pub type WRITE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `read` reader - read"]
pub type READ_R = crate::FieldReader;
#[doc = "Field `read` writer - read"]
pub type READ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:15 - stig"]
    #[inline(always)]
    pub fn stig(&self) -> STIG_R {
        STIG_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - write"]
    #[inline(always)]
    pub fn write(&self) -> WRITE_R {
        WRITE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - read"]
    #[inline(always)]
    pub fn read(&self) -> READ_R {
        READ_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - stig"]
    #[inline(always)]
    #[must_use]
    pub fn stig(&mut self) -> STIG_W<EXT_LOWER_SPEC, 0> {
        STIG_W::new(self)
    }
    #[doc = "Bits 16:23 - write"]
    #[inline(always)]
    #[must_use]
    pub fn write(&mut self) -> WRITE_W<EXT_LOWER_SPEC, 16> {
        WRITE_W::new(self)
    }
    #[doc = "Bits 24:31 - read"]
    #[inline(always)]
    #[must_use]
    pub fn read(&mut self) -> READ_W<EXT_LOWER_SPEC, 24> {
        READ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Cadence QSPI Extension Lower\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_lower::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_lower::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXT_LOWER_SPEC;
impl crate::RegisterSpec for EXT_LOWER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_lower::R`](R) reader structure"]
impl crate::Readable for EXT_LOWER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ext_lower::W`](W) writer structure"]
impl crate::Writable for EXT_LOWER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ext_lower to value 0"]
impl crate::Resettable for EXT_LOWER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
