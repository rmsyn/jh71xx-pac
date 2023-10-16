#[doc = "Register `tar` reader"]
pub type R = crate::R<TAR_SPEC>;
#[doc = "Register `tar` writer"]
pub type W = crate::W<TAR_SPEC>;
#[doc = "Field `address_7bit` reader - Target address, 7-bit mode"]
pub type ADDRESS_7BIT_R = crate::FieldReader;
#[doc = "Field `address_7bit` writer - Target address, 7-bit mode"]
pub type ADDRESS_7BIT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `address_10bit` reader - Target address, 10-bit mode"]
pub type ADDRESS_10BIT_R = crate::FieldReader<u16>;
#[doc = "Field `address_10bit` writer - Target address, 10-bit mode"]
pub type ADDRESS_10BIT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
#[doc = "Field `mode` reader - Target addressing mode - 0: 7-bit, 1: 10-bit"]
pub type MODE_R = crate::BitReader;
#[doc = "Field `mode` writer - Target addressing mode - 0: 7-bit, 1: 10-bit"]
pub type MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:6 - Target address, 7-bit mode"]
    #[inline(always)]
    pub fn address_7bit(&self) -> ADDRESS_7BIT_R {
        ADDRESS_7BIT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 0:9 - Target address, 10-bit mode"]
    #[inline(always)]
    pub fn address_10bit(&self) -> ADDRESS_10BIT_R {
        ADDRESS_10BIT_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 12 - Target addressing mode - 0: 7-bit, 1: 10-bit"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Target address, 7-bit mode"]
    #[inline(always)]
    #[must_use]
    pub fn address_7bit(&mut self) -> ADDRESS_7BIT_W<TAR_SPEC, 0> {
        ADDRESS_7BIT_W::new(self)
    }
    #[doc = "Bits 0:9 - Target address, 10-bit mode"]
    #[inline(always)]
    #[must_use]
    pub fn address_10bit(&mut self) -> ADDRESS_10BIT_W<TAR_SPEC, 0> {
        ADDRESS_10BIT_W::new(self)
    }
    #[doc = "Bit 12 - Target addressing mode - 0: 7-bit, 1: 10-bit"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<TAR_SPEC, 12> {
        MODE_W::new(self)
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
