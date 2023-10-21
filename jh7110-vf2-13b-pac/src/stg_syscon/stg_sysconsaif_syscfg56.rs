#[doc = "Register `stg_sysconsaif_syscfg56` reader"]
pub type R = crate::R<STG_SYSCONSAIF_SYSCFG56_SPEC>;
#[doc = "Register `stg_sysconsaif_syscfg56` writer"]
pub type W = crate::W<STG_SYSCONSAIF_SYSCFG56_SPEC>;
#[doc = "Field `u0_hifi4_pfaultinfovalid` reader - Fault Handling Signals"]
pub type U0_HIFI4_PFAULTINFOVALID_R = crate::BitReader;
#[doc = "Field `u0_hifi4_prid` reader - Module ID"]
pub type U0_HIFI4_PRID_R = crate::FieldReader<u16>;
#[doc = "Field `u0_hifi4_prid` writer - Module ID"]
pub type U0_HIFI4_PRID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `u0_hifi4_pwaitmode` reader - Wait Mode"]
pub type U0_HIFI4_PWAITMODE_R = crate::BitReader;
#[doc = "Field `u0_hifi4_runstall` reader - Run Stall"]
pub type U0_HIFI4_RUNSTALL_R = crate::BitReader;
#[doc = "Field `u0_hifi4_runstall` writer - Run Stall"]
pub type U0_HIFI4_RUNSTALL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    pub fn u0_hifi4_prid(&mut self) -> U0_HIFI4_PRID_W<STG_SYSCONSAIF_SYSCFG56_SPEC, 1> {
        U0_HIFI4_PRID_W::new(self)
    }
    #[doc = "Bit 18 - Run Stall"]
    #[inline(always)]
    #[must_use]
    pub fn u0_hifi4_runstall(&mut self) -> U0_HIFI4_RUNSTALL_W<STG_SYSCONSAIF_SYSCFG56_SPEC, 18> {
        U0_HIFI4_RUNSTALL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 56\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_sysconsaif_syscfg56::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_sysconsaif_syscfg56::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCONSAIF_SYSCFG56_SPEC;
impl crate::RegisterSpec for STG_SYSCONSAIF_SYSCFG56_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_sysconsaif_syscfg56::R`](R) reader structure"]
impl crate::Readable for STG_SYSCONSAIF_SYSCFG56_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_sysconsaif_syscfg56::W`](W) writer structure"]
impl crate::Writable for STG_SYSCONSAIF_SYSCFG56_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
