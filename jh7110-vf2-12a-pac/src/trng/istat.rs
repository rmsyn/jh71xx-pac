#[doc = "Register `istat` reader"]
pub type R = crate::R<ISTAT_SPEC>;
#[doc = "Register `istat` writer"]
pub type W = crate::W<ISTAT_SPEC>;
#[doc = "Field `rand_rdy` reader - RAND Ready Enable"]
pub type RAND_RDY_R = crate::BitReader;
#[doc = "Field `seed_done` reader - Seed Done Enable"]
pub type SEED_DONE_R = crate::BitReader;
#[doc = "Field `lfsr_lockup_en` reader - LFSR Lockup Enable"]
pub type LFSR_LOCKUP_EN_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RAND Ready Enable"]
    #[inline(always)]
    pub fn rand_rdy(&self) -> RAND_RDY_R {
        RAND_RDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Seed Done Enable"]
    #[inline(always)]
    pub fn seed_done(&self) -> SEED_DONE_R {
        SEED_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - LFSR Lockup Enable"]
    #[inline(always)]
    pub fn lfsr_lockup_en(&self) -> LFSR_LOCKUP_EN_R {
        LFSR_LOCKUP_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "TRNG Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`istat::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`istat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISTAT_SPEC;
impl crate::RegisterSpec for ISTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`istat::R`](R) reader structure"]
impl crate::Readable for ISTAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`istat::W`](W) writer structure"]
impl crate::Writable for ISTAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
