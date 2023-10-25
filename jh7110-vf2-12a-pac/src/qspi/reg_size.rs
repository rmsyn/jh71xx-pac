#[doc = "Register `reg_size` reader"]
pub type R = crate::R<REG_SIZE_SPEC>;
#[doc = "Register `reg_size` writer"]
pub type W = crate::W<REG_SIZE_SPEC>;
#[doc = "Field `address` reader - Register Size Address"]
pub type ADDRESS_R = crate::FieldReader;
#[doc = "Field `address` writer - Register Size Address"]
pub type ADDRESS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `page` reader - Read Capture Delay Value"]
pub type PAGE_R = crate::FieldReader<u16>;
#[doc = "Field `page` writer - Read Capture Delay Value"]
pub type PAGE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `block` reader - Read Capture Block Number"]
pub type BLOCK_R = crate::FieldReader;
#[doc = "Field `block` writer - Read Capture Block Number"]
pub type BLOCK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:3 - Register Size Address"]
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - Read Capture Delay Value"]
    #[inline(always)]
    pub fn page(&self) -> PAGE_R {
        PAGE_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:21 - Read Capture Block Number"]
    #[inline(always)]
    pub fn block(&self) -> BLOCK_R {
        BLOCK_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Register Size Address"]
    #[inline(always)]
    #[must_use]
    pub fn address(&mut self) -> ADDRESS_W<REG_SIZE_SPEC, 0> {
        ADDRESS_W::new(self)
    }
    #[doc = "Bits 4:15 - Read Capture Delay Value"]
    #[inline(always)]
    #[must_use]
    pub fn page(&mut self) -> PAGE_W<REG_SIZE_SPEC, 4> {
        PAGE_W::new(self)
    }
    #[doc = "Bits 16:21 - Read Capture Block Number"]
    #[inline(always)]
    #[must_use]
    pub fn block(&mut self) -> BLOCK_W<REG_SIZE_SPEC, 16> {
        BLOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Cadence QSPI Register Size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REG_SIZE_SPEC;
impl crate::RegisterSpec for REG_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_size::R`](R) reader structure"]
impl crate::Readable for REG_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reg_size::W`](W) writer structure"]
impl crate::Writable for REG_SIZE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets reg_size to value 0"]
impl crate::Resettable for REG_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
