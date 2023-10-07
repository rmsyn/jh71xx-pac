#[doc = "Register `enable` reader"]
pub type R = crate::R<ENABLE_SPEC>;
#[doc = "Register `enable` writer"]
pub type W = crate::W<ENABLE_SPEC>;
#[doc = "Field `abort` reader - abort"]
pub type ABORT_R = crate::BitReader;
#[doc = "Field `abort` writer - abort"]
pub type ABORT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - abort"]
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - abort"]
    #[inline(always)]
    #[must_use]
    pub fn abort(&mut self) -> ABORT_W<ENABLE_SPEC, 1> {
        ABORT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DesignWare I2C Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENABLE_SPEC;
impl crate::RegisterSpec for ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enable::R`](R) reader structure"]
impl crate::Readable for ENABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`enable::W`](W) writer structure"]
impl crate::Writable for ENABLE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
