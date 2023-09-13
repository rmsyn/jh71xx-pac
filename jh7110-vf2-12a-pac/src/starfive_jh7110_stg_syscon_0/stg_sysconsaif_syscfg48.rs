#[doc = "Register `stg_sysconsaif_syscfg48` reader"]
pub type R = crate::R<STG_SYSCONSAIF_SYSCFG48_SPEC>;
#[doc = "Register `stg_sysconsaif_syscfg48` writer"]
pub type W = crate::W<STG_SYSCONSAIF_SYSCFG48_SPEC>;
#[doc = "Field `u0_hifi4_breakin` reader - Debug signal"]
pub type U0_HIFI4_BREAKIN_R = crate::BitReader;
#[doc = "Field `u0_hifi4_breakin` writer - Debug signal"]
pub type U0_HIFI4_BREAKIN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `u0_hifi4_breakinack` reader - Debug signal"]
pub type U0_HIFI4_BREAKINACK_R = crate::BitReader;
#[doc = "Field `u0_hifi4_breakout` reader - Debug signal"]
pub type U0_HIFI4_BREAKOUT_R = crate::BitReader;
#[doc = "Field `u0_hifi4_breakoutack` reader - Debug signal"]
pub type U0_HIFI4_BREAKOUTACK_R = crate::BitReader;
#[doc = "Field `u0_hifi4_breakoutack` writer - Debug signal"]
pub type U0_HIFI4_BREAKOUTACK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `u0_hifi4_debugmode` reader - Debug signal"]
pub type U0_HIFI4_DEBUGMODE_R = crate::BitReader;
#[doc = "Field `u0_hifi4_doubleexceptionerror` reader - Fault Handling Signals"]
pub type U0_HIFI4_DOUBLEEXCEPTIONERROR_R = crate::BitReader;
#[doc = "Field `u0_hifi4_iram0loadstore` reader - Indicates that iram0 works"]
pub type U0_HIFI4_IRAM0LOADSTORE_R = crate::BitReader;
#[doc = "Field `u0_hifi4_iram1loadstore` reader - Indicates that iram1 works"]
pub type U0_HIFI4_IRAM1LOADSTORE_R = crate::BitReader;
#[doc = "Field `u0_hifi4_ocdhaltonreset` reader - Debug signal"]
pub type U0_HIFI4_OCDHALTONRESET_R = crate::BitReader;
#[doc = "Field `u0_hifi4_ocdhaltonreset` writer - Debug signal"]
pub type U0_HIFI4_OCDHALTONRESET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `u0_hifi4_pfatalerror` reader - Fault Handling Signals"]
pub type U0_HIFI4_PFATALERROR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Debug signal"]
    #[inline(always)]
    pub fn u0_hifi4_breakin(&self) -> U0_HIFI4_BREAKIN_R {
        U0_HIFI4_BREAKIN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Debug signal"]
    #[inline(always)]
    pub fn u0_hifi4_breakinack(&self) -> U0_HIFI4_BREAKINACK_R {
        U0_HIFI4_BREAKINACK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Debug signal"]
    #[inline(always)]
    pub fn u0_hifi4_breakout(&self) -> U0_HIFI4_BREAKOUT_R {
        U0_HIFI4_BREAKOUT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Debug signal"]
    #[inline(always)]
    pub fn u0_hifi4_breakoutack(&self) -> U0_HIFI4_BREAKOUTACK_R {
        U0_HIFI4_BREAKOUTACK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Debug signal"]
    #[inline(always)]
    pub fn u0_hifi4_debugmode(&self) -> U0_HIFI4_DEBUGMODE_R {
        U0_HIFI4_DEBUGMODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Fault Handling Signals"]
    #[inline(always)]
    pub fn u0_hifi4_doubleexceptionerror(&self) -> U0_HIFI4_DOUBLEEXCEPTIONERROR_R {
        U0_HIFI4_DOUBLEEXCEPTIONERROR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Indicates that iram0 works"]
    #[inline(always)]
    pub fn u0_hifi4_iram0loadstore(&self) -> U0_HIFI4_IRAM0LOADSTORE_R {
        U0_HIFI4_IRAM0LOADSTORE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Indicates that iram1 works"]
    #[inline(always)]
    pub fn u0_hifi4_iram1loadstore(&self) -> U0_HIFI4_IRAM1LOADSTORE_R {
        U0_HIFI4_IRAM1LOADSTORE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Debug signal"]
    #[inline(always)]
    pub fn u0_hifi4_ocdhaltonreset(&self) -> U0_HIFI4_OCDHALTONRESET_R {
        U0_HIFI4_OCDHALTONRESET_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Fault Handling Signals"]
    #[inline(always)]
    pub fn u0_hifi4_pfatalerror(&self) -> U0_HIFI4_PFATALERROR_R {
        U0_HIFI4_PFATALERROR_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Debug signal"]
    #[inline(always)]
    #[must_use]
    pub fn u0_hifi4_breakin(&mut self) -> U0_HIFI4_BREAKIN_W<STG_SYSCONSAIF_SYSCFG48_SPEC, 0> {
        U0_HIFI4_BREAKIN_W::new(self)
    }
    #[doc = "Bit 3 - Debug signal"]
    #[inline(always)]
    #[must_use]
    pub fn u0_hifi4_breakoutack(
        &mut self,
    ) -> U0_HIFI4_BREAKOUTACK_W<STG_SYSCONSAIF_SYSCFG48_SPEC, 3> {
        U0_HIFI4_BREAKOUTACK_W::new(self)
    }
    #[doc = "Bit 8 - Debug signal"]
    #[inline(always)]
    #[must_use]
    pub fn u0_hifi4_ocdhaltonreset(
        &mut self,
    ) -> U0_HIFI4_OCDHALTONRESET_W<STG_SYSCONSAIF_SYSCFG48_SPEC, 8> {
        U0_HIFI4_OCDHALTONRESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 48\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_sysconsaif_syscfg48::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_sysconsaif_syscfg48::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCONSAIF_SYSCFG48_SPEC;
impl crate::RegisterSpec for STG_SYSCONSAIF_SYSCFG48_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_sysconsaif_syscfg48::R`](R) reader structure"]
impl crate::Readable for STG_SYSCONSAIF_SYSCFG48_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_sysconsaif_syscfg48::W`](W) writer structure"]
impl crate::Writable for STG_SYSCONSAIF_SYSCFG48_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
