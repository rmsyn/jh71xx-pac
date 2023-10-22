#[doc = "Register `sda_hold` reader"]
pub type R = crate::R<SDA_HOLD_SPEC>;
#[doc = "Register `sda_hold` writer"]
pub type W = crate::W<SDA_HOLD_SPEC>;
#[doc = "Field `sda_hold` reader - sda_hold"]
pub type SDA_HOLD_R = crate::FieldReader<u32>;
#[doc = "Field `sda_hold` writer - sda_hold"]
pub type SDA_HOLD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - sda_hold"]
    #[inline(always)]
    pub fn sda_hold(&self) -> SDA_HOLD_R {
        SDA_HOLD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sda_hold"]
    #[inline(always)]
    #[must_use]
    pub fn sda_hold(&mut self) -> SDA_HOLD_W<SDA_HOLD_SPEC, 0> {
        SDA_HOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DesignWare I2C SDA Hold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sda_hold::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sda_hold::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDA_HOLD_SPEC;
impl crate::RegisterSpec for SDA_HOLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sda_hold::R`](R) reader structure"]
impl crate::Readable for SDA_HOLD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sda_hold::W`](W) writer structure"]
impl crate::Writable for SDA_HOLD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sda_hold to value 0"]
impl crate::Resettable for SDA_HOLD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
