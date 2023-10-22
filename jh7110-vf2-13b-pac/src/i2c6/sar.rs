#[doc = "Register `sar` reader"]
pub type R = crate::R<SAR_SPEC>;
#[doc = "Register `sar` writer"]
pub type W = crate::W<SAR_SPEC>;
#[doc = "Field `address_7bit` reader - Slave address, 7-bit mode"]
pub type ADDRESS_7BIT_R = crate::FieldReader;
#[doc = "Field `address_7bit` writer - Slave address, 7-bit mode"]
pub type ADDRESS_7BIT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `address_10bit` reader - Slave address, 10-bit mode"]
pub type ADDRESS_10BIT_R = crate::FieldReader<u16>;
#[doc = "Field `address_10bit` writer - Slave address, 10-bit mode"]
pub type ADDRESS_10BIT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
impl R {
    #[doc = "Bits 0:6 - Slave address, 7-bit mode"]
    #[inline(always)]
    pub fn address_7bit(&self) -> ADDRESS_7BIT_R {
        ADDRESS_7BIT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 0:9 - Slave address, 10-bit mode"]
    #[inline(always)]
    pub fn address_10bit(&self) -> ADDRESS_10BIT_R {
        ADDRESS_10BIT_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:6 - Slave address, 7-bit mode"]
    #[inline(always)]
    #[must_use]
    pub fn address_7bit(&mut self) -> ADDRESS_7BIT_W<SAR_SPEC, 0> {
        ADDRESS_7BIT_W::new(self)
    }
    #[doc = "Bits 0:9 - Slave address, 10-bit mode"]
    #[inline(always)]
    #[must_use]
    pub fn address_10bit(&mut self) -> ADDRESS_10BIT_W<SAR_SPEC, 0> {
        ADDRESS_10BIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DesignWare I2C SAR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_SPEC;
impl crate::RegisterSpec for SAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar::R`](R) reader structure"]
impl crate::Readable for SAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar::W`](W) writer structure"]
impl crate::Writable for SAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sar to value 0"]
impl crate::Resettable for SAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
