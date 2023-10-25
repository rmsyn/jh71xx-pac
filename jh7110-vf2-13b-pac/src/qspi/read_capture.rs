#[doc = "Register `read_capture` reader"]
pub type R = crate::R<READ_CAPTURE_SPEC>;
#[doc = "Register `read_capture` writer"]
pub type W = crate::W<READ_CAPTURE_SPEC>;
#[doc = "Field `bypass` reader - Bypass the Read Capture"]
pub type BYPASS_R = crate::BitReader;
#[doc = "Field `bypass` writer - Bypass the Read Capture"]
pub type BYPASS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `delay` reader - Read Capture Delay Value"]
pub type DELAY_R = crate::FieldReader;
#[doc = "Field `delay` writer - Read Capture Delay Value"]
pub type DELAY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bit 0 - Bypass the Read Capture"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - Read Capture Delay Value"]
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Bypass the Read Capture"]
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BYPASS_W<READ_CAPTURE_SPEC, 0> {
        BYPASS_W::new(self)
    }
    #[doc = "Bits 1:4 - Read Capture Delay Value"]
    #[inline(always)]
    #[must_use]
    pub fn delay(&mut self) -> DELAY_W<READ_CAPTURE_SPEC, 1> {
        DELAY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Cadence QSPI Read Capture\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`read_capture::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`read_capture::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct READ_CAPTURE_SPEC;
impl crate::RegisterSpec for READ_CAPTURE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`read_capture::R`](R) reader structure"]
impl crate::Readable for READ_CAPTURE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`read_capture::W`](W) writer structure"]
impl crate::Writable for READ_CAPTURE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets read_capture to value 0"]
impl crate::Resettable for READ_CAPTURE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
