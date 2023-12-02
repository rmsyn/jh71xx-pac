#[doc = "Register `ie` reader"]
pub type R = crate::R<IE_SPEC>;
#[doc = "Register `ie` writer"]
pub type W = crate::W<IE_SPEC>;
#[doc = "Field `rand_rdy_en` reader - RAND Ready Enable"]
pub type RAND_RDY_EN_R = crate::BitReader;
#[doc = "Field `rand_rdy_en` writer - RAND Ready Enable"]
pub type RAND_RDY_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `seed_done_en` reader - Seed Done Enable"]
pub type SEED_DONE_EN_R = crate::BitReader;
#[doc = "Field `seed_done_en` writer - Seed Done Enable"]
pub type SEED_DONE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lfsr_lockup_en` reader - LFSR Lockup Enable"]
pub type LFSR_LOCKUP_EN_R = crate::BitReader;
#[doc = "Field `lfsr_lockup_en` writer - LFSR Lockup Enable"]
pub type LFSR_LOCKUP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `glbl_en` reader - Global Enable"]
pub type GLBL_EN_R = crate::BitReader;
#[doc = "Field `glbl_en` writer - Global Enable"]
pub type GLBL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RAND Ready Enable"]
    #[inline(always)]
    pub fn rand_rdy_en(&self) -> RAND_RDY_EN_R {
        RAND_RDY_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Seed Done Enable"]
    #[inline(always)]
    pub fn seed_done_en(&self) -> SEED_DONE_EN_R {
        SEED_DONE_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - LFSR Lockup Enable"]
    #[inline(always)]
    pub fn lfsr_lockup_en(&self) -> LFSR_LOCKUP_EN_R {
        LFSR_LOCKUP_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 31 - Global Enable"]
    #[inline(always)]
    pub fn glbl_en(&self) -> GLBL_EN_R {
        GLBL_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RAND Ready Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rand_rdy_en(&mut self) -> RAND_RDY_EN_W<IE_SPEC> {
        RAND_RDY_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Seed Done Enable"]
    #[inline(always)]
    #[must_use]
    pub fn seed_done_en(&mut self) -> SEED_DONE_EN_W<IE_SPEC> {
        SEED_DONE_EN_W::new(self, 1)
    }
    #[doc = "Bit 4 - LFSR Lockup Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lfsr_lockup_en(&mut self) -> LFSR_LOCKUP_EN_W<IE_SPEC> {
        LFSR_LOCKUP_EN_W::new(self, 4)
    }
    #[doc = "Bit 31 - Global Enable"]
    #[inline(always)]
    #[must_use]
    pub fn glbl_en(&mut self) -> GLBL_EN_W<IE_SPEC> {
        GLBL_EN_W::new(self, 31)
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
#[doc = "TRNG Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ie::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ie::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IE_SPEC;
impl crate::RegisterSpec for IE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ie::R`](R) reader structure"]
impl crate::Readable for IE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ie::W`](W) writer structure"]
impl crate::Writable for IE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
