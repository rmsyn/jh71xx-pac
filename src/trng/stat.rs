#[doc = "Register `stat` reader"]
pub type R = crate::R<STAT_SPEC>;
#[doc = "Register `stat` writer"]
pub type W = crate::W<STAT_SPEC>;
#[doc = "Field `nonce_mode` reader - TRNG Nonce operating mode"]
pub type NONCE_MODE_R = crate::BitReader;
#[doc = "Field `r256` reader - TRNG 256-bit random number operating mode"]
pub type R256_R = crate::BitReader;
#[doc = "Field `mission_mode` reader - TRNG Mission Mode operating mode"]
pub type MISSION_MODE_R = crate::BitReader;
#[doc = "Field `seeded` reader - TRNG Seeded operating mode"]
pub type SEEDED_R = crate::BitReader;
#[doc = "Field `last_reseed_0` reader - TRNG Last Reseed 0 status"]
pub type LAST_RESEED_0_R = crate::BitReader;
#[doc = "Field `last_reseed_1` reader - TRNG Last Reseed 1 status"]
pub type LAST_RESEED_1_R = crate::BitReader;
#[doc = "Field `last_reseed_2` reader - TRNG Last Reseed 2 status"]
pub type LAST_RESEED_2_R = crate::BitReader;
#[doc = "Field `last_reseed_3` reader - TRNG Last Reseed 3 status"]
pub type LAST_RESEED_3_R = crate::BitReader;
#[doc = "Field `last_reseed_4` reader - TRNG Last Reseed 4 status"]
pub type LAST_RESEED_4_R = crate::BitReader;
#[doc = "Field `last_reseed_5` reader - TRNG Last Reseed 5 status"]
pub type LAST_RESEED_5_R = crate::BitReader;
#[doc = "Field `last_reseed_6` reader - TRNG Last Reseed 6 status"]
pub type LAST_RESEED_6_R = crate::BitReader;
#[doc = "Field `last_reseed_7` reader - TRNG Last Reseed 7 status"]
pub type LAST_RESEED_7_R = crate::BitReader;
#[doc = "Field `srvc_rqst` reader - TRNG Service Request"]
pub type SRVC_RQST_R = crate::BitReader;
#[doc = "Field `rand_generating` reader - TRNG Random Number Generating Status"]
pub type RAND_GENERATING_R = crate::BitReader;
#[doc = "Field `rand_seeding` reader - TRNG Random Number Seeding Status"]
pub type RAND_SEEDING_R = crate::BitReader;
impl R {
    #[doc = "Bit 2 - TRNG Nonce operating mode"]
    #[inline(always)]
    pub fn nonce_mode(&self) -> NONCE_MODE_R {
        NONCE_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TRNG 256-bit random number operating mode"]
    #[inline(always)]
    pub fn r256(&self) -> R256_R {
        R256_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - TRNG Mission Mode operating mode"]
    #[inline(always)]
    pub fn mission_mode(&self) -> MISSION_MODE_R {
        MISSION_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TRNG Seeded operating mode"]
    #[inline(always)]
    pub fn seeded(&self) -> SEEDED_R {
        SEEDED_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - TRNG Last Reseed 0 status"]
    #[inline(always)]
    pub fn last_reseed_0(&self) -> LAST_RESEED_0_R {
        LAST_RESEED_0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TRNG Last Reseed 1 status"]
    #[inline(always)]
    pub fn last_reseed_1(&self) -> LAST_RESEED_1_R {
        LAST_RESEED_1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TRNG Last Reseed 2 status"]
    #[inline(always)]
    pub fn last_reseed_2(&self) -> LAST_RESEED_2_R {
        LAST_RESEED_2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - TRNG Last Reseed 3 status"]
    #[inline(always)]
    pub fn last_reseed_3(&self) -> LAST_RESEED_3_R {
        LAST_RESEED_3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TRNG Last Reseed 4 status"]
    #[inline(always)]
    pub fn last_reseed_4(&self) -> LAST_RESEED_4_R {
        LAST_RESEED_4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TRNG Last Reseed 5 status"]
    #[inline(always)]
    pub fn last_reseed_5(&self) -> LAST_RESEED_5_R {
        LAST_RESEED_5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - TRNG Last Reseed 6 status"]
    #[inline(always)]
    pub fn last_reseed_6(&self) -> LAST_RESEED_6_R {
        LAST_RESEED_6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TRNG Last Reseed 7 status"]
    #[inline(always)]
    pub fn last_reseed_7(&self) -> LAST_RESEED_7_R {
        LAST_RESEED_7_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 27 - TRNG Service Request"]
    #[inline(always)]
    pub fn srvc_rqst(&self) -> SRVC_RQST_R {
        SRVC_RQST_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 30 - TRNG Random Number Generating Status"]
    #[inline(always)]
    pub fn rand_generating(&self) -> RAND_GENERATING_R {
        RAND_GENERATING_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - TRNG Random Number Seeding Status"]
    #[inline(always)]
    pub fn rand_seeding(&self) -> RAND_SEEDING_R {
        RAND_SEEDING_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
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
#[doc = "TRNG STAT Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for STAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets stat to value 0"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
