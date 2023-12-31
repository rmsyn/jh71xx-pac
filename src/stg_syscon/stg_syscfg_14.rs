#[doc = "Register `stg_syscfg_14` reader"]
pub type R = crate::R<STG_SYSCFG_14_SPEC>;
#[doc = "Register `stg_syscfg_14` writer"]
pub type W = crate::W<STG_SYSCFG_14_SPEC>;
#[doc = "Field `u0_hifi4_pfaultinfovalid` reader - Fault Handling Signals"]
pub type U0_HIFI4_PFAULTINFOVALID_R = crate::BitReader;
#[doc = "Field `u0_hifi4_prid` reader - Module ID"]
pub type U0_HIFI4_PRID_R = crate::FieldReader<u16>;
#[doc = "Field `u0_hifi4_prid` writer - Module ID"]
pub type U0_HIFI4_PRID_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `u0_hifi4_pwaitmode` reader - Wait Mode"]
pub type U0_HIFI4_PWAITMODE_R = crate::BitReader;
#[doc = "Field `u0_hifi4_runstall` reader - Run Stall"]
pub type U0_HIFI4_RUNSTALL_R = crate::BitReader;
#[doc = "Field `u0_hifi4_runstall` writer - Run Stall"]
pub type U0_HIFI4_RUNSTALL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Fault Handling Signals"]
    #[inline(always)]
    pub fn u0_hifi4_pfaultinfovalid(&self) -> U0_HIFI4_PFAULTINFOVALID_R {
        U0_HIFI4_PFAULTINFOVALID_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:16 - Module ID"]
    #[inline(always)]
    pub fn u0_hifi4_prid(&self) -> U0_HIFI4_PRID_R {
        U0_HIFI4_PRID_R::new(((self.bits >> 1) & 0xffff) as u16)
    }
    #[doc = "Bit 17 - Wait Mode"]
    #[inline(always)]
    pub fn u0_hifi4_pwaitmode(&self) -> U0_HIFI4_PWAITMODE_R {
        U0_HIFI4_PWAITMODE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Run Stall"]
    #[inline(always)]
    pub fn u0_hifi4_runstall(&self) -> U0_HIFI4_RUNSTALL_R {
        U0_HIFI4_RUNSTALL_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:16 - Module ID"]
    #[inline(always)]
    #[must_use]
    pub fn u0_hifi4_prid(&mut self) -> U0_HIFI4_PRID_W<STG_SYSCFG_14_SPEC> {
        U0_HIFI4_PRID_W::new(self, 1)
    }
    #[doc = "Bit 18 - Run Stall"]
    #[inline(always)]
    #[must_use]
    pub fn u0_hifi4_runstall(&mut self) -> U0_HIFI4_RUNSTALL_W<STG_SYSCFG_14_SPEC> {
        U0_HIFI4_RUNSTALL_W::new(self, 18)
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
#[doc = "STG SYSCONSAIF SYSCFG 56\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_14::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_14::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCFG_14_SPEC;
impl crate::RegisterSpec for STG_SYSCFG_14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_14::R`](R) reader structure"]
impl crate::Readable for STG_SYSCFG_14_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_14::W`](W) writer structure"]
impl crate::Writable for STG_SYSCFG_14_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets stg_syscfg_14 to value 0"]
impl crate::Resettable for STG_SYSCFG_14_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
