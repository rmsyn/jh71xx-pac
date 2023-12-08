#[doc = "Register `sys_syscfg_8` reader"]
pub type R = crate::R<SYS_SYSCFG_8_SPEC>;
#[doc = "Register `sys_syscfg_8` writer"]
pub type W = crate::W<SYS_SYSCFG_8_SPEC>;
#[doc = "Field `pll0_frac` reader - pll0_frac"]
pub type PLL0_FRAC_R = crate::FieldReader<u32>;
#[doc = "Field `pll0_frac` writer - pll0_frac"]
pub type PLL0_FRAC_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `pll0_gvco_bias` reader - pll0_gvco_bias"]
pub type PLL0_GVCO_BIAS_R = crate::FieldReader;
#[doc = "Field `pll0_gvco_bias` writer - pll0_gvco_bias"]
pub type PLL0_GVCO_BIAS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pll0_lock` reader - pll0_lock"]
pub type PLL0_LOCK_R = crate::BitReader;
#[doc = "Field `pll0_pd` reader - pll0_pd"]
pub type PLL0_PD_R = crate::BitReader;
#[doc = "Field `pll0_pd` writer - pll0_pd"]
pub type PLL0_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pll0_postdiv1` reader - pll0_postdiv1"]
pub type PLL0_POSTDIV1_R = crate::FieldReader;
#[doc = "Field `pll0_postdiv1` writer - pll0_postdiv1"]
pub type PLL0_POSTDIV1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pll0_postdiv2` reader - pll0_postdiv2"]
pub type PLL0_POSTDIV2_R = crate::FieldReader;
#[doc = "Field `pll0_postdiv2` writer - pll0_postdiv2"]
pub type PLL0_POSTDIV2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:23 - pll0_frac"]
    #[inline(always)]
    pub fn pll0_frac(&self) -> PLL0_FRAC_R {
        PLL0_FRAC_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:25 - pll0_gvco_bias"]
    #[inline(always)]
    pub fn pll0_gvco_bias(&self) -> PLL0_GVCO_BIAS_R {
        PLL0_GVCO_BIAS_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - pll0_lock"]
    #[inline(always)]
    pub fn pll0_lock(&self) -> PLL0_LOCK_R {
        PLL0_LOCK_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - pll0_pd"]
    #[inline(always)]
    pub fn pll0_pd(&self) -> PLL0_PD_R {
        PLL0_PD_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - pll0_postdiv1"]
    #[inline(always)]
    pub fn pll0_postdiv1(&self) -> PLL0_POSTDIV1_R {
        PLL0_POSTDIV1_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - pll0_postdiv2"]
    #[inline(always)]
    pub fn pll0_postdiv2(&self) -> PLL0_POSTDIV2_R {
        PLL0_POSTDIV2_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - pll0_frac"]
    #[inline(always)]
    #[must_use]
    pub fn pll0_frac(&mut self) -> PLL0_FRAC_W<SYS_SYSCFG_8_SPEC> {
        PLL0_FRAC_W::new(self, 0)
    }
    #[doc = "Bits 24:25 - pll0_gvco_bias"]
    #[inline(always)]
    #[must_use]
    pub fn pll0_gvco_bias(&mut self) -> PLL0_GVCO_BIAS_W<SYS_SYSCFG_8_SPEC> {
        PLL0_GVCO_BIAS_W::new(self, 24)
    }
    #[doc = "Bit 27 - pll0_pd"]
    #[inline(always)]
    #[must_use]
    pub fn pll0_pd(&mut self) -> PLL0_PD_W<SYS_SYSCFG_8_SPEC> {
        PLL0_PD_W::new(self, 27)
    }
    #[doc = "Bits 28:29 - pll0_postdiv1"]
    #[inline(always)]
    #[must_use]
    pub fn pll0_postdiv1(&mut self) -> PLL0_POSTDIV1_W<SYS_SYSCFG_8_SPEC> {
        PLL0_POSTDIV1_W::new(self, 28)
    }
    #[doc = "Bits 30:31 - pll0_postdiv2"]
    #[inline(always)]
    #[must_use]
    pub fn pll0_postdiv2(&mut self) -> PLL0_POSTDIV2_W<SYS_SYSCFG_8_SPEC> {
        PLL0_POSTDIV2_W::new(self, 30)
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
#[doc = "SYS SYSCONSAIF SYSCFG 32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_SYSCFG_8_SPEC;
impl crate::RegisterSpec for SYS_SYSCFG_8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg_8::R`](R) reader structure"]
impl crate::Readable for SYS_SYSCFG_8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_syscfg_8::W`](W) writer structure"]
impl crate::Writable for SYS_SYSCFG_8_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sys_syscfg_8 to value 0"]
impl crate::Resettable for SYS_SYSCFG_8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
