#[doc = "Register `size` reader"]
pub type R = crate::R<SIZE_SPEC>;
#[doc = "Register `size` writer"]
pub type W = crate::W<SIZE_SPEC>;
#[doc = "Field `address` reader - Address Size in Bytes"]
pub type ADDRESS_R = crate::FieldReader;
#[doc = "Field `address` writer - Address Size in Bytes"]
pub type ADDRESS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `page` reader - Page Size in Bytes"]
pub type PAGE_R = crate::FieldReader<u16>;
#[doc = "Field `page` writer - Page Size in Bytes"]
pub type PAGE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `block` reader - Block Size in Bytes"]
pub type BLOCK_R = crate::FieldReader;
#[doc = "Field `block` writer - Block Size in Bytes"]
pub type BLOCK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:3 - Address Size in Bytes"]
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - Page Size in Bytes"]
    #[inline(always)]
    pub fn page(&self) -> PAGE_R {
        PAGE_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:21 - Block Size in Bytes"]
    #[inline(always)]
    pub fn block(&self) -> BLOCK_R {
        BLOCK_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Address Size in Bytes"]
    #[inline(always)]
    #[must_use]
    pub fn address(&mut self) -> ADDRESS_W<SIZE_SPEC, 0> {
        ADDRESS_W::new(self)
    }
    #[doc = "Bits 4:15 - Page Size in Bytes"]
    #[inline(always)]
    #[must_use]
    pub fn page(&mut self) -> PAGE_W<SIZE_SPEC, 4> {
        PAGE_W::new(self)
    }
    #[doc = "Bits 16:21 - Block Size in Bytes"]
    #[inline(always)]
    #[must_use]
    pub fn block(&mut self) -> BLOCK_W<SIZE_SPEC, 16> {
        BLOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Cadence QSPI Size Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SIZE_SPEC;
impl crate::RegisterSpec for SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`size::R`](R) reader structure"]
impl crate::Readable for SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`size::W`](W) writer structure"]
impl crate::Writable for SIZE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets size to value 0"]
impl crate::Resettable for SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
