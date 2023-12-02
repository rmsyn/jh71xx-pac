#[doc = "Register `mode` reader"]
pub type R = crate::R<MODE_SPEC>;
#[doc = "Register `mode` writer"]
pub type W = crate::W<MODE_SPEC>;
#[doc = "Field `r256` reader - 256-bit operation mode: 0 - 128-bit mode, 1 - 256-bit mode"]
pub type R256_R = crate::BitReader;
#[doc = "Field `r256` writer - 256-bit operation mode: 0 - 128-bit mode, 1 - 256-bit mode"]
pub type R256_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - 256-bit operation mode: 0 - 128-bit mode, 1 - 256-bit mode"]
    #[inline(always)]
    pub fn r256(&self) -> R256_R {
        R256_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - 256-bit operation mode: 0 - 128-bit mode, 1 - 256-bit mode"]
    #[inline(always)]
    #[must_use]
    pub fn r256(&mut self) -> R256_W<MODE_SPEC> {
        R256_W::new(self, 3)
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
#[doc = "TRNG MODE Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODE_SPEC;
impl crate::RegisterSpec for MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mode::R`](R) reader structure"]
impl crate::Readable for MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mode::W`](W) writer structure"]
impl crate::Writable for MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
