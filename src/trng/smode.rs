#[doc = "Register `smode` reader"]
pub type R = crate::R<SMODE_SPEC>;
#[doc = "Register `smode` writer"]
pub type W = crate::W<SMODE_SPEC>;
#[doc = "Field `nonce_mode` reader - Nonce operation mode"]
pub type NONCE_MODE_R = crate::BitReader;
#[doc = "Field `nonce_mode` writer - Nonce operation mode"]
pub type NONCE_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mission_mode` reader - Mission operation mode"]
pub type MISSION_MODE_R = crate::BitReader;
#[doc = "Field `mission_mode` writer - Mission operation mode"]
pub type MISSION_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `max_rejects` reader - TRNG Maximum Rejects"]
pub type MAX_REJECTS_R = crate::FieldReader<u16>;
#[doc = "Field `max_rejects` writer - TRNG Maximum Rejects"]
pub type MAX_REJECTS_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 2 - Nonce operation mode"]
    #[inline(always)]
    pub fn nonce_mode(&self) -> NONCE_MODE_R {
        NONCE_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Mission operation mode"]
    #[inline(always)]
    pub fn mission_mode(&self) -> MISSION_MODE_R {
        MISSION_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:31 - TRNG Maximum Rejects"]
    #[inline(always)]
    pub fn max_rejects(&self) -> MAX_REJECTS_R {
        MAX_REJECTS_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 2 - Nonce operation mode"]
    #[inline(always)]
    #[must_use]
    pub fn nonce_mode(&mut self) -> NONCE_MODE_W<SMODE_SPEC> {
        NONCE_MODE_W::new(self, 2)
    }
    #[doc = "Bit 8 - Mission operation mode"]
    #[inline(always)]
    #[must_use]
    pub fn mission_mode(&mut self) -> MISSION_MODE_W<SMODE_SPEC> {
        MISSION_MODE_W::new(self, 8)
    }
    #[doc = "Bits 16:31 - TRNG Maximum Rejects"]
    #[inline(always)]
    #[must_use]
    pub fn max_rejects(&mut self) -> MAX_REJECTS_W<SMODE_SPEC> {
        MAX_REJECTS_W::new(self, 16)
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
#[doc = "TRNG SMODE Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMODE_SPEC;
impl crate::RegisterSpec for SMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smode::R`](R) reader structure"]
impl crate::Readable for SMODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smode::W`](W) writer structure"]
impl crate::Writable for SMODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets smode to value 0"]
impl crate::Resettable for SMODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
