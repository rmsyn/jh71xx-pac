#[doc = "Register `sys_sysconsaif_syscfg32` reader"]
pub type R = crate::R<SYS_SYSCONSAIF_SYSCFG32_SPEC>;
#[doc = "Register `sys_sysconsaif_syscfg32` writer"]
pub type W = crate::W<SYS_SYSCONSAIF_SYSCFG32_SPEC>;
#[doc = "Field `u0_pll_wrap_pll0_frac` reader - u0_pll_wrap_pll0_frac"]
pub type U0_PLL_WRAP_PLL0_FRAC_R = crate::FieldReader<u32>;
#[doc = "Field `u0_pll_wrap_pll0_frac` writer - u0_pll_wrap_pll0_frac"]
pub type U0_PLL_WRAP_PLL0_FRAC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
#[doc = "Field `u0_pll_wrap_pll0_gvco_bias` reader - u0_pll_wrap_pll0_gvco_bias"]
pub type U0_PLL_WRAP_PLL0_GVCO_BIAS_R = crate::FieldReader;
#[doc = "Field `u0_pll_wrap_pll0_gvco_bias` writer - u0_pll_wrap_pll0_gvco_bias"]
pub type U0_PLL_WRAP_PLL0_GVCO_BIAS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `u0_pll_wrap_pll0_lock` reader - u0_pll_wrap_pll0_lock"]
pub type U0_PLL_WRAP_PLL0_LOCK_R = crate::BitReader;
#[doc = "Field `u0_pll_wrap_pll0_pd` reader - u0_pll_wrap_pll0_pd"]
pub type U0_PLL_WRAP_PLL0_PD_R = crate::BitReader;
#[doc = "Field `u0_pll_wrap_pll0_pd` writer - u0_pll_wrap_pll0_pd"]
pub type U0_PLL_WRAP_PLL0_PD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `u0_pll_wrap_pll0_postdiv1` reader - u0_pll_wrap_pll0_postdiv1"]
pub type U0_PLL_WRAP_PLL0_POSTDIV1_R = crate::FieldReader;
#[doc = "Field `u0_pll_wrap_pll0_postdiv1` writer - u0_pll_wrap_pll0_postdiv1"]
pub type U0_PLL_WRAP_PLL0_POSTDIV1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `u0_pll_wrap_pll0_postdiv2` reader - u0_pll_wrap_pll0_postdiv2"]
pub type U0_PLL_WRAP_PLL0_POSTDIV2_R = crate::FieldReader;
#[doc = "Field `u0_pll_wrap_pll0_postdiv2` writer - u0_pll_wrap_pll0_postdiv2"]
pub type U0_PLL_WRAP_PLL0_POSTDIV2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:23 - u0_pll_wrap_pll0_frac"]
    #[inline(always)]
    pub fn u0_pll_wrap_pll0_frac(&self) -> U0_PLL_WRAP_PLL0_FRAC_R {
        U0_PLL_WRAP_PLL0_FRAC_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:25 - u0_pll_wrap_pll0_gvco_bias"]
    #[inline(always)]
    pub fn u0_pll_wrap_pll0_gvco_bias(&self) -> U0_PLL_WRAP_PLL0_GVCO_BIAS_R {
        U0_PLL_WRAP_PLL0_GVCO_BIAS_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - u0_pll_wrap_pll0_lock"]
    #[inline(always)]
    pub fn u0_pll_wrap_pll0_lock(&self) -> U0_PLL_WRAP_PLL0_LOCK_R {
        U0_PLL_WRAP_PLL0_LOCK_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - u0_pll_wrap_pll0_pd"]
    #[inline(always)]
    pub fn u0_pll_wrap_pll0_pd(&self) -> U0_PLL_WRAP_PLL0_PD_R {
        U0_PLL_WRAP_PLL0_PD_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - u0_pll_wrap_pll0_postdiv1"]
    #[inline(always)]
    pub fn u0_pll_wrap_pll0_postdiv1(&self) -> U0_PLL_WRAP_PLL0_POSTDIV1_R {
        U0_PLL_WRAP_PLL0_POSTDIV1_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - u0_pll_wrap_pll0_postdiv2"]
    #[inline(always)]
    pub fn u0_pll_wrap_pll0_postdiv2(&self) -> U0_PLL_WRAP_PLL0_POSTDIV2_R {
        U0_PLL_WRAP_PLL0_POSTDIV2_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - u0_pll_wrap_pll0_frac"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pll_wrap_pll0_frac(
        &mut self,
    ) -> U0_PLL_WRAP_PLL0_FRAC_W<SYS_SYSCONSAIF_SYSCFG32_SPEC, 0> {
        U0_PLL_WRAP_PLL0_FRAC_W::new(self)
    }
    #[doc = "Bits 24:25 - u0_pll_wrap_pll0_gvco_bias"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pll_wrap_pll0_gvco_bias(
        &mut self,
    ) -> U0_PLL_WRAP_PLL0_GVCO_BIAS_W<SYS_SYSCONSAIF_SYSCFG32_SPEC, 24> {
        U0_PLL_WRAP_PLL0_GVCO_BIAS_W::new(self)
    }
    #[doc = "Bit 27 - u0_pll_wrap_pll0_pd"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pll_wrap_pll0_pd(
        &mut self,
    ) -> U0_PLL_WRAP_PLL0_PD_W<SYS_SYSCONSAIF_SYSCFG32_SPEC, 27> {
        U0_PLL_WRAP_PLL0_PD_W::new(self)
    }
    #[doc = "Bits 28:29 - u0_pll_wrap_pll0_postdiv1"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pll_wrap_pll0_postdiv1(
        &mut self,
    ) -> U0_PLL_WRAP_PLL0_POSTDIV1_W<SYS_SYSCONSAIF_SYSCFG32_SPEC, 28> {
        U0_PLL_WRAP_PLL0_POSTDIV1_W::new(self)
    }
    #[doc = "Bits 30:31 - u0_pll_wrap_pll0_postdiv2"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pll_wrap_pll0_postdiv2(
        &mut self,
    ) -> U0_PLL_WRAP_PLL0_POSTDIV2_W<SYS_SYSCONSAIF_SYSCFG32_SPEC, 30> {
        U0_PLL_WRAP_PLL0_POSTDIV2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYS SYSCONSAIF SYSCFG 32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg32::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg32::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_SYSCONSAIF_SYSCFG32_SPEC;
impl crate::RegisterSpec for SYS_SYSCONSAIF_SYSCFG32_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_sysconsaif_syscfg32::R`](R) reader structure"]
impl crate::Readable for SYS_SYSCONSAIF_SYSCFG32_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_sysconsaif_syscfg32::W`](W) writer structure"]
impl crate::Writable for SYS_SYSCONSAIF_SYSCFG32_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
