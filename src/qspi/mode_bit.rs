#[doc = "Register `mode_bit` reader"]
pub type R = crate::R<MODE_BIT_SPEC>;
#[doc = "Register `mode_bit` writer"]
pub type W = crate::W<MODE_BIT_SPEC>;
#[doc = "Field `mode` reader - mode"]
pub type MODE_R = crate::FieldReader<u32>;
#[doc = "Field `mode` writer - mode"]
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<MODE_BIT_SPEC> {
        MODE_W::new(self, 0)
    }
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
#[doc = "Cadence QSPI Mode Bit(s)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode_bit::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode_bit::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODE_BIT_SPEC;
impl crate::RegisterSpec for MODE_BIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mode_bit::R`](R) reader structure"]
impl crate::Readable for MODE_BIT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mode_bit::W`](W) writer structure"]
impl crate::Writable for MODE_BIT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mode_bit to value 0"]
impl crate::Resettable for MODE_BIT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
