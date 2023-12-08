#[doc = "Register `sys_syscfg_12` reader"]
pub type R = crate::R<SYS_SYSCFG_12_SPEC>;
#[doc = "Register `sys_syscfg_12` writer"]
pub type W = crate::W<SYS_SYSCFG_12_SPEC>;
#[doc = "Field `pll2_frac` reader - pll2_frac"]
pub type PLL2_FRAC_R = crate::FieldReader<u32>;
#[doc = "Field `pll2_frac` writer - pll2_frac"]
pub type PLL2_FRAC_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `pll2_gvco_bias` reader - pll2_gvco_bias"]
pub type PLL2_GVCO_BIAS_R = crate::FieldReader;
#[doc = "Field `pll2_gvco_bias` writer - pll2_gvco_bias"]
pub type PLL2_GVCO_BIAS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pll2_lock` reader - pll2_lock"]
pub type PLL2_LOCK_R = crate::BitReader;
#[doc = "Field `pll2_pd` reader - pll2_pd"]
pub type PLL2_PD_R = crate::BitReader;
#[doc = "Field `pll2_pd` writer - pll2_pd"]
pub type PLL2_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pll2_postdiv1` reader - pll2_postdiv1"]
pub type PLL2_POSTDIV1_R = crate::FieldReader;
#[doc = "Field `pll2_postdiv1` writer - pll2_postdiv1"]
pub type PLL2_POSTDIV1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pll2_postdiv2` reader - pll2_postdiv2"]
pub type PLL2_POSTDIV2_R = crate::FieldReader;
#[doc = "Field `pll2_postdiv2` writer - pll2_postdiv2"]
pub type PLL2_POSTDIV2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:23 - pll2_frac"]
    #[inline(always)]
    pub fn pll2_frac(&self) -> PLL2_FRAC_R {
        PLL2_FRAC_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:25 - pll2_gvco_bias"]
    #[inline(always)]
    pub fn pll2_gvco_bias(&self) -> PLL2_GVCO_BIAS_R {
        PLL2_GVCO_BIAS_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - pll2_lock"]
    #[inline(always)]
    pub fn pll2_lock(&self) -> PLL2_LOCK_R {
        PLL2_LOCK_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - pll2_pd"]
    #[inline(always)]
    pub fn pll2_pd(&self) -> PLL2_PD_R {
        PLL2_PD_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - pll2_postdiv1"]
    #[inline(always)]
    pub fn pll2_postdiv1(&self) -> PLL2_POSTDIV1_R {
        PLL2_POSTDIV1_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - pll2_postdiv2"]
    #[inline(always)]
    pub fn pll2_postdiv2(&self) -> PLL2_POSTDIV2_R {
        PLL2_POSTDIV2_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - pll2_frac"]
    #[inline(always)]
    #[must_use]
    pub fn pll2_frac(&mut self) -> PLL2_FRAC_W<SYS_SYSCFG_12_SPEC> {
        PLL2_FRAC_W::new(self, 0)
    }
    #[doc = "Bits 24:25 - pll2_gvco_bias"]
    #[inline(always)]
    #[must_use]
    pub fn pll2_gvco_bias(&mut self) -> PLL2_GVCO_BIAS_W<SYS_SYSCFG_12_SPEC> {
        PLL2_GVCO_BIAS_W::new(self, 24)
    }
    #[doc = "Bit 27 - pll2_pd"]
    #[inline(always)]
    #[must_use]
    pub fn pll2_pd(&mut self) -> PLL2_PD_W<SYS_SYSCFG_12_SPEC> {
        PLL2_PD_W::new(self, 27)
    }
    #[doc = "Bits 28:29 - pll2_postdiv1"]
    #[inline(always)]
    #[must_use]
    pub fn pll2_postdiv1(&mut self) -> PLL2_POSTDIV1_W<SYS_SYSCFG_12_SPEC> {
        PLL2_POSTDIV1_W::new(self, 28)
    }
    #[doc = "Bits 30:31 - pll2_postdiv2"]
    #[inline(always)]
    #[must_use]
    pub fn pll2_postdiv2(&mut self) -> PLL2_POSTDIV2_W<SYS_SYSCFG_12_SPEC> {
        PLL2_POSTDIV2_W::new(self, 30)
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
#[doc = "SYS SYSCONSAIF SYSCFG 48\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_SYSCFG_12_SPEC;
impl crate::RegisterSpec for SYS_SYSCFG_12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg_12::R`](R) reader structure"]
impl crate::Readable for SYS_SYSCFG_12_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_syscfg_12::W`](W) writer structure"]
impl crate::Writable for SYS_SYSCFG_12_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sys_syscfg_12 to value 0"]
impl crate::Resettable for SYS_SYSCFG_12_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
